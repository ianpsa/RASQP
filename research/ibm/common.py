"""Shared IBM experiment boundaries: logical mappings, calibration and budgets."""
import copy
import datetime
import hashlib
import io
import json
from pathlib import Path
import subprocess
import sys

import numpy as np
from qiskit import QuantumCircuit, qpy, transpile
from qiskit.circuit import Delay
from qiskit.quantum_info import Operator
from qiskit.transpiler import Target
from qiskit_aer import AerSimulator
from qiskit_aer.noise import NoiseModel, RelaxationNoisePass

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from research.wildisley.prepare import lower_qasm
from research.wildisley.analyze import quantile, wilson
from research.web_variants.prepare import detector_hashes


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def default_json(value):
    if isinstance(value, np.ndarray): return value.tolist()
    if isinstance(value, np.generic): return value.item()
    if isinstance(value, complex): return {'__complex__': [value.real, value.imag]}
    if isinstance(value, (datetime.date, datetime.datetime)): return value.isoformat()
    raise TypeError(type(value).__name__)


def write_json(path, value):
    Path(path).write_text(json.dumps(value, indent=2, default=default_json)+'\n')


def fresh(path):
    path = Path(path)
    if path.exists() and any(path.iterdir()): raise ValueError('output directory must be fresh')
    path.mkdir(parents=True, exist_ok=True)
    return path


def unitary_source(relative):
    path = ROOT/'results/false-positives-v1/sources/QASMBench'/relative
    manifest = json.loads((ROOT/'results/false-positives-v1/preparation_manifest.json').read_text())
    expected = manifest['source_files_sha256']['sources/QASMBench/'+relative]
    if sha(path) != expected: raise ValueError('source QASM changed')
    _, circuit, conversion = lower_qasm(path, {'lowering': {'basis': ['x','z','h','rx','ry','rz','cx'],
                                                          'seed_transpiler': 19451},
                                              'max_lowered_gates': 4080})
    return circuit, {'source_path': relative, 'source_sha256': expected, 'conversion': conversion}


def connected_paths(backend, width, count=2):
    edges = backend.coupling_map.get_edges()
    graph = {q: set() for q in range(backend.num_qubits)}
    for a, b in edges: graph[a].add(b); graph[b].add(a)
    chosen, occupied = [], set()
    def walk(path):
        if len(path) == width: return path
        for q in sorted(graph[path[-1]]):
            if q not in path and q not in occupied:
                result = walk(path+[q])
                if result: return result
        return None
    for start in sorted(graph):
        if start in occupied: continue
        result = walk([start])
        if result:
            chosen.append(result); occupied.update(result)
            if len(chosen) == count: return chosen
    raise ValueError('not enough disjoint connected physical paths')


def compact_target(backend, physical):
    mapping = {p: i for i, p in enumerate(physical)}
    target = Target(num_qubits=len(physical), dt=backend.dt,
                    qubit_properties=[backend.qubit_properties(p) for p in physical],
                    pulse_alignment=backend.target.pulse_alignment,
                    acquire_alignment=backend.target.acquire_alignment)
    for name in backend.target.operation_names:
        if name in {'if_else','for_loop','while_loop','switch_case','break_loop','continue_loop'}: continue
        properties = {}
        for qargs, info in backend.target[name].items():
            if qargs is None: continue
            if all(q in mapping for q in qargs):
                properties[tuple(mapping[q] for q in qargs)] = copy.copy(info)
        if properties: target.add_instruction(backend.target.operation_from_name(name), properties, name=name)
    return target


def asymmetric_readout(backend, physical):
    matrices = []
    props = backend.properties()
    for q in physical:
        values = {v.name: v.value for v in props.qubits[q]}
        p10 = values.get('prob_meas1_prep0', values['readout_error'])
        p01 = values.get('prob_meas0_prep1', values['readout_error'])
        matrices.append([[1-p10, p10], [p01, 1-p01]])
    return np.array(matrices, dtype=float)


def restricted_noise(backend, physical):
    mapping = {p: i for i, p in enumerate(physical)}
    complete = NoiseModel.from_backend(backend).to_dict()
    restricted = {'errors': []}
    for error in complete['errors']:
        if error['type'] == 'roerror': continue  # Applied explicitly, exactly once.
        for qargs in error.get('gate_qubits', []):
            if all(q in mapping for q in qargs):
                e = copy.deepcopy(error)
                e['gate_qubits'] = [[mapping[q] for q in qargs]]
                restricted['errors'].append(e)
    return NoiseModel.from_dict(restricted), restricted, asymmetric_readout(backend, physical)


def apply_readout(probabilities, matrices, factor=1.0):
    p = np.array(probabilities, dtype=float, copy=True)
    width = len(matrices)
    assert p.shape == (1 << width,)
    for q, matrix in enumerate(matrices):
        p10 = min(float(matrix[0, 1])*factor, .5) if factor != 1 else float(matrix[0, 1])
        p01 = min(float(matrix[1, 0])*factor, .5) if factor != 1 else float(matrix[1, 0])
        for i in range(len(p)):
            if not i & (1 << q):
                j = i | (1 << q)
                a, b = p[i], p[j]
                p[i] = a*(1-p10)+b*p01
                p[j] = a*p10+b*(1-p01)
    assert np.isfinite(p).all() and p.min() > -1e-12 and abs(p.sum()-1) < 1e-9
    p = np.maximum(p, 0)
    return p/p.sum()


def variants(clean, hardware=False):
    result = {'clean': clean.copy()}
    if hardware:
        result['equivalent_x_pair'] = clean.copy(); result['equivalent_x_pair'].x([clean.num_qubits-1]*2)
    else:
        optimized = transpile(clean, basis_gates=['x','z','h','rx','ry','rz','cx'], optimization_level=3,
                              seed_transpiler=29671)
        # Operator.from_circuit respects virtual wire permutations.
        a, b = Operator(clean).data, Operator.from_circuit(optimized).data
        phase = np.vdot(b.ravel(), a.ravel()) / len(a)
        if abs(abs(phase)-1) < 1e-9 and np.max(np.abs(a-phase*b)) < 1e-9:
            # Materialize permutations into the logical circuit before compilation.
            permutation = optimized.layout.final_index_layout() if optimized.layout else list(range(clean.num_qubits))
            c = QuantumCircuit(clean.num_qubits); c.global_phase = optimized.global_phase
            for item in optimized.data: c.append(item.operation, [optimized.find_bit(q).index for q in item.qubits])
            for wire in range(clean.num_qubits):
                other = permutation[wire]
                if other != wire:
                    c.swap(wire, other)
                    permutation = [other if x == wire else wire if x == other else x for x in permutation]
            assert Operator(clean).equiv(Operator(c))
            result['equivalent_optimized'] = c
    for name, gate in [('x0','x'), ('z0','z')]:
        result[name] = clean.copy(); getattr(result[name], gate)(0)
    result['rz0_0.12'] = clean.copy(); result['rz0_0.12'].rz(.12, 0)
    if not hardware:
        result['cx01'] = clean.copy(); result['cx01'].cx(0,1)
        result['swap_end'] = clean.copy(); result['swap_end'].swap(0,1)
        c = QuantumCircuit(clean.num_qubits)
        for item in clean.data:
            op = item.operation
            if op.name in ('rx','ry','rz'): getattr(c,op.name)(float(op.params[0])*1.1, clean.find_bit(item.qubits[0]).index)
            else: c.append(op,[clean.find_bit(q).index for q in item.qubits])
        result['parameter_scale_1.1'] = c
    return result


def probe(circuit, state, basis):
    c = QuantumCircuit(circuit.num_qubits)
    if state == '1': c.x(range(c.num_qubits))
    if state == '+': c.h(range(c.num_qubits))
    c.compose(circuit, inplace=True)
    if basis == 'X': c.h(range(c.num_qubits))
    c.measure_all()
    return c


def compile_probe(circuit, state, basis, target, protocol):
    p = protocol.get('compiler', {'optimization_level':1,'seed_transpiler':29681,'scheduling_method':'alap'})
    result = transpile(probe(circuit,state,basis), target=target,
                       initial_layout=list(range(circuit.num_qubits)),
                       optimization_level=p['optimization_level'], seed_transpiler=p['seed_transpiler'],
                       scheduling_method=p['scheduling_method'])
    # Measurements explicitly map final physical wires to original classical bits.
    measurements = {result.find_bit(x.clbits[0]).index: result.find_bit(x.qubits[0]).index
                    for x in result.data if x.operation.name == 'measure'}
    if set(measurements) != set(range(circuit.num_qubits)) or sorted(measurements.values()) != list(range(circuit.num_qubits)):
        raise ValueError('non-bijective logical measurement map')
    return result, [measurements[i] for i in range(circuit.num_qubits)]


def density_probabilities(isa, measurement_map, model, backend, physical):
    c = isa.remove_final_measurements(inplace=False)
    # Save in physical compact order, then apply readout and reorder to logical bits.
    props = [backend.qubit_properties(q) for q in physical]
    t1s = [v.t1 for v in props]; t2s = [min(v.t2, 2*v.t1) for v in props]
    c = RelaxationNoisePass(t1s, t2s, dt=backend.dt, op_types=[Delay])(c)
    c.save_probabilities(list(range(len(physical))), label='rasqp_probability')
    simulator = AerSimulator(method='density_matrix', noise_model=model, max_parallel_threads=1)
    p = np.asarray(simulator.run(c, shots=1).result().data(0)['rasqp_probability'])
    normalization_error = float(abs(p.sum()-1))
    # Serialized backend Kraus operators accumulate small floating-point error.
    # Record the correction; reject larger loss rather than silently hiding it.
    assert np.isfinite(p).all() and p.min() >= -1e-12 and normalization_error < 1e-7
    return p/p.sum(), normalization_error


def logical_probabilities(physical_probabilities, matrices, measurement_map, factor=1):
    p = apply_readout(physical_probabilities, matrices, factor)
    result = np.zeros_like(p)
    for physical, value in enumerate(p):
        logical = sum(((physical >> q)&1) << bit for bit,q in enumerate(measurement_map))
        result[logical] += value
    assert abs(result.sum()-1) < 1e-9
    return result


def rust_evaluate(references, clean, batches, alpha, binary=None):
    binary = binary or ROOT/'target/release/rasqp'
    request = {'operation':'evaluate_joint', 'references':references, 'clean':clean, 'batches':batches, 'alpha':alpha}
    result = subprocess.run([str(binary)], input=json.dumps(request), capture_output=True, text=True, check=True)
    observed = json.loads(result.stdout)
    def score(batch):
        return max(float(np.abs(np.array(counts)/sum(counts)-np.array(reference)).sum()/2)
                   for counts,reference in zip(batch,references))
    threshold = quantile([score(b) for b in clean], alpha)
    scores = [score(b) for b in batches]
    assert abs(threshold-observed['verifier']['threshold']) < 1e-12
    assert np.max(np.abs(np.array(scores)-observed['scores']), initial=0) < 1e-12
    assert [s > threshold+1e-12 for s in scores] == observed['rejected']
    return observed


def rate(flags):
    k, n = sum(flags), len(flags)
    return {'alarms':int(k), 'decisions':n, 'rate':float(k/n), 'wilson95':wilson(k,n)}


def qpy_write(path, circuits):
    with Path(path).open('wb') as handle: qpy.dump(circuits,handle)


def qpy_read(path):
    with Path(path).open('rb') as handle: return qpy.load(handle)
