use rasqp::{
    transcript::{NonceStore, verify},
    verification::CalibratedVerifier,
    *,
};
fn circuit() -> Circuit {
    Circuit {
        width: 1,
        gates: vec![],
    }
}
fn request() -> Request {
    Request {
        job_id: "job".into(),
        nonce: "nonce".into(),
        circuit_digest: circuit().digest().unwrap(),
        backend: "aer".into(),
        width: 1,
        shots: 100,
        artifact_digest: "a".repeat(64),
        context_digest: "b".repeat(64),
    }
}
fn policy() -> Policy {
    Policy {
        max_extra_gates: 4,
        context: Context {
            artifact_digest: "a".repeat(64),
            context_digest: "b".repeat(64),
        },
        distribution: CalibratedVerifier {
            reference: vec![1., 0.],
            threshold: 0.05,
        },
    }
}
const KEY: &[u8] = &[7; 32];
#[test]
fn joint_batch_never_partially_releases_and_consumes_rejected_responses() {
    use rasqp::guard::{BatchEntry, BatchPolicy, protect_batch};
    use rasqp::verification::JointVerifier;
    for attack in [false, true] {
        let c = circuit();
        let req = request();
        let mut second = req.clone();
        second.nonce = "second".into();
        let entries = vec![
            BatchEntry {
                approved: c.clone(),
                submitted: c.clone(),
                request: req.clone(),
                response: sign(req, vec![100, 0], KEY).unwrap(),
            },
            BatchEntry {
                approved: c.clone(),
                submitted: c,
                request: second.clone(),
                response: sign(
                    second,
                    if attack { vec![0, 100] } else { vec![100, 0] },
                    KEY,
                )
                .unwrap(),
            },
        ];
        let p = BatchPolicy {
            context: policy().context,
            max_extra_gates: 4,
            verifier: JointVerifier {
                references: vec![vec![1., 0.]; 2],
                threshold: 0.05,
            },
        };
        let mut ledger = SqliteLedger::open(":memory:").unwrap();
        let out = protect_batch(&entries, &p, KEY, &mut ledger).unwrap();
        assert_eq!(out.accepted_counts.is_some(), !attack);
        assert!(
            protect_batch(&entries, &p, KEY, &mut ledger)
                .unwrap()
                .accepted_counts
                .is_none()
        );
    }
}
#[test]
fn joint_calibration_checks_order_and_dimensions() {
    use rasqp::verification::JointVerifier;
    let v = JointVerifier::calibrate(
        vec![vec![1., 0.], vec![0., 1.]],
        &vec![vec![vec![100, 0], vec![0, 100]]; 99],
        0.05,
    )
    .unwrap();
    assert_eq!(v.threshold, 0.);
    assert!(!v.rejects(&[vec![100, 0], vec![0, 100]]).unwrap());
    assert!(v.rejects(&[vec![0, 100], vec![100, 0]]).unwrap());
    assert!(v.rejects(&[vec![100, 0]]).is_err());
}
#[test]
fn normalization_has_a_work_budget() {
    let mut c = Circuit {
        width: 2,
        gates: Vec::new(),
    };
    for _ in 0..1024 {
        c.gates.push(Gate {
            name: "rz".into(),
            qubits: vec![1],
            angle: 0.1,
        });
        c.gates.push(Gate {
            name: "rx".into(),
            qubits: vec![0],
            angle: 0.1,
        });
    }
    assert!(c.normalize().is_err());
}
#[test]
fn replay_survives_restart_and_concurrency() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("ledger.db");
    let mut ledger = SqliteLedger::open(&path).unwrap();
    let req = request();
    let response = sign(req.clone(), vec![100, 0], KEY).unwrap();
    assert!(verify(&req, &response, KEY, &mut ledger).unwrap());
    drop(ledger);
    assert!(
        !verify(
            &req,
            &response,
            KEY,
            &mut SqliteLedger::open(&path).unwrap()
        )
        .unwrap()
    );
    let jobs: Vec<_> = (0..16)
        .map(|_| {
            let path = path.clone();
            std::thread::spawn(move || {
                SqliteLedger::open(path)
                    .unwrap()
                    .claim("another nonce")
                    .unwrap()
            })
        })
        .collect();
    assert_eq!(
        jobs.into_iter()
            .filter_map(|j| j.join().unwrap().then_some(()))
            .count(),
        1
    );
}
#[test]
fn invalid_auth_does_not_burn_nonce() {
    let mut ledger = SqliteLedger::open(":memory:").unwrap();
    let req = request();
    let response = sign(req.clone(), vec![100, 0], KEY).unwrap();
    let mut bad = response.clone();
    bad.tag = "00".repeat(32);
    assert!(!verify(&req, &bad, KEY, &mut ledger).unwrap());
    assert!(verify(&req, &response, KEY, &mut ledger).unwrap());
}
#[test]
fn binding_all_fields_and_counts() {
    let req = request();
    let response = sign(req.clone(), vec![100, 0], KEY).unwrap();
    for field in [
        "job_id",
        "nonce",
        "circuit_digest",
        "backend",
        "width",
        "shots",
        "artifact_digest",
        "context_digest",
    ] {
        let mut value = serde_json::to_value(&req).unwrap();
        value[field] = if ["width", "shots"].contains(&field) {
            serde_json::json!(3)
        } else {
            serde_json::json!("c".repeat(64))
        };
        let changed: Request = serde_json::from_value(value).unwrap();
        assert!(
            !verify(
                &changed,
                &response,
                KEY,
                &mut SqliteLedger::open(":memory:").unwrap()
            )
            .unwrap(),
            "{field}"
        );
    }
    let mut changed = response.clone();
    changed.counts = vec![0, 100];
    assert!(
        !verify(
            &req,
            &changed,
            KEY,
            &mut SqliteLedger::open(":memory:").unwrap()
        )
        .unwrap()
    );
}
#[test]
fn result_gate_quarantines_context_and_unknown_variants() {
    for kind in [
        "accept", "artifact", "context", "behavior", "unknown", "cost", "width",
    ] {
        let c = circuit();
        let mut candidate = c.clone();
        let mut req = request();
        let mut counts = vec![100, 0];
        match kind {
            "artifact" => req.artifact_digest = "c".repeat(64),
            "context" => req.context_digest = "c".repeat(64),
            "behavior" => counts = vec![0, 100],
            "unknown" => candidate.gates.push(Gate {
                name: "x".into(),
                qubits: vec![0],
                angle: 0.,
            }),
            "cost" => {
                candidate.gates = vec![
                    Gate {
                        name: "x".into(),
                        qubits: vec![0],
                        angle: 0.
                    };
                    100
                ]
            }
            "width" => {
                req.width = 2;
                counts = vec![100, 0, 0, 0];
            }
            _ => {}
        }
        req.circuit_digest = candidate.digest().unwrap();
        let response = sign(req.clone(), counts, KEY).unwrap();
        let result = protect(
            &c,
            &candidate,
            &req,
            &response,
            &policy(),
            KEY,
            &mut SqliteLedger::open(":memory:").unwrap(),
        )
        .unwrap();
        let expected = match kind {
            "accept" => Decision::Accept,
            "context" | "unknown" => Decision::Inconclusive,
            _ => Decision::Reject,
        };
        assert_eq!(result.decision, expected, "{kind}");
        assert_eq!(result.accepted_counts.is_some(), kind == "accept");
    }
}
#[test]
fn malformed_inputs_and_overflow_fail_closed() {
    assert!(sign(request(), vec![u64::MAX, 101], KEY).is_err());
    assert!(sign(request(), vec![100], KEY).is_err());
    assert!(sign(request(), vec![100, 0], b"short").is_err());
    for threshold in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
        assert!(
            CalibratedVerifier {
                reference: vec![1., 0.],
                threshold
            }
            .score(&[100, 0])
            .is_err()
        );
    }
    assert!(
        CalibratedVerifier {
            reference: vec![0.9, 0.9],
            threshold: 0.1
        }
        .score(&[100, 0])
        .is_err()
    );
    let mut c = circuit();
    c.gates.push(Gate {
        name: "x".into(),
        qubits: vec![0],
        angle: 1.,
    });
    assert!(c.digest().is_err());
}
#[test]
fn normalization_preserves_cost_and_unknown_semantics() {
    let c = circuit();
    let padded = Circuit {
        width: 1,
        gates: vec![
            Gate {
                name: "x".into(),
                qubits: vec![0],
                angle: 0.
            };
            100
        ],
    };
    assert_eq!(padded.normalize().unwrap(), c);
    assert_eq!(inspect(&c, &padded, 4).unwrap().decision, Decision::Reject);
    let a = Circuit {
        width: 1,
        gates: vec![
            Gate {
                name: "rz".into(),
                qubits: vec![0],
                angle: 0.2,
            },
            Gate {
                name: "rz".into(),
                qubits: vec![0],
                angle: 0.3,
            },
        ],
    };
    let b = Circuit {
        width: 1,
        gates: vec![Gate {
            name: "rz".into(),
            qubits: vec![0],
            angle: 0.5,
        }],
    };
    assert_eq!(inspect(&a, &b, 4).unwrap().decision, Decision::Inconclusive);
}
#[test]
fn conformal_small_calibration_and_strict_boundary() {
    let v = CalibratedVerifier::calibrate(vec![1., 0.], &[vec![99, 1]], 0.05).unwrap();
    assert_eq!(v.threshold, 1.0);
    let v = CalibratedVerifier {
        reference: vec![1., 0.],
        threshold: 0.05,
    };
    assert!(!v.rejects(&[95, 5]).unwrap());
    assert!(v.rejects(&[94, 6]).unwrap());
}
