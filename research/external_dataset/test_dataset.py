"""Boundary tests for the independently versioned external-data importer."""
import hashlib
import json
from pathlib import Path
import subprocess

import numpy as np
import pytest
from qiskit import QuantumCircuit
from qiskit.quantum_info import Operator

from research.external_dataset.prepare import lower_qasm, source_root
from research.external_dataset.analyze import classification, quantile

PROTOCOL = json.loads((Path(__file__).resolve().parents[2]/"docs/EXTERNAL_DATASET_PROTOCOL_V1.json").read_text())


def write_qasm(tmp_path, body):
    path = tmp_path/"test.qasm"
    path.write_text('OPENQASM 2.0; include "qelib1.inc"; '+body)
    return path


def test_conversion_preserves_full_operator_and_terminal_mapping(tmp_path):
    path = write_qasm(tmp_path, 'gate custom a,b { h a; t b; cx a,b; } qreg q[2]; creg c[3]; custom q[0],q[1]; measure q[1] -> c[0]; measure q[0] -> c[2];')
    converted, original, provenance = lower_qasm(path, PROTOCOL)
    restored = QuantumCircuit(2)
    for gate in converted["gates"]:
        method = getattr(restored,gate["name"])
        if gate["name"].startswith("r"):
            method(gate["angle"],*gate["qubits"])
        else:
            method(*gate["qubits"])
    assert Operator(restored).equiv(Operator(original))
    assert provenance["terminal_measurements"] == [[1,0],[0,2]]
    assert provenance["source_classical_bits"] == 3


@pytest.mark.parametrize("body", [
    'qreg q[1]; creg c[1]; measure q[0] -> c[0]; x q[0];',
    'qreg q[1]; reset q[0];',
    'qreg q[1]; creg c[1]; if(c==0) x q[0];',
])
def test_nonunitary_and_mid_circuit_effects_are_not_silently_removed(tmp_path,body):
    with pytest.raises((ValueError,TypeError)):
        lower_qasm(write_qasm(tmp_path,body),PROTOCOL)


def test_unused_classical_register_is_not_an_exclusion(tmp_path):
    converted, _, provenance = lower_qasm(write_qasm(tmp_path,'qreg q[1]; creg c[4]; h q[0];'),PROTOCOL)
    assert converted["width"] == 1
    assert provenance["terminal_measurements"] == []


def test_lowered_resource_limit_is_checked(tmp_path):
    with pytest.raises(ValueError,match="lowered_gate_budget"):
        lower_qasm(write_qasm(tmp_path,'qreg q[1]; x q[0];'),{**PROTOCOL,"max_lowered_gates":0})


def test_archive_hash_is_checked_before_extraction(tmp_path):
    archive=tmp_path/"untrusted.tar.gz"
    archive.write_bytes(b"not the pinned dataset")
    with pytest.raises(ValueError,match="SHA-256"):
        source_root(archive,tmp_path/"cache",hashlib.sha256(b"approved").hexdigest())
    assert not (tmp_path/"cache").exists()


def test_majority_prediction_is_not_good_balanced_detection():
    result=classification([{"label":0,"prediction":1}]+[{"label":1,"prediction":1}]*10)
    assert result["accuracy"] == pytest.approx(10/11)
    assert result["balanced_accuracy"] == .5
    assert result["false_positive_rate"] == 1


def test_finite_calibration_resolution_is_preserved():
    assert quantile([0.,.1,.2],.01) == 1
    assert quantile([0.,.1,.2],.5) == .1


def test_imported_probes_match_native_engine_and_independent_aer(tmp_path):
    from qiskit_aer import AerSimulator
    from qiskit_aer.noise import depolarizing_error
    from qiskit.quantum_info import Statevector
    binary = Path(__file__).resolve().parents[2]/"target/debug/rasqp"
    assert binary.is_file(), "Build the CLI first: cargo build --locked"
    path = write_qasm(tmp_path, 'qreg q[2]; creg c[2]; h q[0]; t q[1]; ch q[0],q[1]; cy q[1],q[0]; ry(0.37) q[1]; measure q -> c;')
    converted, original, _ = lower_qasm(path,PROTOCOL)
    def call(circuit,noise):
        result = subprocess.run([str(binary)],input=json.dumps({"operation":"probabilities","circuit":circuit,"noise":noise}),text=True,capture_output=True,check=True)
        return json.loads(result.stdout)["probabilities"]
    for state,basis in [('0','Z'),('1','Z'),('+','X'),('i','Y')]:
        before,after=[],[]
        prepared=QuantumCircuit(2)
        for q in range(2):
            if state=='1':
                before.append({"name":"x","qubits":[q],"angle":0.});prepared.x(q)
            elif state in {'+','i'}:
                before.append({"name":"h","qubits":[q],"angle":0.});prepared.h(q)
                if state=='i':
                    before.append({"name":"rz","qubits":[q],"angle":np.pi/2});prepared.rz(np.pi/2,q)
        prepared.compose(original,inplace=True)
        for q in range(2):
            if basis=='Y':
                after.append({"name":"rz","qubits":[q],"angle":-np.pi/2});prepared.rz(-np.pi/2,q)
            if basis in {'X','Y'}:
                after.append({"name":"h","qubits":[q],"angle":0.});prepared.h(q)
        native={"width":2,"gates":before+converted["gates"]+after}
        assert np.allclose(call(native,None),Statevector(prepared).probabilities(),atol=1e-12,rtol=0)
        noisy=QuantumCircuit(2)
        for gate in native["gates"]:
            method=getattr(noisy,gate["name"])
            if gate["name"].startswith('r'):
                method(gate["angle"],*gate["qubits"])
            else:
                method(*gate["qubits"])
            arity=len(gate["qubits"])
            noisy.append(depolarizing_error(.005 if arity==1 else .01,arity).to_instruction(),gate["qubits"])
        noisy.save_probabilities()
        probabilities=AerSimulator(method="density_matrix").run(noisy,shots=1).result().data(0)["probabilities"]
        readout=np.array([[.995,.005],[.005,.995]])
        expected=np.kron(readout,readout)@probabilities
        assert np.allclose(call(native,.005),expected,atol=1e-11,rtol=0)
