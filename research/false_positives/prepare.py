"""Pinned clean benchmarks and QDoor numeric artifacts; never run upstream code."""
import argparse
import ast
import collections
import gzip
import hashlib
import json
from pathlib import Path
import re
import sys
import tarfile
import zipfile

import numpy as np
from qiskit import QuantumCircuit, qasm2, transpile
from qiskit.quantum_info import Operator, Statevector

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from research.web_variants.prepare import detector_hashes
from research.wildisley.prepare import lower_qasm

PROTOCOL = ROOT / 'docs/FALSE_POSITIVE_PROTOCOL_V1.json'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def extract(archive, cache, source):
    if sha(archive) != source['archive_sha256']:
        raise ValueError('upstream archive hash mismatch')
    with tarfile.open(archive) as handle:
        roots = {Path(m.name).parts[0] for m in handle.getmembers()}
        assert len(roots) == 1
        root = roots.pop()
        # QASMBench also contains very large circuits. Only extract data needed.
        members = [m for m in handle.getmembers() if m.isfile() and (
            len(Path(m.name).parts) == 2 or (
                m.name.endswith('.qasm') and
                Path(m.name).parts[1] == source.get('subdirectory')) or (
                'subdirectory' not in source and m.name.endswith(('.py', '.pt', '.qasm'))))]
        handle.extractall(cache, members=members, filter='data')
    return cache / root


def lower(circuit, protocol, optimization=0):
    c = transpile(circuit, basis_gates=protocol['lowering']['basis'],
                  optimization_level=optimization,
                  seed_transpiler=protocol['lowering']['seed_transpiler'])
    if c.layout is not None:
        # A compiler may remove SWAPs by changing its logical output mapping.
        # Our native boundary is fixed wire order, so materialize that mapping.
        assert c.layout.initial_index_layout(filter_ancillas=True) == list(range(c.num_qubits))
        permutation = c.layout.final_index_layout()
        restored = QuantumCircuit(c.num_qubits)
        restored.global_phase = c.global_phase
        for item in c.data:
            restored.append(item.operation, [c.find_bit(q).index for q in item.qubits])
        for wire in range(c.num_qubits):
            other = permutation[wire]
            if other != wire:
                restored.swap(wire, other)
                permutation = [other if x == wire else wire if x == other else x for x in permutation]
        c = transpile(restored, basis_gates=protocol['lowering']['basis'], optimization_level=0,
                      seed_transpiler=protocol['lowering']['seed_transpiler'])
    if len(c.data) > protocol['max_lowered_gates']:
        raise ValueError('lowered_gate_budget')
    a, b = Operator(circuit).data, Operator(c).data
    phase = np.vdot(b.ravel(), a.ravel()) / a.shape[0]
    assert abs(abs(phase)-1) < 1e-9 and np.max(np.abs(a-phase*b)) < 1e-9, 'lowering/operator mismatch'
    native = {'width': c.num_qubits, 'gates': [
        {'name': x.operation.name,
         'qubits': [c.find_bit(q).index for q in x.qubits],
         'angle': float(x.operation.params[0]) if x.operation.params else 0.0}
        for x in c.data]}
    return native, c


def probabilities(circuit):
    values = []
    for state, basis in [('0', 'Z'), ('1', 'Z'), ('+', 'X')]:
        c = QuantumCircuit(circuit.num_qubits)
        if state == '1': c.x(range(c.num_qubits))
        if state == '+': c.h(range(c.num_qubits))
        c.compose(circuit, inplace=True)
        if basis == 'X': c.h(range(c.num_qubits))
        values.append(Statevector(c).probabilities())
    return values


def sample(sid, role, circuit, golden, family, protocol, label=0, native=None):
    if native is None:
        native, _ = lower(circuit, protocol)
    clean, candidate = probabilities(golden), probabilities(circuit)
    return {'sample_id': sid, 'label': label, 'gate': role, 'mode': family,
            'circuit': native,
            'ideal_probe_tvd_qiskit': [float(np.sum(np.abs(a-b))/2)
                                      for a, b in zip(clean, candidate)],
            'operator_equivalent': Operator(golden).equiv(Operator(circuit))}


WEIGHTS = {
    'init_whole8.pt': ('f38ea8a7d33418f3760257684fbcbd9f93925cfbec563da77cb2a73955bc1f98',
                       '7c473d66e2f41902b4f440bc5de0b1e1c5e5df34e934239a7d0019129c1a36aa', 28),
    'backdoor_init.pt': ('86385f47a399c49b7fab8f929888089f5326aa80872a9b41a683bcedfe0c8942',
                         '145bfaac67b7836bde71fcb6b94288b680c0191229ded6b11a1cbdb22c1ddc99', 48),
    'backdoor.pt': ('a626b6380672f74554bb2d076c303a6be64503d3fce070599822c8c6525a9702',
                    '145bfaac67b7836bde71fcb6b94288b680c0191229ded6b11a1cbdb22c1ddc99', 48),
}


def numeric_weights(path):
    """Extract only pinned float32 storage. Never invoke a pickle interpreter."""
    file_sha, metadata_sha, count = WEIGHTS[path.name]
    if sha(path) != file_sha:
        raise ValueError('unrecognized QDoor checkpoint')
    with zipfile.ZipFile(path) as handle:
        metadata = handle.read('archive/data.pkl')
        if hashlib.sha256(metadata).hexdigest() != metadata_sha:
            raise ValueError('unrecognized tensor metadata')
        data = handle.read('archive/data/0')
    assert len(data) == count * 4
    result = np.frombuffer(data, dtype='<f4').astype(float)
    assert result.shape == (count,) and np.isfinite(result).all()
    return result


def original_block(weights):
    c = QuantumCircuit(8)
    for q in range(8):
        c.rx(weights[q], q)
        c.ry(weights[8+q], q)
        c.rz(weights[16+q], q)
    for i, (left, right) in enumerate([(0, 1), (2, 3), (4, 5), (6, 7)]):
        c.crx(weights[24+i], left, right)
    return c


def compiled_block(source_path, weights):
    """Translate the explicit U2/CNOT function using a restrictive AST reader."""
    module = ast.parse(source_path.read_text())
    function = next(n for n in module.body if isinstance(n, ast.FunctionDef) and n.name == 'circuit')
    c = QuantumCircuit(8)
    used = []
    for statement in function.body:
        assert isinstance(statement, ast.Expr) and isinstance(statement.value, ast.Call)
        call = statement.value
        assert isinstance(call.func, ast.Attribute) and isinstance(call.func.value, ast.Name)
        assert call.func.value.id == 'qml'
        assert len(call.keywords) == 1 and call.keywords[0].arg == 'wires'
        wires = ast.literal_eval(call.keywords[0].value)
        if call.func.attr == 'CNOT':
            assert not call.args and len(wires) == 2
            c.cx(*wires)
        else:
            assert call.func.attr == 'U2' and len(call.args) == 1
            argument = call.args[0]
            assert isinstance(argument, ast.Starred) and isinstance(argument.value, ast.Subscript)
            argument = argument.value
            assert isinstance(argument.value, ast.Name) and argument.value.id == 'w'
            assert isinstance(argument.slice, ast.Slice) and argument.slice.step is None
            start, stop = ast.literal_eval(argument.slice.lower), ast.literal_eval(argument.slice.upper)
            assert stop == start + 2 and 0 <= start < stop <= len(weights)
            used.extend(range(start, stop))
            c.u(np.pi/2, weights[start], weights[start+1], wires)
    assert sorted(used) == list(range(48))
    return c


def qdoor_groups(source, protocol):
    assert sha(source/'original.py') == 'e56be9c02576a6afc3f60315c13b561571e2ea70fc6e90aa1a2df4d337a986e3'
    assert sha(source/'backdoor.py') == '8c008f2cb5aa251dd50e86e4bc98b859179d6b9508bb58e2dc86f21788fa5a57'
    weights = {name: numeric_weights(source/'model'/name) for name in WEIGHTS}
    original = original_block(weights['init_whole8.pt'])
    initialized = compiled_block(source/'backdoor.py', weights['backdoor_init.pt'])
    attacked = compiled_block(source/'backdoor.py', weights['backdoor.pt'])
    supplied = qasm2.load(source/'qasm/circuit.qasm')
    qasm_angles = np.array([float(v) for x in supplied.data if x.operation.name == 'u2'
                            for v in x.operation.params])
    # The supplied QASM and float32 initialization need not be numerically equal.
    # Record the relationship instead of labeling this QASM as an attack.
    a = Operator(original).data
    diagnostic = {'qasm_initialization_max_parameter_error': float(np.max(np.abs(
        qasm_angles - weights['backdoor_init.pt']))), 'overlap_distance_1_minus_normalized_abs_trace': {}}
    for name, circuit in [('compiled_initialization', initialized), ('trained_backdoor', attacked),
                          ('supplied_qasm', supplied)]:
        b = Operator(circuit).data
        diagnostic['overlap_distance_1_minus_normalized_abs_trace'][name] = float(
            1-abs(np.sum(a*np.conj(b)))/a.shape[0])
    groups = []
    for index, features in enumerate(protocol['qdoor']['feature_vectors']):
        prefix = QuantumCircuit(8)
        for q, value in enumerate(features): prefix.ry(float(value), q)
        circuits = {name: prefix.compose(c) for name, c in [
            ('original', original), ('compiled_initialization', initialized), ('trained_backdoor', attacked)]}
        for approved in ('original', 'compiled_initialization'):
            golden = circuits[approved]
            base = f'QDoor-{index:02d}-{approved}'
            samples = [sample(base+'-approved', 'approved', golden, golden, 'QDoor', protocol)]
            if approved == 'original':
                samples.append(sample(base+'-compiled', 'compiled_initialization',
                                      circuits['compiled_initialization'], golden, 'QDoor', protocol))
            samples.append(sample(base+'-attacked', 'trained_backdoor', circuits['trained_backdoor'],
                                  golden, 'QDoor', protocol, label=1))
            groups.append({'base_circuit_id': base, 'fold': 0, 'family': 'QDoor',
                           'approved': approved, 'features': features, 'samples': samples})
    return groups, diagnostic


def prepare(archives, cache, output):
    import qiskit
    protocol = json.loads(PROTOCOL.read_text())
    frozen = detector_hashes()
    cache.mkdir(parents=True, exist_ok=True)
    output.mkdir(parents=True, exist_ok=True)
    if (output/'input.jsonl').exists():
        raise ValueError('prepared output must be fresh')
    sources = {name: extract(path, cache, protocol['source'][name]) for name, path in archives.items()}
    previous_hashes = set()
    previous_inventory = ROOT/'results/wildisley-v1/inventory.jsonl.gz'
    if previous_inventory.exists():
        with gzip.open(previous_inventory, 'rt') as handle:
            previous_hashes = {json.loads(line)['sha256'] for line in handle}
    groups, inventory = [], []
    for family in ('QASMBench', 'VeriQBench'):
        source = sources[family]
        paths = sorted((source/protocol['source'][family]['subdirectory']).rglob('*.qasm'))
        for path in paths:
            relative = str(path.relative_to(source))
            width = sum(map(int, re.findall(r'qreg\s+\w+\[(\d+)\]', path.read_text())))
            row = {'family': family, 'source_path': relative, 'sha256': sha(path), 'width': width,
                   'byte_identical_to_wildisley': sha(path) in previous_hashes}
            inventory.append(row)
            if not 1 <= width <= protocol['benchmark_max_qubits']:
                row['excluded'] = 'width_budget'
                continue
            try:
                native, original, conversion = lower_qasm(path, protocol)
            except Exception as error:
                row['excluded'] = str(error)
                continue
            row.update({'eligible': True, 'conversion': conversion,
                        'original_gates': len(native['gates'])})
            base = family+'-'+relative.replace('/', '__')
            group = {'base_circuit_id': base+'-original', 'fold': 0, 'family': family,
                     'source_path': relative, 'source_sha256': row['sha256'], 'approved': 'original',
                     'samples': [sample(base+'-original', 'approved', original, original, family, protocol, native=native)]}
            try:
                opt_native, optimized = lower(original, protocol, optimization=3)
            except Exception as error:
                # A failed optional rewrite never removes an eligible original.
                row['rewrite_excluded'] = str(error)
            else:
                row.update({'optimized_gates': len(opt_native['gates']), 'operator_equivalence_verified': True})
                group['samples'].append(sample(base+'-optimized', 'exact_optimized_rewrite', optimized, original,
                                               family, protocol, native=opt_native))
                groups.append({'base_circuit_id': base+'-optimized', 'fold': 0, 'family': family,
                               'source_path': relative, 'source_sha256': row['sha256'], 'approved': 'optimized',
                               'samples': [sample(base+'-optimized-approved', 'approved', optimized, optimized,
                                                  family, protocol, native=opt_native)]})
            groups.append(group)
            # Retain exact author QASM as data, never executable code.
            destination = output/'sources'/family/relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(path.read_bytes())
    qgroups, qdiagnostic = qdoor_groups(sources['QDoor'], protocol)
    groups.extend(qgroups)
    for relative in ['original.py', 'backdoor.py', 'compile.py', 'qasm/circuit.qasm',
                     *['model/'+name for name in WEIGHTS]]:
        destination = output/'sources/QDoor'/relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes((sources['QDoor']/relative).read_bytes())
    input_path = output/'input.jsonl'
    with input_path.open('w') as handle:
        for group in groups: handle.write(json.dumps(group, separators=(',', ':'))+'\n')
    (output/'inventory.json').write_text(json.dumps(inventory, indent=2)+'\n')
    sources_sha = {str(p.relative_to(output)): sha(p) for p in sorted((output/'sources').rglob('*')) if p.is_file()}
    preparation = {'version': protocol['version'], 'protocol_sha256': sha(PROTOCOL),
                   'input_sha256': sha(input_path), 'detector_sha256': frozen,
                   'source': protocol['source'], 'qiskit': qiskit.__version__, 'numpy': np.__version__,
                   'groups': len(groups), 'eligible_benchmark_circuits': sum(bool(r.get('eligible')) for r in inventory),
                   'inventory_counts': dict(collections.Counter(r['family'] for r in inventory)),
                   'exclusions': dict(collections.Counter(r['excluded'] for r in inventory if 'excluded' in r)),
                   'byte_identical_wildisley_eligible': sum(bool(r.get('eligible')) and r['byte_identical_to_wildisley'] for r in inventory),
                   'source_files_sha256': sources_sha, 'qdoor_diagnostics': qdiagnostic}
    (output/'preparation.json').write_text(json.dumps(preparation, indent=2)+'\n')
    print(json.dumps({k: preparation[k] for k in ['groups', 'eligible_benchmark_circuits',
          'inventory_counts', 'exclusions', 'byte_identical_wildisley_eligible', 'qdoor_diagnostics']}, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    for name in ('qasmbench', 'veriqbench', 'qdoor'):
        parser.add_argument('--'+name+'-archive', type=Path, required=True)
    parser.add_argument('--cache', type=Path, default=ROOT/'results/runs/false-positives-sources')
    parser.add_argument('--output', type=Path, default=ROOT/'results/runs/false-positives-input')
    args = parser.parse_args()
    prepare({'QASMBench': args.qasmbench_archive, 'VeriQBench': args.veriqbench_archive,
             'QDoor': args.qdoor_archive}, args.cache, args.output)
