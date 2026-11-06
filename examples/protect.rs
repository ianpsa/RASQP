use rasqp::{verification::CalibratedVerifier, *};
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let circuit = Circuit {
        width: 1,
        gates: vec![],
    };
    let context = Context {
        artifact_digest: artifact_digest(b"approved classical executable"),
        context_digest: artifact_digest(b"calibration-v1"),
    };
    let request = Request {
        job_id: "demo".into(),
        nonce: format!("demo-{:?}", std::time::SystemTime::now()),
        circuit_digest: circuit.digest()?,
        backend: "local-fixture".into(),
        width: 1,
        shots: 100,
        artifact_digest: context.artifact_digest.clone(),
        context_digest: context.context_digest.clone(),
    };
    let key = [7_u8; 32]; // Demo only. Supply independent random keys and nonces in applications.
    let response = sign(request.clone(), vec![100, 0], &key)?;
    let policy = Policy {
        max_extra_gates: 4,
        context,
        distribution: CalibratedVerifier {
            reference: vec![1., 0.],
            threshold: 0.05,
        },
    };
    let mut ledger = SqliteLedger::open("demo.sqlite")?;
    println!(
        "{}",
        serde_json::to_string_pretty(&protect(
            &circuit,
            &circuit,
            &request,
            &response,
            &policy,
            &key,
            &mut ledger
        )?)?
    );
    Ok(())
}
