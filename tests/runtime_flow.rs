use rasqp::application::runtime::{GuardedRuntime, ProbePlan, QuantumBackend, RuntimePolicy};
use rasqp::classical::{artifact::ArtifactFingerprint, transcript::MemoryLedger};
use rasqp::research::verification::{CalibratedVerifier, JointVerifier};
use rasqp::{Circuit, Decision, Gate, Request, Result};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

struct FakeBackend {
    id: String,
    responses: Vec<Vec<u64>>,
    requests: Arc<Mutex<Vec<Request>>>,
    on_call: Option<Box<dyn FnOnce()>>,
}
impl QuantumBackend for FakeBackend {
    fn backend_id(&self) -> &str {
        &self.id
    }
    fn execute(&mut self, request: &Request, circuit: &Circuit) -> Result<Vec<u64>> {
        assert_eq!(request.circuit_digest, circuit.digest()?);
        self.requests.lock().unwrap().push(request.clone());
        if let Some(action) = self.on_call.take() {
            action();
        }
        Ok(self.responses.remove(0))
    }
}
fn policy(path: &Path) -> RuntimePolicy {
    RuntimePolicy {
        approved: Circuit::identity(1).unwrap(),
        artifact_path: path.into(),
        artifact: ArtifactFingerprint::measure(path).unwrap(),
        backend_id: "test-backend".into(),
        shots: 100,
        max_extra_gates: 0,
        distribution: CalibratedVerifier {
            reference: vec![1.0, 0.0],
            threshold: 0.05,
        },
        probes: Some(ProbePlan {
            circuits: vec![Circuit::new(1, vec![Gate::h(0)]).unwrap()],
            shots: 100,
            verifier: JointVerifier {
                references: vec![vec![0.5, 0.5]],
                threshold: 0.1,
            },
        }),
    }
}
fn backend(responses: Vec<Vec<u64>>, requests: Arc<Mutex<Vec<Request>>>) -> FakeBackend {
    FakeBackend {
        id: "test-backend".into(),
        responses,
        requests,
        on_call: None,
    }
}

#[test]
fn guard_releases_counts_only_after_workload_and_probe_pass() {
    let dir = tempfile::tempdir().unwrap();
    let artifact = dir.path().join("binary");
    std::fs::write(&artifact, b"trusted").unwrap();
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut runtime = GuardedRuntime::new(
        policy(&artifact),
        backend(vec![vec![100, 0], vec![52, 48]], log.clone()),
        MemoryLedger::default(),
    )
    .unwrap();
    let outcome = runtime.execute(&Circuit::identity(1).unwrap()).unwrap();
    assert_eq!(outcome.decision, Decision::Accept);
    assert_eq!(outcome.accepted_counts, Some(vec![100, 0]));
    assert_eq!(outcome.shots_submitted, 200);
    let requests = log.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert_ne!(requests[0].nonce, requests[1].nonce);
    assert_eq!(requests[0].context_digest, requests[1].context_digest);
}

#[test]
fn guard_blocks_preflight_and_measurement_violations() {
    let dir = tempfile::tempdir().unwrap();
    let artifact = dir.path().join("binary");
    std::fs::write(&artifact, b"trusted").unwrap();
    let approved = Circuit::identity(1).unwrap();
    for (responses, expected, calls) in [
        (vec![vec![90, 10]], Decision::Reject, 1),
        (vec![vec![100, 0], vec![100, 0]], Decision::Reject, 2),
    ] {
        let log = Arc::new(Mutex::new(Vec::new()));
        let mut runtime = GuardedRuntime::new(
            policy(&artifact),
            backend(responses, log.clone()),
            MemoryLedger::default(),
        )
        .unwrap();
        let outcome = runtime.execute(&approved).unwrap();
        assert_eq!(outcome.decision, expected);
        assert!(outcome.accepted_counts.is_none());
        assert_eq!(log.lock().unwrap().len(), calls);
    }
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut runtime = GuardedRuntime::new(
        policy(&artifact),
        backend(vec![], log.clone()),
        MemoryLedger::default(),
    )
    .unwrap();
    let outcome = runtime
        .execute(&approved.appended([Gate::x(0)]).unwrap())
        .unwrap();
    assert_ne!(outcome.decision, Decision::Accept);
    assert_eq!(log.lock().unwrap().len(), 0);
    std::fs::write(&artifact, b"tampered").unwrap();
    let outcome = runtime.execute(&approved).unwrap();
    assert_eq!(outcome.decision, Decision::Reject);
    assert_eq!(log.lock().unwrap().len(), 0);
}

#[test]
fn guard_rechecks_artifact_and_commits_policy_fields() {
    let dir = tempfile::tempdir().unwrap();
    let artifact = dir.path().join("binary");
    std::fs::write(&artifact, b"trusted").unwrap();
    let mut trusted = policy(&artifact);
    let initial = trusted.context_digest().unwrap();
    trusted.probes.as_mut().unwrap().verifier.threshold = 0.2;
    assert_ne!(initial, trusted.context_digest().unwrap());
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut executor = backend(vec![vec![100, 0], vec![50, 50]], log);
    let file = artifact.clone();
    executor.on_call = Some(Box::new(move || std::fs::write(file, b"tampered").unwrap()));
    let mut runtime =
        GuardedRuntime::new(policy(&artifact), executor, MemoryLedger::default()).unwrap();
    let outcome = runtime.execute(&Circuit::identity(1).unwrap()).unwrap();
    assert_eq!(outcome.decision, Decision::Reject);
    assert!(outcome.accepted_counts.is_none());
}
