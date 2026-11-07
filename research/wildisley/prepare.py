"""Prepare the pinned Wildisley Filho dataset without executing upstream code."""
import argparse
import collections
import csv
import hashlib
import json
from pathlib import Path
import tarfile

ROOT = Path(__file__).resolve().parents[2]
PROTOCOL = ROOT / "docs/WILDISLEY_PROTOCOL_V1.json"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def lower_qasm(path, protocol):
    from qiskit import QuantumCircuit, qasm2, transpile
    circuit = qasm2.load(path, custom_instructions=qasm2.LEGACY_CUSTOM_INSTRUCTIONS)
    unitary = QuantumCircuit(circuit.num_qubits)
    measured = False
    mapping = []
    for item in circuit.data:
        name = item.operation.name
        if name == "barrier":
            continue
        if name == "measure":
            measured = True
            mapping.append([circuit.find_bit(item.qubits[0]).index, circuit.find_bit(item.clbits[0]).index])
            continue
        if measured:
            raise ValueError("mid_circuit_measurement")
        if item.clbits or name in {"reset", "if_else", "while_loop", "for_loop", "switch_case", "initialize"}:
            raise ValueError("nonunitary_or_classical_control:" + name)
        unitary.append(item.operation, [circuit.find_bit(q).index for q in item.qubits])
    lowered = transpile(unitary, basis_gates=protocol["lowering"]["basis"],
                        optimization_level=0, seed_transpiler=protocol["lowering"]["seed_transpiler"])
    gates = []
    for item in lowered.data:
        if item.operation.name == "barrier":
            continue
        if item.operation.name not in protocol["lowering"]["basis"] or item.clbits:
            raise ValueError("unsupported_lowered_operation:" + item.operation.name)
        gates.append({"name": item.operation.name,
                      "qubits": [lowered.find_bit(q).index for q in item.qubits],
                      "angle": float(item.operation.params[0]) if item.operation.params else 0.0})
    if len(gates) > protocol["max_lowered_gates"]:
        raise ValueError("lowered_gate_budget")
    return {"width": lowered.num_qubits, "gates": gates}, unitary, {"terminal_measurements": mapping, "source_classical_bits": circuit.num_clbits, "lowered_global_phase": float(lowered.global_phase)}


def source_root(archive, cache, expected):
    if sha(archive) != expected:
        raise ValueError("source archive SHA-256 mismatch")
    cache.mkdir(parents=True, exist_ok=True)
    with tarfile.open(archive) as handle:
        roots = {Path(m.name).parts[0] for m in handle.getmembers()}
        if len(roots) != 1:
            raise ValueError("unexpected archive roots")
        # Python's data filter rejects traversal, external links and device files.
        handle.extractall(cache, filter="data")
    return cache / roots.pop()


def prepare(archive, cache, output):
    import qiskit
    import numpy as np
    from qiskit.quantum_info import Statevector
    protocol = json.loads(PROTOCOL.read_text())
    source = source_root(archive, cache, protocol["source"]["archive_sha256"])
    meta_path = source / protocol["source"]["dataset"] / "metadata.csv"
    rows = list(csv.DictReader(meta_path.open()))
    assert collections.Counter(r["label"] for r in rows) == {"0": 535, "1": 5350}
    assert len({r["sample_id"] for r in rows}) == len(rows)
    groups = collections.defaultdict(list)
    for row in rows:
        groups[row["base_circuit_id"]].append(row)
    assert len(groups) == 535 and all(len(v) == 11 and sum(r["label"] == "0" for r in v) == 1 for v in groups.values())
    predictions_path = source / "results/detector_analysis/cv_predictions_by_sample.csv"
    labels = {r["sample_id"]: int(r["label"]) for r in rows}
    predictions = collections.defaultdict(dict)
    folds = {}
    for row in csv.DictReader(predictions_path.open()):
        if row["cv_strategy"] != "GroupKFold":
            continue
        sid, fold = row["sample_id"], int(row["fold"])
        assert int(row["y_true"]) == labels[sid]
        key = row["pipeline"]
        assert sid not in predictions[key]
        predictions[key][sid] = {"prediction": int(row["y_pred"]), "score": float(row["y_score"])}
        if sid in folds:
            assert folds[sid] == fold
        folds[sid] = fold
    assert len(predictions) == 8 and all(set(v) == set(labels) for v in predictions.values())
    for group in groups.values():
        assert len({folds[r["sample_id"]] for r in group}) == 1
    inventory, prepared, excluded = [], [], []
    max_parity_error = 0.0
    for group_id in sorted(groups, key=int):
        members = sorted(groups[group_id], key=lambda r: (int(r["label"]), r["sample_id"]))
        converted, reasons = [], []
        for row in members:
            relative = row["infected_path"] if row["label"] == "1" else row["original_path"]
            path = source / relative
            if not path.is_file():
                raise FileNotFoundError(path)
            info = {"sample_id": row["sample_id"], "base_circuit_id": group_id, "fold": folds[row["sample_id"]],
                    "label": int(row["label"]), "source_path": relative, "sha256": sha(path),
                    "width": int(row["num_qubits"]), "gate": row["gate"], "mode": row["mode"],
                    "family": row["source_family"], "source_dataset": row["source_dataset"]}
            inventory.append(info)
            if info["width"] > protocol["max_qubits"]:
                reasons.append("width_exceeds_native_density_budget")
                continue
            try:
                circuit, original, conversion = lower_qasm(path, protocol)
                # Compare pre-lowering semantics with a fresh circuit built from
                # serialized RASQP gates, not merely with transpile's own output.
                from qiskit import QuantumCircuit
                rebuilt = QuantumCircuit(circuit["width"])
                for g in circuit["gates"]:
                    method = getattr(rebuilt, g["name"])
                    if g["name"].startswith("r"):
                        method(g["angle"], *g["qubits"])
                    else:
                        method(*g["qubits"])
                # State overlap detects relative-phase errors that output
                # probabilities alone can hide. Global phase is immaterial.
                state, restored = Statevector(original).data, Statevector(rebuilt).data
                overlap = np.vdot(state, restored)
                phase = overlap / abs(overlap) if abs(overlap) else 1.0
                error = float(np.max(np.abs(restored - phase * state)))
                if error > 1e-10:
                    raise ValueError("lowering_state_parity")
                max_parity_error = max(max_parity_error, error)
                converted.append({**info, "circuit": circuit, "conversion": conversion, "lowering_state_max_error": error})
            except Exception as exc:
                reasons.append(type(exc).__name__ + ":" + str(exc)[:250])
        if reasons:
            excluded.append({"base_circuit_id": group_id, "fold": folds[members[0]["sample_id"]], "sample_count": len(members), "reasons": dict(collections.Counter(reasons))})
        else:
            prepared.append({"base_circuit_id": group_id, "fold": folds[members[0]["sample_id"]], "samples": converted})
        if len(prepared) % 25 == 0 and converted:
            print(f"prepared {len(prepared)} complete groups; examined base {group_id}", flush=True)
    output.mkdir(parents=True, exist_ok=True)
    input_path = output / "input.jsonl"
    with input_path.open("w") as h:
        for group in prepared:
            h.write(json.dumps(group, separators=(",", ":")) + "\n")
    with (output / "inventory.jsonl").open("w") as h:
        for row in inventory:
            h.write(json.dumps(row, separators=(",", ":")) + "\n")
    included = {s["sample_id"] for g in prepared for s in g["samples"]}
    evidence = {"version": protocol["version"], "source": protocol["source"],
                "protocol_sha256": sha(PROTOCOL), "metadata_sha256": sha(meta_path),
                "published_predictions_sha256": sha(predictions_path), "input_sha256": sha(input_path),
                "inventory_sha256": sha(output / "inventory.jsonl"), "qiskit": qiskit.__version__,
                "numpy": np.__version__, "source_rows": len(rows), "source_bases": len(groups),
                "eligible_bases": len(prepared), "eligible_rows": len(included), "excluded_groups": excluded,
                "conversion_state_max_error": max_parity_error,
                "predictions": {model: {sid: {**v, "label": labels[sid], "fold": folds[sid], "eligible": sid in included} for sid, v in values.items()} for model, values in predictions.items()}}
    (output / "preparation.json").write_text(json.dumps(evidence, indent=2) + "\n")
    print(json.dumps({k: evidence[k] for k in ("source_rows", "source_bases", "eligible_bases", "eligible_rows", "conversion_state_max_error")}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--archive", type=Path, required=True)
    parser.add_argument("--cache", type=Path, default=ROOT / "results/runs/wildisley-source")
    parser.add_argument("--output", type=Path, default=ROOT / "results/runs/wildisley-input")
    args = parser.parse_args()
    prepare(args.archive, args.cache, args.output)
