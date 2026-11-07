"""Independent Aer channel check on every distinct QDoor artifact and probe."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys

import numpy as np
from qiskit import QuantumCircuit
from qiskit.quantum_info import Statevector
from qiskit_aer import AerSimulator
from qiskit_aer.noise import depolarizing_error

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from research.false_positives.prepare import sha


def probed(circuit, state, basis):
    width = circuit['width']
    before, after = [], []
    if state in ('1', '+'):
        before = [{'name': 'x' if state == '1' else 'h', 'qubits': [q], 'angle': 0.0}
                  for q in range(width)]
    if basis == 'X':
        after = [{'name': 'h', 'qubits': [q], 'angle': 0.0} for q in range(width)]
    return {'width': width, 'gates': before+circuit['gates']+after}


def qiskit_circuit(native, noise=None):
    c = QuantumCircuit(native['width'])
    for gate in native['gates']:
        method = getattr(c, gate['name'])
        if gate['name'] in ('rx', 'ry', 'rz'):
            method(gate['angle'], *gate['qubits'])
        else:
            method(*gate['qubits'])
        if noise is not None:
            arity = len(gate['qubits'])
            c.append(depolarizing_error(noise if arity == 1 else min(2*noise, 1), arity).to_instruction(), gate['qubits'])
    return c


def validate(output, binary):
    groups = [json.loads(line) for line in (output/'circuits.jsonl').open()]
    distinct = {}
    for group in groups:
        if group['family'] != 'QDoor': continue
        for sample in group['samples']:
            distinct[json.dumps(sample['circuit'], sort_keys=True)] = sample['circuit']
    ideal_error, noisy_error, checked = 0.0, 0.0, []
    simulator = AerSimulator(method='density_matrix', max_parallel_threads=1)
    for index, native in enumerate(distinct.values()):
        for state, basis in [('0', 'Z'), ('1', 'Z'), ('+', 'X')]:
            circuit = probed(native, state, basis)
            expected_ideal = Statevector(qiskit_circuit(circuit)).probabilities()
            noisy = qiskit_circuit(circuit, .005)
            noisy.save_probabilities()
            expected_noisy = np.asarray(simulator.run(noisy, shots=1).result().data(0)['probabilities'])
            # Explicit independent classical readout channel in little-endian order.
            readout = np.array([[.995, .005], [.005, .995]])
            channel = np.array([[1.0]])
            for _ in range(native['width']): channel = np.kron(channel, readout)
            expected_noisy = channel @ expected_noisy
            errors = []
            for noise, expected in [(None, expected_ideal), (.005, expected_noisy)]:
                result = subprocess.run([str(binary)], input=json.dumps({
                    'operation': 'probabilities', 'circuit': circuit, 'noise': noise}),
                    capture_output=True, text=True, check=True)
                observed = np.array(json.loads(result.stdout)['probabilities'])
                error = float(np.max(np.abs(observed-expected)))
                assert error < 1e-11, (index, state, basis, noise, error)
                errors.append(error)
            ideal_error = max(ideal_error, errors[0])
            noisy_error = max(noisy_error, errors[1])
            checked.append({'artifact_index': index, 'input': state, 'basis': basis,
                            'native_circuit_sha256': hashlib.sha256(
                                json.dumps(circuit, sort_keys=True).encode()).hexdigest(),
                            'maximum_probability_error_ideal': errors[0],
                            'maximum_probability_error_noisy': errors[1]})
        print(f'QDoor native/Aer validation: {index+1}/{len(distinct)} artifacts', flush=True)
    import qiskit_aer
    evidence = {'scope': 'All nine distinct eight-qubit QDoor circuits under every fixed probe. Explicit native primitive depolarization/readout channels versus Aer density matrices. Not validation of physical hardware noise or the article classification pipeline.',
                'distinct_artifacts': len(distinct), 'comparisons': len(checked)*2,
                'native_binary_sha256': sha(binary), 'qiskit_aer': qiskit_aer.__version__,
                'native_summary_sha256': sha(output/'native_summary.json'),
                'maximum_ideal_probability_error': ideal_error,
                'maximum_noisy_probability_error': noisy_error, 'tolerance': 1e-11,
                'checks': checked}
    (output/'oracle_validation.json').write_text(json.dumps(evidence, indent=2)+'\n')
    print(json.dumps({k: v for k, v in evidence.items() if k != 'checks'}, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, default=ROOT/'results/false-positives-v1')
    parser.add_argument('--binary', type=Path, default=ROOT/'target/release/rasqp')
    args = parser.parse_args()
    validate(args.output, args.binary.resolve())
