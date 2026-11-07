//! Adversarial audit counterexamples, separate from the frozen research protocol.
//! Passing these tests confirms the limitations described in the audit; it does
//! not establish successful attack detection. No archived results are modified.
use rasqp::{
    Circuit, Decision, Gate, Request, Result,
    adapters::simulator::{NativeExecutor, Statevector},
    application::runtime::{GuardedRuntime, ProbePlan, QuantumBackend, RuntimePolicy},
    classical::{artifact::ArtifactFingerprint, transcript::MemoryLedger},
    domain::{EquivalenceOracle, ProbabilityOracle, QuantumExecutor},
    research::{
        probes::Probe,
        statistics::tvd,
        verification::{CalibratedVerifier, JointVerifier},
    },
};
use std::collections::VecDeque;

#[test]
fn audit_swap_is_invisible_to_every_global_catalogue_probe() -> Result<()> {
    let approved = Circuit::identity(4)?;
    let swapped = approved.appended([Gate::cx(2, 3), Gate::cx(3, 2), Gate::cx(2, 3)])?;
    assert!(!Statevector.equivalent(&approved, &swapped)?);
    for probe in Probe::catalogue(4) {
        let clean = Statevector.probabilities(&probe.apply(&approved)?)?;
        let changed = Statevector.probabilities(&probe.apply(&swapped)?)?;
        assert!(tvd(&clean, &changed)? < 1e-12, "{probe:?}");
    }
    // Heterogeneous preparation witnesses a maximally different output.
    let mixed = Probe {
        input_state: "0010".into(),
        basis: "ZZZZ".into(),
    };
    let clean = Statevector.probabilities(&mixed.apply(&approved)?)?;
    let changed = Statevector.probabilities(&mixed.apply(&swapped)?)?;
    assert!((tvd(&clean, &changed)? - 1.0).abs() < 1e-12);
    // This is a behavioral blind spot for a hidden execution mutation. A visible
    // candidate would still be withheld by structural inspection.
    Ok(())
}

struct ReplayRows {
    rows: VecDeque<Vec<u64>>,
}
impl QuantumBackend for ReplayRows {
    fn backend_id(&self) -> &str {
        "audit-fixed-context"
    }
    fn execute(&mut self, _: &Request, _: &Circuit) -> Result<Vec<u64>> {
        Ok(self.rows.pop_front().expect("one precomputed row per job"))
    }
}

#[test]
fn audit_runtime_reuses_calibration_for_an_allowed_different_raw_circuit() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let path = directory.path().join("approved-artifact");
    std::fs::write(&path, b"approved")?;
    let approved = Circuit::identity(1)?;
    let padded = approved.appended(vec![Gate::x(0); 100])?;
    assert!(Statevector.equivalent(&approved, &padded)?);
    assert_eq!(
        rasqp::inspect(&approved, &padded, 100)?.decision,
        Decision::Accept
    );
    let mut executor = NativeExecutor::new(0.005)?;
    let calibration = executor.sample(&vec![approved.clone(); 127], 1024, 9_100_001)?;
    let reference = vec![0.995, 0.005]; // Identity: readout noise only.
    let verifier = CalibratedVerifier::calibrate(reference, &calibration, 0.05)?;
    let policy = RuntimePolicy {
        approved: approved.clone(),
        artifact_path: path.clone(),
        artifact: ArtifactFingerprint::measure(&path)?,
        backend_id: "audit-fixed-context".into(),
        shots: 1024,
        max_extra_gates: 100,
        distribution: verifier,
        probes: None,
    };
    let mut rates = Vec::new();
    for (index, candidate) in [&approved, &padded].into_iter().enumerate() {
        let rows = executor.sample(
            &vec![candidate.clone(); 256],
            1024,
            9_100_002 + index as u64,
        )?;
        let mut runtime = GuardedRuntime::new(
            policy.clone(),
            ReplayRows { rows: rows.into() },
            MemoryLedger::default(),
        )?;
        assert_eq!(runtime.policy().context_digest()?, policy.context_digest()?);
        let mut alarms = 0;
        for _ in 0..256 {
            alarms += usize::from(runtime.execute(candidate)?.decision == Decision::Reject);
        }
        rates.push(alarms);
    }
    println!(
        "audit raw-context alarms: identity={}/256; equivalent 100X={}/256",
        rates[0], rates[1]
    );
    assert!(rates[0] <= 25);
    assert_eq!(rates[1], 256);
    // Resource permission is not independent approval of a new noise context.
    // H44 calibrates each raw variant separately; the runtime does not enforce it.
    Ok(())
}

struct PublicOracleFabricator;
impl QuantumBackend for PublicOracleFabricator {
    fn backend_id(&self) -> &str {
        "audit-public-fabricator"
    }
    fn execute(&mut self, request: &Request, circuit: &Circuit) -> Result<Vec<u64>> {
        // Fabricate a plausible histogram analytically; execute no sampled job.
        let probabilities = Statevector.probabilities(circuit)?;
        let mut counts: Vec<u64> = probabilities
            .iter()
            .map(|p| (p * request.shots as f64).floor() as u64)
            .collect();
        let remainder = request.shots - counts.iter().sum::<u64>();
        let top = probabilities
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .unwrap()
            .0;
        counts[top] += remainder;
        Ok(counts)
    }
}

#[test]
fn audit_runtime_accepts_fabrication_without_sampled_workload_execution() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let path = directory.path().join("approved-artifact");
    std::fs::write(&path, b"approved")?;
    let approved = Circuit::new(2, vec![Gate::rotation("ry", 0, 0.71), Gate::cx(0, 1)])?;
    let circuits = [('0', 'Z'), ('1', 'Z'), ('+', 'X')]
        .into_iter()
        .map(|(input, basis)| Probe::global(2, input, basis).apply(&approved))
        .collect::<Result<Vec<_>>>()?;
    let references = circuits
        .iter()
        .map(|c| Statevector.probabilities(c))
        .collect::<Result<_>>()?;
    let policy = RuntimePolicy {
        approved: approved.clone(),
        artifact_path: path.clone(),
        artifact: ArtifactFingerprint::measure(&path)?,
        backend_id: "audit-public-fabricator".into(),
        shots: 1024,
        max_extra_gates: 0,
        distribution: CalibratedVerifier {
            reference: Statevector.probabilities(&approved)?,
            threshold: 0.02,
        },
        probes: Some(ProbePlan {
            circuits,
            shots: 256,
            verifier: JointVerifier {
                references,
                threshold: 0.02,
            },
        }),
    };
    let mut runtime = GuardedRuntime::new(policy, PublicOracleFabricator, MemoryLedger::default())?;
    let result = runtime.execute(&approved)?;
    assert_eq!(result.decision, Decision::Accept);
    assert!(result.accepted_counts.is_some());
    assert_eq!(result.shots_submitted, 1792); // Requested, not evidence of execution.
    Ok(())
}
