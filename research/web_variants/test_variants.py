"""Physical checks for new interventions, independent of measured detection."""
import numpy as np
from qiskit import QuantumCircuit
from qiskit.quantum_info import Operator,Statevector

from research.web_variants.prepare import encoding,mixed_trigger,qaoa


def test_qtrojan_active_encoding_is_input_independent_for_zero_initial_state():
    target=[.2,.4,.6,.8]
    first=encoding([.1,.2,.3,.4],target)
    second=encoding([1.2,-.7,.6,-.4],target)
    assert Statevector(first).equiv(Statevector(second))
    assert Statevector(first).equiv(Statevector(encoding(target)))
    assert not Statevector(encoding([.1,.2,.3,.4])).equiv(Statevector(encoding([1.2,-.7,.6,-.4])))


def test_mixed_control_permutation_is_invisible_to_fixed_preparations():
    clean=qaoa([(0,1),(1,2),(2,3)],[.3,.6],[.2,.4])
    changed=mixed_trigger(clean)
    assert not Operator(clean).equiv(Operator(changed))
    for state in ('0','1','+'):
        preparation=QuantumCircuit(4)
        if state=='1':preparation.x(range(4))
        if state=='+':preparation.h(range(4))
        assert Statevector(preparation.compose(clean)).equiv(Statevector(preparation.compose(changed)))
    # On an activating input, the two resulting states are orthogonal. This
    # witnesses a real operation change, even if these three probes miss it.
    initial=Statevector.from_int(4,16)
    assert abs(np.vdot(initial.evolve(clean).data,initial.evolve(changed).data))<1e-12


def test_global_phase_cannot_change_probability_or_fidelity():
    circuit=QuantumCircuit(1)
    circuit.ry(.73,0)
    state=Statevector(circuit)
    phased=Statevector(np.exp(.41j)*state.data)
    assert state.equiv(phased)
    assert np.allclose(state.probabilities(),phased.probabilities(),atol=1e-15,rtol=0)
