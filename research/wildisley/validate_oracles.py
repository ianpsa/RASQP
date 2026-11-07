"""Check native ideal probe distances against original author QASM in Qiskit.

No lowering/transpiler is used for the reference calculation. This covers every
eligible source sample under the three actual probes, not arbitrary input states.
"""
import argparse
import hashlib
import json
from pathlib import Path

import numpy as np
from qiskit import QuantumCircuit, qasm2
from qiskit.quantum_info import Statevector

ROOT = Path(__file__).resolve().parents[2]


def original_probabilities(path):
    loaded = qasm2.load(path, custom_instructions=qasm2.LEGACY_CUSTOM_INSTRUCTIONS)
    unitary = QuantumCircuit(loaded.num_qubits)
    for item in loaded.data:
        if item.operation.name in {"barrier", "measure"}:
            continue
        assert not item.clbits and item.operation.name not in {"reset", "if_else", "while_loop"}
        unitary.append(item.operation, [loaded.find_bit(q).index for q in item.qubits])
    probabilities = []
    for state, basis in [('0','Z'),('1','Z'),('+','X')]:
        circuit = QuantumCircuit(unitary.num_qubits)
        for q in range(unitary.num_qubits):
            if state=='1':
                circuit.x(q)
            elif state=='+':
                circuit.h(q)
        circuit.compose(unitary,inplace=True)
        if basis=='X':
            circuit.h(range(unitary.num_qubits))
        probabilities.append(Statevector(circuit).probabilities())
    return probabilities


def validate(prepared, output, cache):
    preparation = json.loads((prepared/'preparation.json').read_text())
    summary_path = output/'native_summary.json'
    summary = json.loads(summary_path.read_text())
    source = cache/('IC-QT-2026-'+preparation['source']['commit'])
    groups = [json.loads(line) for line in (prepared/'input.jsonl').open()]
    expected = {(r['context']['noise'],r['sample_id']):r['ideal_probe_tvd'] for r in summary['outcomes'] if r['case']!='equivalent_x_pair'}
    checked_samples,checked_distances,maximum_error = 0,0,0.0
    for index,group in enumerate(groups):
        reference = None
        for sample in group['samples']:
            path = source/sample['source_path']
            assert hashlib.sha256(path.read_bytes()).hexdigest()==sample['sha256']
            probabilities = original_probabilities(path)
            if sample['label']==0:
                reference = probabilities
            assert reference is not None
            distances = [float(np.sum(np.abs(a-b))/2) for a,b in zip(probabilities,reference)]
            for noise in (0.0,.005):
                error = max(abs(a-b) for a,b in zip(distances,expected[noise,sample['sample_id']]))
                maximum_error = max(maximum_error,error)
                assert error < 1e-9,(sample['sample_id'],distances,expected[noise,sample['sample_id']])
                checked_distances += 3
            checked_samples += 1
        if (index+1)%50==0:
            print(f"Original-QASM oracle parity: {index+1}/{len(groups)} base groups",flush=True)
    evidence = {'source_commit':preparation['source']['commit'],
                'native_summary_sha256':hashlib.sha256(summary_path.read_bytes()).hexdigest(),
                'source_samples_checked':checked_samples,'probe_distances_checked_in_two_contexts':checked_distances,
                'maximum_absolute_tvd_error':maximum_error,'tolerance':1e-9,
                'scope':'Original author QASM simulated directly by Qiskit versus recorded native ideal TVDs for all three fixed probes; no claim of all-input operator equivalence or noisy-hardware validation.'}
    (output/'oracle_validation.json').write_text(json.dumps(evidence,indent=2)+'\n')
    print(json.dumps(evidence,indent=2))


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--prepared',type=Path,default=ROOT/'results/runs/wildisley-input')
    parser.add_argument('--output',type=Path,default=ROOT/'results/wildisley-v1')
    parser.add_argument('--cache',type=Path,default=ROOT/'results/runs/wildisley-source')
    args=parser.parse_args()
    validate(args.prepared,args.output,args.cache)
