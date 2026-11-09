//! Small educational experiments call the SDK directly; they are not frozen lab reruns.
use rasqp::{
    Circuit, Context, Gate, Policy, Request, Result,
    adapters::{
        qasm::OpenQasm,
        simulator::{DensityMatrix, NativeExecutor, NoiseModel, Statevector},
    },
    classical::transcript::MemoryLedger,
    domain::{EquivalenceOracle, ProbabilityOracle, QuantumExecutor, workload::Family},
    research::{
        genome::Alignment,
        normalization::Normalizer,
        probes::Probe,
        statistics::{Rate, probabilities},
    },
    verification::CalibratedVerifier,
};
use serde_json::{Value, json};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    pub demo: usize,
    pub probe: usize,
    pub noise: usize,
    pub shots: usize,
    pub seed: u64,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            demo: 0,
            probe: 0,
            noise: 0,
            shots: 1,
            seed: 710_000,
        }
    }
}
impl Config {
    pub fn noise_value(self) -> f64 {
        [0.0, 0.005, 0.02][self.noise]
    }
    pub fn shot_count(self) -> u64 {
        [256, 1024, 4096][self.shots]
    }
    pub fn probe_label(self) -> &'static str {
        ["0 / Z", "+ / X", "1 / Z", "i / Y"][self.probe]
    }
    fn probe_plan(self, width: u8) -> Probe {
        let (state, basis) = [('0', 'Z'), ('+', 'X'), ('1', 'Z'), ('i', 'Y')][self.probe];
        Probe::global(width, state, basis)
    }
}

pub struct DemoResult {
    pub reference: Vec<f64>,
    pub observed: Vec<f64>,
    pub counts: Vec<u64>,
    pub approved: Circuit,
    pub candidate: Circuit,
    pub threshold: f64,
    pub score: f64,
    pub alarm: bool,
    pub clean: Rate,
    pub changed: Rate,
    pub equivalent: bool,
    pub inspection: rasqp::Inspection,
    pub contracts: Option<Value>,
    pub runtime: Option<Value>,
    pub raw: String,
}

pub fn circuits(demo: usize) -> Result<(Circuit, Circuit)> {
    let approved = match demo {
        0 | 1 | 5..=8 => Family::Ghz.circuit(2, 17)?,
        3 => Circuit::identity(2)?,
        _ => Circuit::identity(1)?,
    };
    let gates = match demo {
        1 => vec![Gate::x(0)],
        2 => vec![Gate::z(0)],
        3 => vec![Gate::cx(0, 1)],
        4 => vec![Gate::rotation("ry", 0, 0.15)],
        5 => vec![Gate::h(0), Gate::h(0)],
        _ => vec![],
    };
    let candidate = approved.appended(gates)?;
    Ok((approved, candidate))
}

pub fn run(config: Config) -> Result<DemoResult> {
    let (approved, candidate) = circuits(config.demo)?;
    let probe = config.probe_plan(approved.width);
    let clean_circuit = probe.apply(&approved)?;
    let changed_circuit = probe.apply(&candidate)?;
    let noise = config.noise_value();
    let candidate_noise = noise + if config.demo == 6 { 0.03 } else { 0.0 };
    let reference =
        DensityMatrix::new(NoiseModel::depolarizing(noise)?)?.probabilities(&clean_circuit)?;
    let mut clean_executor = NativeExecutor::new(noise)?;
    let calibration = clean_executor.sample(
        &vec![clean_circuit.clone(); 79],
        config.shot_count(),
        config.seed,
    )?;
    let verifier = CalibratedVerifier::calibrate(reference.clone(), &calibration, 0.05)?;
    let clean_samples = clean_executor.sample(
        &vec![clean_circuit; 64],
        config.shot_count(),
        config.seed + 1,
    )?;
    let mut candidate_executor = NativeExecutor::new(candidate_noise)?;
    let samples = candidate_executor.sample(
        &vec![changed_circuit; 64],
        config.shot_count(),
        config.seed + 2,
    )?;
    let flags = |rows: &[Vec<u64>]| {
        rows.iter()
            .map(|c| verifier.rejects(c))
            .collect::<Result<Vec<_>>>()
    };
    let clean = Rate::from_flags(&flags(&clean_samples)?)?;
    let changed = Rate::from_flags(&flags(&samples)?)?;
    let counts = samples[0].clone();
    let observed = probabilities(&counts)?;
    let score = verifier.score(&counts)?;
    let alarm = verifier.rejects(&counts)?;
    let inspection = rasqp::inspect(&approved, &candidate, 4)?;
    let equivalent = Statevector.equivalent(&approved, &candidate)?;
    let contracts = if config.demo == 7 {
        Some(signed_contracts(
            &probe.apply(&approved)?,
            &counts,
            &verifier,
        )?)
    } else {
        None
    };
    let normalized = Normalizer::default().run(&candidate)?;
    let runtime = if config.demo == 8 {
        Some(runtime_contracts(
            &probe.apply(&approved)?,
            &verifier,
            config,
        )?)
    } else {
        None
    };
    let raw = serde_json::to_string_pretty(&json!({
        "kind":"educational_demo_not_frozen_protocol", "demo":config.demo,
        "seed_calibration":config.seed,"seed_clean":config.seed+1,"seed_candidate":config.seed+2,
        "noise":noise,"candidate_noise":candidate_noise,"shots_per_repetition":config.shot_count(),
        "probe":probe,"approved":approved,"candidate":candidate,
        "reference_kind":"exact density-matrix probabilities in approved noise context",
        "verifier":verifier,"first_counts":counts,"first_tvd":score,"first_alarm":alarm,
        "clean_alarm_rate":clean,"candidate_alarm_rate":changed,
        "calibration_counts":calibration,"clean_counts":clean_samples,"candidate_counts":samples,
        "structural_inspection":inspection,"ideal_operator_equivalent":equivalent,
        "normalization":normalized,"alignment":Alignment::align(&approved,&candidate)?,
        "candidate_digest":candidate.digest()?,"candidate_qasm":OpenQasm::export(&candidate,true)?,
        "signed_gate_contracts":contracts,
        "guarded_runtime_contracts":runtime,
    }))?;
    Ok(DemoResult {
        reference,
        observed,
        counts,
        approved,
        candidate,
        threshold: verifier.threshold,
        score,
        alarm,
        clean,
        changed,
        equivalent,
        inspection,
        contracts,
        runtime,
        raw,
    })
}

fn runtime_contracts(
    measured: &Circuit,
    verifier: &CalibratedVerifier,
    config: Config,
) -> Result<Value> {
    use rasqp::{
        application::runtime::{GuardedRuntime, QuantumBackend, RuntimePolicy},
        classical::artifact::ArtifactFingerprint,
    };
    struct SeededBackend {
        executor: NativeExecutor,
        seed: u64,
    }
    impl QuantumBackend for SeededBackend {
        fn backend_id(&self) -> &str {
            "tui-seeded-local-simulator"
        }
        fn execute(&mut self, request: &Request, circuit: &Circuit) -> Result<Vec<u64>> {
            self.executor
                .sample(std::slice::from_ref(circuit), request.shots, self.seed)?
                .pop()
                .ok_or(rasqp::Error::Invalid("missing educational runtime counts"))
        }
    }
    // The currently running executable is an existing read-only artifact. This
    // self-approved fixture demonstrates mechanics, not independent attestation.
    let artifact_path = std::env::current_exe()?;
    let artifact = ArtifactFingerprint::measure(&artifact_path)?;
    let backend = SeededBackend {
        executor: NativeExecutor::new(config.noise_value())?,
        seed: config.seed + 2,
    };
    let policy = RuntimePolicy {
        approved: measured.clone(),
        artifact_path,
        artifact,
        backend_id: backend.backend_id().into(),
        shots: config.shot_count(),
        max_extra_gates: 4,
        distribution: verifier.clone(),
        probes: None,
    };
    let mut runtime = GuardedRuntime::new(policy, backend, MemoryLedger::default())?;
    let clean = runtime.execute(measured)?;
    let changed = runtime.execute(&measured.appended([Gate::x(0)])?)?;
    Ok(json!({"approved_execution":clean,"visible_edit":changed,
        "artifact":"running executable, self-approved for this educational fixture",
        "optional_probe_jobs":"disabled; the selected preparation/basis is part of the approved workload"}))
}

fn signed_contracts(
    measured: &Circuit,
    counts: &[u64],
    verifier: &CalibratedVerifier,
) -> Result<Value> {
    // Sign the exact probe-wrapped recipe whose counts were actually sampled.
    let key = [0x42; 32]; // Public demonstration fixture, never a production secret.
    let context = Context {
        artifact_digest: "a".repeat(64),
        context_digest: "b".repeat(64),
    };
    let request = Request {
        job_id: "tui-local-fixture".into(),
        nonce: "tui-one-use-fixture".into(),
        circuit_digest: measured.digest()?,
        backend: "local-educational-signer".into(),
        width: measured.width,
        shots: counts.iter().sum(),
        artifact_digest: context.artifact_digest.clone(),
        context_digest: context.context_digest.clone(),
    };
    let response = rasqp::sign(request.clone(), counts.to_vec(), &key)?;
    let policy = Policy {
        max_extra_gates: 4,
        context,
        distribution: verifier.clone(),
    };
    let mut ledger = MemoryLedger::default();
    let first = rasqp::protect(
        measured,
        measured,
        &request,
        &response,
        &policy,
        &key,
        &mut ledger,
    )?;
    let replay = rasqp::protect(
        measured,
        measured,
        &request,
        &response,
        &policy,
        &key,
        &mut ledger,
    )?;
    let mut tampered = response.clone();
    let from = tampered
        .counts
        .iter()
        .position(|n| *n > 0)
        .expect("nonempty sampled counts");
    let to = (from + 1) % tampered.counts.len();
    tampered.counts[from] -= 1;
    tampered.counts[to] += 1;
    let altered = rasqp::protect(
        measured,
        measured,
        &request,
        &tampered,
        &policy,
        &key,
        &mut MemoryLedger::default(),
    )?;
    Ok(
        json!({"first_response":first,"replayed_response":replay,"altered_counts":altered,
        "ledger":"in-memory, reset for each educational run", "key":"public fixture, not for production"}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn phase_blind_spot_and_alternative_probe_are_real_sdk_results() {
        let a = run(Config {
            demo: 2,
            ..Config::default()
        })
        .unwrap();
        assert!(!a.equivalent);
        assert_eq!(a.changed.rejected, 0);
        let b = run(Config {
            demo: 2,
            probe: 1,
            ..Config::default()
        })
        .unwrap();
        assert_eq!(b.changed.rejected, 64);
        assert_eq!(b.score, 1.0);
    }
    #[test]
    fn dormant_control_activates_and_equivalent_padding_preserves_operator() {
        assert_eq!(
            run(Config {
                demo: 3,
                ..Config::default()
            })
            .unwrap()
            .changed
            .rejected,
            0
        );
        assert_eq!(
            run(Config {
                demo: 3,
                probe: 2,
                ..Config::default()
            })
            .unwrap()
            .changed
            .rejected,
            64
        );
        let benign = run(Config {
            demo: 5,
            ..Config::default()
        })
        .unwrap();
        assert!(benign.equivalent);
        assert_eq!(benign.inspection.decision, rasqp::Decision::Accept);
    }
    #[test]
    fn signed_fixture_withholds_replay_and_tampering_for_each_probe() {
        for probe in 0..4 {
            let r = run(Config {
                demo: 7,
                probe,
                ..Config::default()
            })
            .unwrap();
            let contracts = r.contracts.unwrap();
            for key in ["replayed_response", "altered_counts"] {
                assert_eq!(contracts[key]["decision"], "reject");
                assert!(contracts[key]["accepted_counts"].is_null());
            }
            assert_eq!(r.counts.iter().sum::<u64>(), 1024);
        }
    }
    #[test]
    fn runtime_fixture_executes_approved_recipe_and_withholds_visible_edit_before_submission() {
        let r = run(Config {
            demo: 8,
            ..Config::default()
        })
        .unwrap();
        let runtime = r.runtime.unwrap();
        assert_eq!(runtime["approved_execution"]["shots_submitted"], 1024);
        assert_eq!(runtime["visible_edit"]["decision"], "inconclusive");
        assert_eq!(runtime["visible_edit"]["shots_submitted"], 0);
        assert!(runtime["visible_edit"]["accepted_counts"].is_null());
        assert_eq!(runtime["approved_execution"]["decision"], "accept");
    }
    #[test]
    fn selection_never_changes_clean_calibration_and_all_demos_produce_valid_evidence() {
        let clean = run(Config {
            noise: 1,
            ..Config::default()
        })
        .unwrap();
        let mutated = run(Config {
            demo: 1,
            noise: 1,
            ..Config::default()
        })
        .unwrap();
        assert_eq!(clean.reference, mutated.reference);
        assert_eq!(clean.threshold, mutated.threshold);
        for demo in 0..8 {
            let r = run(Config {
                demo,
                noise: 2,
                shots: 0,
                ..Config::default()
            })
            .unwrap();
            assert_eq!(r.counts.iter().sum::<u64>(), 256);
            assert!((r.reference.iter().sum::<f64>() - 1.0).abs() < 1e-12);
            serde_json::from_str::<Value>(&r.raw).unwrap();
        }
    }
}
