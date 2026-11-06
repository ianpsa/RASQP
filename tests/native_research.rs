use rasqp::{
    adapters::{
        checks::PauliSandwich,
        compiler::{InjectGatePass, NormalizePass, PassManager},
        qasm::OpenQasm,
        simulator::{DensityMatrix, NativeExecutor, NoiseModel, Statevector},
    },
    domain::{
        EquivalenceOracle, ProbabilityOracle, QuantumExecutor,
        workload::{Family, MirrorTrap},
    },
    research::{
        genome::{Alignment, Edit},
        normalization::{Normalizer, Resources},
        obfuscation::WorkloadTransforms,
        policy::EvidenceFlags,
        probes::{Probe, ProbeSelector},
        statistics::{PairedComparison, binomial_tail, holm, wilson},
    },
    *,
};
#[test]
fn native_artifact_fingerprint_detects_changed_bytes() {
    use rasqp::classical::artifact::ArtifactFingerprint;
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("fixture.bin");
    std::fs::write(&path, b"approved machine code fixture").unwrap();
    let trusted = ArtifactFingerprint::measure(&path).unwrap();
    assert!(
        ArtifactFingerprint::measure(&path)
            .unwrap()
            .matches(&trusted)
    );
    std::fs::write(&path, b"changed machine code fixture").unwrap();
    assert!(
        !ArtifactFingerprint::measure(&path)
            .unwrap()
            .matches(&trusted)
    );
}
#[test]
fn bell_and_all_basis_gates_use_little_endian() {
    let bell = WorkloadTransforms::bell_predicate();
    assert_eq!(
        Statevector
            .probabilities(&bell)
            .unwrap()
            .iter()
            .map(|p| (*p * 2.).round() as u64)
            .collect::<Vec<_>>(),
        vec![1, 0, 0, 1]
    );
    for q in 0..4 {
        let c = Circuit::new(4, vec![Gate::x(q)]).unwrap();
        let p = Statevector.probabilities(&c).unwrap();
        assert_eq!(p[1 << q], 1.);
    }
}
#[test]
fn density_noise_has_correct_analytic_limits() {
    let c = Circuit::new(1, vec![Gate::x(0)]).unwrap();
    let noise = NoiseModel {
        single_qubit: 0.2,
        two_qubit: 0.,
        readout: 0.,
    };
    let p = DensityMatrix::new(noise)
        .unwrap()
        .probabilities(&c)
        .unwrap();
    assert!((p[0] - 0.1).abs() < 1e-12);
    assert!((p[1] - 0.9).abs() < 1e-12);
    let p = DensityMatrix::new(NoiseModel {
        readout: 0.1,
        ..noise
    })
    .unwrap()
    .probabilities(&c)
    .unwrap();
    assert!((p[0] - 0.18).abs() < 1e-12);
    let c = Circuit::new(2, vec![Gate::cx(0, 1)]).unwrap();
    let p = DensityMatrix::new(NoiseModel {
        single_qubit: 0.,
        two_qubit: 1.,
        readout: 0.,
    })
    .unwrap()
    .probabilities(&c)
    .unwrap();
    assert!(p.iter().all(|p| (*p - 0.25).abs() < 1e-12));
}
#[test]
fn simulator_inverse_and_operator_properties() {
    for width in 1..=4 {
        for seed in 0..10 {
            let c = Family::Variational.circuit(width, seed).unwrap();
            let inverse = c.appended(c.gates.iter().rev().map(Gate::inverse)).unwrap();
            assert!(
                Statevector
                    .equivalent(&inverse, &Circuit::identity(width).unwrap())
                    .unwrap()
            );
            let a = Statevector.probabilities(&c).unwrap();
            let b = DensityMatrix::new(NoiseModel::depolarizing(0.).unwrap())
                .unwrap()
                .probabilities(&c)
                .unwrap();
            assert!(a.iter().zip(b).all(|(a, b)| (a - b).abs() < 1e-12));
        }
    }
}
#[test]
fn random_sampling_is_seeded_and_preserves_shot_count() {
    let mut e = NativeExecutor::new(0.005).unwrap();
    let c = Family::Ghz.circuit(3, 1).unwrap();
    let a = e.sample(std::slice::from_ref(&c), 1024, 12).unwrap();
    let b = e.sample(&[c], 1024, 12).unwrap();
    assert_eq!(a, b);
    assert_eq!(a[0].iter().sum::<u64>(), 1024);
    assert_eq!(e.cost.shots, 2048);
}
#[test]
fn trace_offsets_and_alignment_localize_insertions() {
    let c = Circuit::new(1, vec![Gate::h(0), Gate::rotation("ry", 0, 0.3)]).unwrap();
    let b = Circuit::new(
        1,
        vec![
            Gate::h(0),
            Gate::rotation("rz", 0, 0.7),
            Gate::rotation("ry", 0, 0.3),
        ],
    )
    .unwrap();
    assert_eq!(
        Alignment::align(&c, &b).unwrap(),
        vec![Edit {
            operation: "insert".into(),
            source_offset: 1,
            target_offset: 1
        }]
    );
    let padded = c.appended([Gate::x(0), Gate::x(0)]).unwrap();
    let n = Normalizer::default().run(&padded).unwrap();
    assert!(Normalizer::default().replay(&padded, &n).unwrap());
    assert!(!n.trace.is_empty());
    assert_eq!(Resources::measure(&padded).unwrap().gates, 4);
}
#[test]
fn probes_and_portfolio_activate_phase_and_dormant_payloads() {
    let c = Circuit::identity(2).unwrap();
    let suspects = [
        c.appended([Gate::x(0)]).unwrap(),
        c.appended([Gate::z(0)]).unwrap(),
        c.appended([Gate::cx(0, 1)]).unwrap(),
    ];
    let selected = ProbeSelector {
        oracle: &Statevector,
    }
    .portfolio(&c, &suspects, 3)
    .unwrap();
    assert!(selected.objective > 0.99);
    assert_eq!(selected.probes.len(), 3);
    let phase = Probe::global(2, '+', 'X');
    let p = Statevector
        .probabilities(&phase.apply(&suspects[1]).unwrap())
        .unwrap();
    assert!(p[1] > 0.99);
}
#[test]
fn checks_detect_noncommuting_errors_and_keep_dynamic_flags() {
    for (payload, one, two) in [("i", 0., 0.), ("x", 0., 1.), ("y", 1., 1.), ("z", 1., 1.)] {
        for (layers, expected) in [(1, one), (2, two)] {
            let mut e = NativeExecutor::new(0.).unwrap();
            let c = PauliSandwich::new(payload, layers, true).unwrap();
            let rows = c.sample(&mut e, 1, 1024, 9).unwrap();
            assert_eq!(PauliSandwich::flag_probability(&rows[0]).unwrap(), expected);
            if expected == 1. {
                assert!(
                    rows[0]
                        .iter()
                        .enumerate()
                        .all(|(i, n)| i & 1 == 0 || *n == 0)
                );
            }
        }
    }
}
#[test]
fn transformations_preserve_only_declared_properties() {
    let c = Family::Variational.circuit(3, 17).unwrap();
    let padded = WorkloadTransforms::identity_padding(&c).unwrap();
    assert!(Statevector.equivalent(&c, &padded).unwrap());
    let mask = 5;
    let p = Statevector.probabilities(&c).unwrap();
    let masked = Statevector
        .probabilities(&WorkloadTransforms::mask_output(&c, mask).unwrap())
        .unwrap();
    let recovered = WorkloadTransforms::recover(&masked, mask).unwrap();
    assert!(p.iter().zip(recovered).all(|(a, b)| (a - b).abs() < 1e-12));
    assert!(MirrorTrap::recognizable(&MirrorTrap::build(3, 8).unwrap()));
}
#[test]
fn statistical_boundaries_and_policy_states() {
    let test = PairedComparison::evaluate(&[false; 10], &[true; 10]).unwrap();
    assert!((test.p_one_sided - 1. / 1024.).abs() < 1e-14);
    assert_eq!(holm(&[0.01, 0.04, 0.03]).unwrap(), vec![0.03, 0.06, 0.06]);
    assert!((binomial_tail(500, 1000, 0.5).unwrap() - 0.512612509089).abs() < 1e-9);
    assert_eq!(wilson(0, 0).unwrap(), [0., 1.]);
    assert_eq!(
        EvidenceFlags::default().assess().decision,
        Decision::Inconclusive
    );
    assert_eq!(
        EvidenceFlags {
            known: true,
            drift: true,
            ..Default::default()
        }
        .assess()
        .decision,
        Decision::Inconclusive
    );
}
#[test]
fn compiler_pass_and_export_are_explicit_and_validated() {
    let mut manager = PassManager::default();
    manager.add(InjectGatePass { angle: None });
    manager.add(InjectGatePass { angle: None });
    manager.add(NormalizePass);
    let c = Circuit::identity(2).unwrap();
    assert_eq!(manager.run(&c).unwrap(), c);
    assert!(
        OpenQasm::export(&Family::Ghz.circuit(2, 1).unwrap(), true)
            .unwrap()
            .contains("cx q[0],q[1];")
    );
    assert!(
        DensityMatrix::new(NoiseModel::depolarizing(0.).unwrap())
            .unwrap()
            .probabilities(&Circuit::identity(9).unwrap())
            .is_err()
    );
}
