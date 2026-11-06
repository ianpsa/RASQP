//! A local guarded run. Replace NativeBackend with an authenticated provider adapter
//! when deploying against a real quantum service.
use rasqp::adapters::runtime_backend::NativeBackend;
use rasqp::application::runtime::{GuardedRuntime, ProbePlan, QuantumBackend, RuntimePolicy};
use rasqp::classical::artifact::ArtifactFingerprint;
use rasqp::research::verification::{CalibratedVerifier, JointVerifier};
use rasqp::{Circuit, Decision, Gate, SqliteLedger};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let approved = Circuit::identity(1)?;
    let challenge = approved.appended([Gate::h(0)])?;
    let backend = NativeBackend::new(0.0)?;
    let path = std::env::current_exe()?;
    let policy = RuntimePolicy {
        approved: approved.clone(),
        artifact: ArtifactFingerprint::measure(&path)?,
        artifact_path: path,
        backend_id: backend.backend_id().to_owned(),
        shots: 512,
        max_extra_gates: 0,
        distribution: CalibratedVerifier {
            reference: vec![1.0, 0.0],
            threshold: 0.05,
        },
        probes: Some(ProbePlan {
            circuits: vec![challenge],
            shots: 512,
            verifier: JointVerifier {
                references: vec![vec![0.5, 0.5]],
                threshold: 0.1,
            },
        }),
    };
    let workdir = tempfile::tempdir()?;
    let ledger = SqliteLedger::open(workdir.path().join("requests.sqlite"))?;
    let mut runtime = GuardedRuntime::new(policy, backend, ledger)?;
    let outcome = runtime.execute(&approved)?;
    println!(
        "decision: {:?}, reason: {}",
        outcome.decision, outcome.reason
    );
    if outcome.decision == Decision::Accept {
        println!("accepted measurements: {:?}", outcome.accepted_counts);
    }
    Ok(())
}
