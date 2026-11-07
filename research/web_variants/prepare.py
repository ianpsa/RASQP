"""Literature-derived interventions on fresh workloads, with no detector fitting.

The downloaded QTrojan notebook is provenance only and is never executed.
"""
import argparse
import hashlib
import json
from pathlib import Path

import numpy as np
from qiskit import QuantumCircuit, transpile
from qiskit.quantum_info import Operator, Statevector

ROOT = Path(__file__).resolve().parents[2]
PROTOCOL = ROOT/'docs/WEB_VARIANTS_PROTOCOL_V1.json'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def detector_hashes():
    files = ['rust/research/probes.rs','rust/research/verification.rs','rust/research/statistics.rs']
    hashes = {name:sha(ROOT/name) for name in files}
    archived = json.loads((ROOT/'results/wildisley-v1/validation_manifest.json').read_text())['source_files_sha256']
    assert all(archived[name]==value for name,value in hashes.items()), 'detector changed since Wildisley evaluation'
    return hashes


def encoding(features, target=None):
    circuit=QuantumCircuit(4)
    if target is not None:
        circuit.rx(np.pi/2,range(4))
    for q,angle in enumerate(features):
        circuit.ry(float(angle),q)
    if target is not None:
        circuit.rx(3*np.pi/2,range(4))
        for q,angle in enumerate(target):
            circuit.ry(float(angle),q)
    return circuit


def ansatz(parameters):
    circuit=QuantumCircuit(4)
    for layer in parameters:
        for q,angles in enumerate(layer):
            circuit.rx(float(angles[0]),q)
            circuit.ry(float(angles[1]),q)
            circuit.rz(float(angles[2]),q)
        for control,target in [(0,1),(1,2),(2,3),(3,0)]:
            circuit.cx(control,target)
    return circuit


def qaoa(edges,gamma,beta,drop_first_cost=False,flip_mixer=False):
    circuit=QuantumCircuit(4)
    circuit.h(range(4))
    for layer,(g,b) in enumerate(zip(gamma,beta)):
        if not (layer==0 and drop_first_cost):
            for left,right in edges:
                circuit.cx(left,right)
                circuit.rz(-float(g),right)
                circuit.cx(left,right)
        circuit.rx(float((-1 if flip_mixer else 1)*2*b),range(4))
    return circuit


def insert_swap(circuit,position):
    before=QuantumCircuit(4)
    for index,item in enumerate(circuit.data):
        if index==position:
            before.swap(0,1)
        before.append(item.operation,[circuit.find_bit(q).index for q in item.qubits])
    if position==len(circuit.data):
        before.swap(0,1)
    return before


def mixed_trigger(circuit,phase=None):
    prefix=QuantumCircuit(4)
    negative=[0,1] if phase is None else [0,2]
    prefix.x(negative)
    if phase is None:
        prefix.mcx([0,1,2],3)
    else:
        prefix.mcp(float(phase),[0,1,2],3)
    prefix.x(negative)
    return prefix.compose(circuit)


def lower(circuit,basis):
    lowered=transpile(circuit,basis_gates=basis,optimization_level=0,seed_transpiler=29551)
    gates=[{'name':item.operation.name,
            'qubits':[lowered.find_bit(q).index for q in item.qubits],
            'angle':float(item.operation.params[0]) if item.operation.params else 0.0} for item in lowered.data]
    assert all(g['name'] in basis for g in gates)
    assert Operator(circuit).equiv(Operator(lowered)), 'gate lowering changed the operator'
    return {'width':4,'gates':gates}


def probabilities(circuit):
    result=[]
    for state,basis in [('0','Z'),('1','Z'),('+','X')]:
        measured=QuantumCircuit(4)
        if state=='1':measured.x(range(4))
        if state=='+':measured.h(range(4))
        measured.compose(circuit,inplace=True)
        if basis=='X':measured.h(range(4))
        result.append(Statevector(measured).probabilities())
    return result


def build(protocol):
    rng=np.random.default_rng(protocol['generator_seed'])
    basis=['x','z','h','rx','ry','rz','cx']
    groups=[]
    graphs=[[(0,1),(1,2),(2,3),(3,0)],[(0,1),(1,2),(2,3)],
            [(0,1),(0,2),(0,3)],[(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)]]
    for family in ('encoding_variational','qaoa_maxcut'):
        count=protocol['workloads'][family]
        for index in range(count):
            if family=='encoding_variational':
                features=rng.uniform(-1.3,1.3,4)
                parameters=rng.uniform(-1.8,1.8,(2,4,3))
                golden=encoding(features).compose(ansatz(parameters))
                scaled={name:encoding(features).compose(ansatz(parameters*factor)) for name,factor in [('parameter_scale_plus10',1.1),('parameter_scale_plus50',1.5),('parameter_scale_minus50',.5)]}
                specific={
                    'qtrojan_encoding_sandwich':encoding(features,protocol['variant_parameters']['encoding_target']).compose(ansatz(parameters)),
                    'encoding_layer_erasure':ansatz(parameters),
                }
                settings={'features':features.tolist(),'ansatz':parameters.tolist()}
            else:
                edges=graphs[index%4]
                gamma=rng.uniform(.1,1.4,2)
                beta=rng.uniform(.1,.7,2)
                golden=qaoa(edges,gamma,beta)
                scaled={name:qaoa(edges,gamma*factor,beta*factor) for name,factor in [('parameter_scale_plus10',1.1),('parameter_scale_plus50',1.5),('parameter_scale_minus50',.5)]}
                specific={'cost_layer_erasure':qaoa(edges,gamma,beta,drop_first_cost=True),
                          'mixer_sign_flip':qaoa(edges,gamma,beta,flip_mixer=True)}
                settings={'edges':edges,'gamma':gamma.tolist(),'beta':beta.tolist()}
            variants={
                'swap_start':insert_swap(golden,0),
                'swap_middle':insert_swap(golden,len(golden.data)//2),
                'swap_end':insert_swap(golden,len(golden.data)),
                **scaled,
                'mixed_trigger_mcx_001':mixed_trigger(golden),
                'mixed_trigger_phase_010':mixed_trigger(golden,protocol['variant_parameters']['phase_angle']),
                **specific,
            }
            expected=set(protocol['variants']['both_workloads']+protocol['variants'][family])
            assert set(variants)==expected and len(variants)==10
            base=f'{family}-{index:02d}'
            samples=[]
            clean=probabilities(golden)
            for variant,circuit in [('clean',golden),*variants.items()]:
                sid=base+'-'+variant
                native=lower(circuit,basis)
                assert len(native['gates'])<=protocol['max_lowered_gates']
                probs=probabilities(circuit)
                distances=[float(np.sum(np.abs(a-b))/2) for a,b in zip(probs,clean)]
                samples.append({'sample_id':sid,'label':int(variant!='clean'),'gate':variant,'mode':family,
                    'circuit':native,'ideal_probe_tvd_qiskit':distances,
                    'operator_changed':not Operator(golden).equiv(Operator(circuit))})
            # No training folds: field retained for executor schema compatibility.
            groups.append({'base_circuit_id':base,'fold':1+index//4,'family':family,
                           'settings':settings,'samples':samples})
    return groups


def prepare(output):
    import qiskit
    protocol=json.loads(PROTOCOL.read_text())
    frozen=detector_hashes()
    groups=build(protocol)
    output.mkdir(parents=True,exist_ok=True)
    input_path=output/'input.jsonl'
    with input_path.open('w') as handle:
        for group in groups:
            handle.write(json.dumps(group,separators=(',',':'))+'\n')
    provenance={'version':protocol['version'],'protocol_sha256':sha(PROTOCOL),'input_sha256':sha(input_path),
                'detector_sha256':frozen,'source':protocol['source'],'qiskit':qiskit.__version__,
                'numpy':np.__version__,'bases':len(groups),'variants':sum(len(g['samples'])-1 for g in groups),
                'meaning':'New locally generated benchmark circuits and article-derived interventions; no external attack dataset or QNN training reproduced.'}
    (output/'preparation.json').write_text(json.dumps(provenance,indent=2)+'\n')
    print(json.dumps({key:provenance[key] for key in ['version','bases','variants','detector_sha256']},indent=2))


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--output',type=Path,default=ROOT/'results/runs/web-variants-input')
    args=parser.parse_args()
    prepare(args.output)
