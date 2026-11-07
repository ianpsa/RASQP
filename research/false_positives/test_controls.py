"""Independent physical and provenance boundaries, without external downloads."""
import json
from pathlib import Path

import numpy as np
import pytest
from qiskit import QuantumCircuit, transpile
from qiskit.quantum_info import Operator

from research.false_positives.prepare import lower, numeric_weights, original_block

ROOT = Path(__file__).resolve().parents[2]
PROTOCOL = json.loads((ROOT/'docs/FALSE_POSITIVE_PROTOCOL_V1.json').read_text())


def test_compiler_output_mapping_is_restored_to_logical_wire_order():
    c = QuantumCircuit(3)
    c.h(0)
    c.cx(0, 2)
    c.rz(.37, 1)
    c.swap(0, 1)
    c.swap(1, 2)
    raw_optimized = transpile(c, basis_gates=PROTOCOL['lowering']['basis'],
                             optimization_level=3, seed_transpiler=19451)
    assert raw_optimized.layout is not None
    assert raw_optimized.layout.final_index_layout() != [0, 1, 2]
    native, optimized = lower(c, PROTOCOL, optimization=3)
    assert Operator(c).equiv(Operator(optimized))
    rebuilt = QuantumCircuit(3)
    for gate in native['gates']:
        method = getattr(rebuilt, gate['name'])
        if gate['name'] in ('rx', 'ry', 'rz'):
            method(gate['angle'], *gate['qubits'])
        else:
            method(*gate['qubits'])
    assert Operator(c).equiv(Operator(rebuilt))


def test_checkpoint_extraction_rejects_unpinned_data_without_deserialization(tmp_path):
    checkpoint = tmp_path/'backdoor.pt'
    checkpoint.write_bytes(b'not a pinned tensor archive')
    with pytest.raises(ValueError, match='unrecognized QDoor checkpoint'):
        numeric_weights(checkpoint)


def test_clean_compilation_and_maliciousness_are_separate_labels():
    # A legitimate approximation can be physically distinguishable from the
    # approved original; distribution shift alone cannot identify intent.
    clean = original_block(np.zeros(28))
    approximate = clean.copy()
    approximate.rz(.02, 0)
    assert not Operator(clean).equiv(Operator(approximate))
    assert PROTOCOL['alpha'] == .01
    assert PROTOCOL['shots_per_decision'] == 768
