"""Independent, standard-library-only audit of frozen native observations.

Does not import the SDK or change its protocols/results. Run from any directory:
  python3 audit/verify_evidence.py --fresh /tmp/rasqp-adversarial-audit-native
"""
import argparse
import collections
import cmath
import hashlib
import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def distance(counts, reference):
    assert len(counts) == len(reference) and sum(counts) > 0
    return sum(abs(c / sum(counts) - p) for c, p in zip(counts, reference)) / 2


def threshold(scores, alpha):
    rank = math.ceil((len(scores) + 1) * (1 - alpha))
    return sorted(scores)[rank - 1] if rank <= len(scores) else 1.0


def wilson(k, n):
    z = 1.959963984540054
    p = k / n
    den = 1 + z * z / n
    center = (p + z * z / (2 * n)) / den
    radius = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / den
    return [center - radius, center + radius]


def check_rate(record, flags):
    k, n = sum(flags), len(flags)
    assert record["n"] == n and record["rejected"] == k
    assert abs(record["rate"] - k / n) < 1e-14
    assert max(abs(a - b) for a, b in zip(record["wilson95"], wilson(k, n))) < 1e-12


def ideal(circuit):
    """Small independent gate calculation for the archived probe references."""
    state = [0j] * (1 << circuit["width"])
    state[0] = 1 + 0j
    for gate in circuit["gates"]:
        name, qubits, angle = gate["name"], gate["qubits"], gate["angle"]
        if name == "cx":
            control, target = (1 << q for q in qubits)
            for i in range(len(state)):
                if i & control and not i & target:
                    state[i], state[i | target] = state[i | target], state[i]
            continue
        cosine, sine = math.cos(angle / 2), math.sin(angle / 2)
        matrices = {
            "x": (0, 1, 1, 0), "z": (1, 0, 0, -1),
            "h": tuple(v / math.sqrt(2) for v in (1, 1, 1, -1)),
            "rx": (cosine, -1j * sine, -1j * sine, cosine),
            "ry": (cosine, -sine, sine, cosine),
            "rz": (cmath.exp(-0.5j * angle), 0, 0, cmath.exp(0.5j * angle)),
        }
        a, b, c, d = matrices[name]
        bit = 1 << qubits[0]
        for i in range(len(state)):
            if not i & bit:
                left, right = state[i], state[i | bit]
                state[i], state[i | bit] = a * left + b * right, c * left + d * right
    return [abs(v) ** 2 for v in state]


def close(left, right):
    assert len(left) == len(right)
    assert max(abs(a - b) for a, b in zip(left, right)) < 1e-12


def comparisons(records, flags, key):
    ps = []
    for record in records:
        baseline, candidate = key(record, flags)
        gains = sum(not a and b for a, b in zip(baseline, candidate))
        losses = sum(a and not b for a, b in zip(baseline, candidate))
        n = gains + losses
        p = sum(math.comb(n, i) for i in range(gains, n + 1)) / 2**n
        assert record["gains"] == gains and record["losses"] == losses
        assert record["n"] == len(candidate)
        assert abs(record["p_one_sided"] - p) <= max(1e-300, 1e-9 * p)
        assert abs(record["delta"] - (gains - losses) / len(candidate)) < 1e-14
        ps.append(p)
    running = 0
    for index, original in enumerate(sorted(range(len(ps)), key=ps.__getitem__)):
        running = max(running, min(1, (len(ps) - index) * ps[original]))
        assert abs(records[original]["p_holm"] - running) <= max(1e-300, 1e-9 * running)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--fresh", type=Path)
    parser.add_argument("--output", type=Path, default=ROOT / "audit/numeric_evidence.json")
    args = parser.parse_args()
    directory = ROOT / "results/native-v1"
    raw_path = directory / "native_trials.jsonl"
    rows = [json.loads(line) for line in raw_path.open()]
    summary = json.loads((directory / "native_summary.json").read_text())
    samples = [r for r in rows if r["kind"] == "samples"]
    for row in samples:
        assert all(len(c) == 1 << row["circuit"]["width"] and sum(c) == row["shots"] for c in row["counts"])
    dynamic = [r for r in rows if r["kind"] == "dynamic_check"]
    assert len(dynamic) == 1
    executor_shots = sum(len(r["counts"]) * r["shots"] for r in samples) + sum(map(sum, dynamic[0]["counts"]))
    assert executor_shots == summary["cost"]["shots"]
    assert sum(len(r["counts"]) for r in samples) + len(dynamic[0]["counts"]) == summary["cost"]["circuits"]
    assert len(samples) + len(dynamic) == summary["cost"]["jobs"]
    fabricated = [r for r in rows if r["kind"] == "batch_decision" and r["attack"] == "forged_uniform"]
    ext = summary["studies"]["extensions"]
    cases = ["clean", "x", "z", "cx", "xz", "cxz", "weak_rotation", "benign_noise_drift", "public_probe_adaptive_fabrication", "equivalent_x_pair"]
    ext_rows = iter(r for r in rows if r["kind"] == "extension_decisions" or r.get("label", "").startswith("extension/"))
    all_ext_flags = {}
    checked = 0
    for condition in ext["conditions"]:
        noise = condition["noise"]
        for name, count in [("fixed_0Z", 1), ("trained_single", 1), ("portfolio_joint", 3)]:
            calibration = [next(ext_rows) for _ in range(count)]
            assert all(r["label"] == "extension/cal" for r in calibration)
            refs = [ideal(r["circuit"]) for r in calibration]
            scores = [max(distance(c["counts"][i], ref) for c, ref in zip(calibration, refs)) for i in range(127)]
            cut = threshold(scores, .05)
            individual = [threshold([distance(c, ref) for c in row["counts"]], .05) for row, ref in zip(calibration, refs)]
            assert abs(cut - condition["methods"][name]["threshold"]) < 1e-12
            for case in cases:
                observations = [next(ext_rows) for _ in range(count)]
                assert all(r["label"] == f"extension/{name}/{case}" for r in observations)
                record = next(ext_rows)
                assert record["kind"] == "extension_decisions" and record["noise"] == noise and record["method"] == name and record["case"] == case
                for actual, expected in zip(record["references"], refs):
                    close(actual, expected)
                flags = [max(distance(row["counts"][i], ref) for row, ref in zip(observations, refs)) > cut + 1e-12 for i in range(128)]
                assert flags == record["decisions"]
                check_rate(condition["methods"][name]["cases"][case], flags)
                all_ext_flags[noise, name, case] = flags
                checked += len(flags)
                if name == "portfolio_joint":
                    naive = [any(distance(row["counts"][i], ref) > t + 1e-12 for row, ref, t in zip(observations, refs, individual)) for i in range(128)]
                    check_rate(condition["methods"]["portfolio_naive_or"]["cases"][case], naive)
                    all_ext_flags[noise, "portfolio_naive_or", case] = naive
    assert next(ext_rows, None) is None
    strong = ("x", "z", "cx", "xz", "cxz")
    for condition in ext["conditions"]:
        methods = condition["methods"]
        joint = methods["portfolio_joint"]["cases"]
        minimum = min(joint[k]["rate"] for k in strong)
        gains = [minimum - min(methods[m]["cases"][k]["rate"] for k in strong) for m in ("fixed_0Z", "trained_single")]
        h41 = (all(joint[k]["wilson95"][1] <= .10 for k in ("clean", "equivalent_x_pair"))
               and all(joint[k]["wilson95"][0] >= .90 for k in strong)
               and all(g >= .20 for g in gains))
        h42 = (joint["clean"]["wilson95"][1] <= .10
               and all(joint[k]["rate"] + .05 >= methods["portfolio_naive_or"]["cases"][k]["rate"] for k in strong))
        assert condition["H41_pass"] == h41 and condition["H42_pass"] == h42
    assert ext["H41_pass"] == all(c["H41_pass"] for c in ext["conditions"])
    assert ext["H42_pass"] == all(c["H42_pass"] for c in ext["conditions"])
    def extension_pair(record, flags):
        c = record["condition"]
        return flags[c["noise"], c["baseline"], c["case"]], flags[c["noise"], "portfolio_joint", c["case"]]
    comparisons(ext["comparisons"], all_ext_flags, extension_pair)
    context = summary["studies"]["context"]
    context_rows = iter(r for r in rows if r["kind"] == "context_decisions" or r.get("label", "").startswith("context/"))
    all_context_flags = {}
    for condition in context["conditions"]:
        noise, padded = condition["noise"], condition["padded"]
        learned, exact, calibration = [], [], []
        for _ in range(3):
            reference, cal = next(context_rows), next(context_rows)
            assert reference["label"] == "context/reference" and cal["label"] == "context/cal"
            learned.append([c / sum(reference["counts"][0]) for c in reference["counts"][0]])
            exact.append(ideal(reference["circuit"]))
            calibration.append(cal)
        thresholds = {}
        for name, method in condition["methods"].items():
            model, alpha = name.rsplit("_", 1)
            refs = learned if model == "trusted_noisy" else exact
            for actual, expected in zip(method["reference"], refs):
                close(actual, expected)
            scores = [max(distance(row["counts"][i], ref) for row, ref in zip(calibration, refs)) for i in range(511)]
            thresholds[name] = threshold(scores, float(alpha))
            assert abs(method["threshold"] - thresholds[name]) < 1e-12
        for case in cases[:-1]:
            observations = [next(context_rows) for _ in range(3)]
            assert all(r["label"] == f"context/{case}" for r in observations)
            for name in sorted(condition["methods"]):
                record = next(context_rows)
                assert record["kind"] == "context_decisions" and record["noise"] == noise and record["padded"] == padded and record["case"] == case and record["method"] == name
                refs = learned if name.startswith("trusted_noisy") else exact
                flags = [max(distance(row["counts"][i], ref) for row, ref in zip(observations, refs)) > thresholds[name] + 1e-12 for i in range(256)]
                assert flags == record["decisions"]
                check_rate(condition["methods"][name]["cases"][case], flags)
                all_context_flags[noise, padded, name, case] = flags
                checked += len(flags)
    assert next(context_rows, None) is None
    for condition in context["conditions"]:
        candidate = condition["methods"]["trusted_noisy_0.01"]["cases"]
        passed = candidate["clean"]["wilson95"][1] <= .10 and all(candidate[k]["wilson95"][0] >= .90 for k in strong)
        assert passed == condition["H44_pass"]
    assert context["H44_pass"] == all(c["H44_pass"] for c in context["conditions"])
    def context_pair(record, flags):
        c = record["condition"]
        return flags[c["noise"], c["padded"], c["baseline"], c["case"]], flags[c["noise"], c["padded"], "trusted_noisy_0.01", c["case"]]
    comparisons(context["comparisons"], all_context_flags, context_pair)
    evidence = {
        "scope": "Independent recomputation of extension/context references, quantiles, decisions, rates, Wilson intervals, McNemar and Holm; no tuning of frozen experiments.",
        "raw_sha256": hashlib.sha256(raw_path.read_bytes()).hexdigest(),
        "raw_kinds": dict(collections.Counter(r["kind"] for r in rows)),
        "decisions_recomputed": checked,
        "comparisons_recomputed": len(ext["comparisons"]) + len(context["comparisons"]),
        "executor_cost_reconciles": True,
        "executor_cost": summary["cost"],
        "classical_fabrication_histograms": len(fabricated),
        "additional_classical_uniform_draws": sum(r["shots"] for r in fabricated),
        "executor_plus_fabrication_draws": executor_shots + sum(r["shots"] for r in fabricated),
        "sample_rows_without_noise_metadata": sum("noise" not in r for r in samples),
        "H41_pass": ext["H41_pass"], "H42_pass": ext["H42_pass"], "H44_pass": context["H44_pass"],
        "weak_rz_ideal_detection_probability_at_256_sensitive_shots": 1 - (1 - math.sin(.12 / 2)**2)**256,
    }
    if args.fresh:
        evidence["fresh_raw_byte_identical"] = (args.fresh / "native_trials.jsonl").read_bytes() == raw_path.read_bytes()
        assert evidence["fresh_raw_byte_identical"]
        def untimed(value):
            if isinstance(value, dict):
                return {k: untimed(v) for k, v in value.items() if k != "seconds"}
            if isinstance(value, list):
                return [untimed(v) for v in value]
            return value
        assert untimed(summary) == untimed(json.loads((args.fresh / "native_summary.json").read_text()))
        evidence["fresh_summary_identical_except_timings"] = True
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(evidence, indent=2) + "\n")
    print(json.dumps(evidence, indent=2))


if __name__ == "__main__":
    main()
