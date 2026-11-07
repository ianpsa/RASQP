"""Offline physics, IBM primitive interoperability, result mapping and budget tests."""
import json
from pathlib import Path

import numpy as np
import pytest
from qiskit import QuantumCircuit
from qiskit.quantum_info import Operator
from qiskit.circuit import Delay
from qiskit_aer import AerSimulator
from qiskit_aer.noise import NoiseModel, ReadoutError, RelaxationNoisePass
from qiskit_ibm_runtime import SamplerV2
from qiskit_ibm_runtime.fake_provider import FakeManilaV2

from research.ibm import hardware
from research.ibm.common import (ROOT, apply_readout, compact_target, compile_probe,
    density_probabilities, logical_probabilities, restricted_noise, rust_evaluate, variants)

SIM_PROTOCOL=json.loads((ROOT/'docs/IBM_CALIBRATION_PROTOCOL_V1.json').read_text())


def test_asymmetric_readout_and_permuted_measurements_match_analytic_channel():
    matrices=np.array([[[.9,.1],[.2,.8]], [[.7,.3],[.4,.6]]])
    p=apply_readout([0,1,0,0],matrices)
    assert np.allclose(p,[.14,.56,.06,.24],atol=1e-15)
    assert np.allclose(logical_probabilities([0,1,0,0],matrices,[1,0]),[.14,.06,.56,.24],atol=1e-15)
    c=QuantumCircuit(2,2);c.x(0);c.measure([0,1],[0,1])
    noise=NoiseModel()
    for q,m in enumerate(matrices): noise.add_readout_error(ReadoutError(m),[q])
    counts=AerSimulator(noise_model=noise,max_parallel_threads=1).run(c,shots=100000,seed_simulator=29671).result().get_counts()
    sampled=np.array([counts.get(format(i,'02b'),0)/100000 for i in range(4)])
    assert np.abs(sampled-p).sum()/2 < .01


def test_restricted_gate_channels_and_idle_relaxation_match_full_five_qubit_backend():
    backend=FakeManilaV2(); physical=[2,3]
    c=QuantumCircuit(2);c.h(0);c.cx(0,1);c.ry(.37,1)
    isa,mapping=compile_probe(c,'+','X',compact_target(backend,physical),SIM_PROTOCOL)
    model,model_dict,readout=restricted_noise(backend,physical)
    compact,norm=density_probabilities(isa,mapping,model,backend,physical)
    assert norm<1e-7
    assert all('rz' not in error['operations'] for error in model_dict['errors'])
    # Independent execution keeps the actual physical identifiers and full device.
    full=hardware.physical_isa(isa,backend,physical).remove_final_measurements(inplace=False)
    props=[backend.qubit_properties(q) for q in range(backend.num_qubits)]
    full=RelaxationNoisePass([v.t1 for v in props],[min(v.t2,2*v.t1) for v in props],
                             dt=backend.dt,op_types=[Delay])(full)
    full.save_probabilities(physical,label='p')
    sim=AerSimulator(method='density_matrix',noise_model=NoiseModel.from_backend(backend),max_parallel_threads=1)
    expected=np.asarray(sim.run(full,shots=1).result().data(0)['p']);expected/=expected.sum()
    assert np.allclose(compact,expected,rtol=0,atol=1e-10)
    assert abs(readout[0,0,1]-backend.properties().qubit_property(2,'prob_meas1_prep0')[0])<1e-15


def test_sampler_v2_local_mode_preserves_logical_width_and_actual_bitstrings():
    backend=FakeManilaV2(); physical=[2,3]
    # A non-palindromic logical result exposes reversed classical-bit mappings.
    c=QuantumCircuit(2);c.x(0);c.swap(0,1)
    isa,mapping=compile_probe(c,'0','Z',compact_target(backend,physical),SIM_PROTOCOL)
    sampler=SamplerV2(mode=backend,options={'simulator':{'seed_simulator':29671}})
    result=sampler.run([(hardware.physical_isa(isa,backend,physical),None,256)]).result()
    bits=result[0].data.meas.get_bitstrings()
    assert len(bits)==256 and all(len(v)==2 and set(v)<={'0','1'} for v in bits)
    counts=result[0].data.meas.get_counts()
    assert sum(counts.values())==256
    assert counts.get('10',0)>180 and counts.get('10',0)>counts.get('01',0)
    assert sorted(mapping)==[0,1]


def test_external_counts_use_actual_rust_joint_verifier():
    refs=[[1.,0.],[0.,1.],[1.,0.]]
    clean=[[[256,0],[0,256],[256,0]] for _ in range(127)]
    batches=[clean[0],[[0,256],[0,256],[256,0]]]
    value=rust_evaluate(refs,clean,batches,.01)
    assert value['verifier']['threshold']==0
    assert value['rejected']==[False,True]
    assert value['scores']==[0.,1.]


def pilot_fixture(tmp_path,monkeypatch):
    def source(relative):
        width=3 if 'qaoa' in relative else 4
        c=QuantumCircuit(width);c.h(0);c.cx(0,1)
        return c,{'source_path':relative,'source_sha256':'fixture'}
    monkeypatch.setattr(hardware,'unitary_source',source)
    output=tmp_path/'pilot'
    hardware.prepare(output,FakeManilaV2(),local=True)
    return output


def test_hardware_plan_accounts_all_shots_and_cannot_submit_mock_counts(tmp_path,monkeypatch):
    output=pilot_fixture(tmp_path,monkeypatch)
    plan,protocol=hardware.load_plan(output)
    assert plan['total_shots']==735744
    assert sum(protocol['stage_execution_caps_seconds'].values())==420 < 600
    assert [len(plan['stages'][s]['pubs']) for s in hardware.STAGES]==[6,6,30]
    def forbidden(): raise AssertionError('cloud service must not be initialized')
    monkeypatch.setattr(hardware,'service',forbidden)
    with pytest.raises(ValueError,match='mock backend'):
        hardware.submit(output,'reference')
    assert not list(output.glob('*-submission.json'))


def test_modified_pub_shots_fail_before_network_access(tmp_path,monkeypatch):
    output=pilot_fixture(tmp_path,monkeypatch)
    p=output/'plan.json';value=json.loads(p.read_text())
    value['stages']['evaluation']['pubs'][0]['shots']+=1
    p.write_text(json.dumps(value))
    with pytest.raises(ValueError,match='per-PUB budget mismatch'):
        hardware.load_plan(output)


def test_connection_selects_verified_open_instance_and_never_paid(monkeypatch):
    import qiskit_ibm_runtime
    calls=[]
    class Service:
        def __init__(self,**options): calls.append(options);self.options=options
        def instances(self): return [{'crn':'paid','plan':'pay-as-you-go'},{'crn':'open','plan':'open'}]
        def active_instance(self): return self.options['instance']
    monkeypatch.setattr(qiskit_ibm_runtime,'QiskitRuntimeService',Service)
    monkeypatch.setenv('QISKIT_IBM_TOKEN','test-only-not-a-real-key')
    value=hardware.service()
    assert value.active_instance()=='open'
    assert len(calls)==2 and all(c['plans_preference']==['open'] for c in calls)
    assert calls[-1]['instance']=='open'


def test_missing_stage_and_ambiguous_submission_cannot_trigger_retry(tmp_path,monkeypatch):
    output=pilot_fixture(tmp_path,monkeypatch)
    path=output/'plan.json';plan=json.loads(path.read_text());plan['local_mock']=False
    path.write_text(json.dumps(plan))
    def forbidden(): raise AssertionError('network access would create a duplicate or out-of-order job')
    monkeypatch.setattr(hardware,'service',forbidden)
    with pytest.raises(ValueError,match='preceding stage'):
        hardware.submit(output,'evaluation')
    (output/'reference-submission.json').write_text('{"state":"intent_recorded"}')
    with pytest.raises(ValueError,match='submission already attempted'):
        hardware.submit(output,'reference')


def test_identity_pair_and_unapplicable_parameter_case_remain_benign():
    c=QuantumCircuit(2);c.h(0);c.cx(0,1)
    equivalent=variants(c,hardware=True)['equivalent_x_pair']
    assert Operator(c).equiv(Operator(equivalent))
    # No explicit source rotation means this diagnostic makes no intervention.
    assert Operator(c).equiv(Operator(variants(c)['parameter_scale_1.1']))
