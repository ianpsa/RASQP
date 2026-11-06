use rasqp::{Circuit, Policy, Request, Response, SqliteLedger, inspect, protect};
use serde::Deserialize;
use serde_json::{Value, json};
use std::io::{self, Read};

#[derive(Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
enum Command {
    /// Show each local normalization rewrite and its gate offset.
    NormalizeTrace { circuit: Circuit },
    /// Compare paired detection outcomes from the offline study.
    Paired {
        baseline: Vec<bool>,
        candidate: Vec<bool>,
    },
    /// Correct a family of research p-values for multiple comparisons.
    Holm { pvalues: Vec<f64> },
    /// Compute local ideal/noisy simulator probabilities; this does not run RASP.
    Probabilities {
        circuit: Circuit,
        noise: Option<f64>,
    },
    /// Test full unitary equivalence in the bounded statevector oracle.
    Equivalent { left: Circuit, right: Circuit },
    /// Export the circuit for interoperability with an OpenQASM consumer.
    Qasm { circuit: Circuit },
    /// Localize normalized gate-sequence edits between two circuits.
    Align { left: Circuit, right: Circuit },
    /// Select an offline probe portfolio against supplied synthetic suspects.
    SelectProbes {
        reference: Circuit,
        suspects: Vec<Circuit>,
        size: usize,
    },
    /// Measure the largest deviation across an already collected probe batch.
    JointScore {
        verifier: rasqp::verification::JointVerifier,
        counts: Vec<Vec<u64>>,
    },
    /// Gate an already collected, signed batch before releasing its counts.
    ProtectBatch {
        entries: Vec<rasqp::guard::BatchEntry>,
        policy: Box<rasqp::guard::BatchPolicy>,
    },
    /// Compute the versioned digest that binds a submitted circuit to a request.
    Digest { circuit: Circuit },
    /// Apply conservative local circuit rewrites.
    Normalize { circuit: Circuit },
    /// Compare an approved and submitted circuit, including a resource budget.
    Inspect {
        approved: Circuit,
        submitted: Circuit,
        max_extra_gates: usize,
    },
    /// Score already collected counts against one calibrated distribution.
    Score {
        verifier: rasqp::verification::CalibratedVerifier,
        counts: Vec<u64>,
    },
    /// Gate an already collected signed response before releasing counts.
    Protect {
        approved: Circuit,
        submitted: Circuit,
        request: Box<Request>,
        response: Box<Response>,
        policy: Box<Policy>,
    },
}
fn run() -> Result<Value, Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        return Ok(json!({"version":env!("CARGO_PKG_VERSION"),"wire_version":1}));
    }
    if args.get(1).map(String::as_str) == Some("--help") {
        return Ok(
            json!({"usage":"rasqp [LEDGER_PATH] < command.json", "operations":["digest","normalize","normalize_trace","inspect","score","joint_score","protect","protect_batch","probabilities","equivalent","qasm","align","select_probes","paired","holm"],"key":"Protection operations read a hex key (>=32 bytes) from RASQP_KEY_HEX. Trusted caller supplies policy, references and expected requests."}),
        );
    }
    let mut bytes = Vec::new();
    io::stdin()
        .take(8 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > 8 * 1024 * 1024 {
        return Err("input exceeds 8 MiB".into());
    }
    Ok(match serde_json::from_slice::<Command>(&bytes)? {
        Command::NormalizeTrace { circuit } => serde_json::to_value(
            rasqp::research::normalization::Normalizer::default().run(&circuit)?,
        )?,
        Command::Paired {
            baseline,
            candidate,
        } => serde_json::to_value(rasqp::research::statistics::PairedComparison::evaluate(
            &baseline, &candidate,
        )?)?,
        Command::Holm { pvalues } => {
            json!({"adjusted":rasqp::research::statistics::holm(&pvalues)?})
        }
        Command::Probabilities { circuit, noise } => {
            use rasqp::domain::ProbabilityOracle;
            let probabilities = match noise {
                None => rasqp::adapters::simulator::Statevector.probabilities(&circuit)?,
                Some(p) => rasqp::adapters::simulator::DensityMatrix::new(
                    rasqp::adapters::simulator::NoiseModel::depolarizing(p)?,
                )?
                .probabilities(&circuit)?,
            };
            json!({"probabilities":probabilities})
        }
        Command::Equivalent { left, right } => {
            use rasqp::domain::EquivalenceOracle;
            json!({"equivalent":rasqp::adapters::simulator::Statevector.equivalent(&left,&right)?})
        }
        Command::Qasm { circuit } => {
            json!({"qasm":rasqp::adapters::qasm::OpenQasm::export(&circuit,true)?})
        }
        Command::Align { left, right } => {
            json!({"edits":rasqp::research::genome::Alignment::align(&left,&right)?})
        }
        Command::SelectProbes {
            reference,
            suspects,
            size,
        } => serde_json::to_value(
            rasqp::research::probes::ProbeSelector {
                oracle: &rasqp::adapters::simulator::Statevector,
            }
            .portfolio(&reference, &suspects, size)?,
        )?,
        Command::JointScore { verifier, counts } => {
            json!({"score":verifier.score(&counts)?,"rejected":verifier.rejects(&counts)?})
        }
        Command::ProtectBatch { entries, policy } => {
            let path = args
                .get(1)
                .ok_or("protect_batch requires an explicit persistent ledger path")?;
            if path == ":memory:" {
                return Err("CLI requires a persistent ledger".into());
            }
            let key = hex::decode(std::env::var("RASQP_KEY_HEX")?)?;
            let mut ledger = SqliteLedger::open(path)?;
            serde_json::to_value(rasqp::guard::protect_batch(
                &entries,
                &policy,
                &key,
                &mut ledger,
            )?)?
        }
        Command::Digest { circuit } => json!({"digest":circuit.digest()?}),
        Command::Normalize { circuit } => json!({"circuit":circuit.normalize()?}),
        Command::Inspect {
            approved,
            submitted,
            max_extra_gates,
        } => serde_json::to_value(inspect(&approved, &submitted, max_extra_gates)?)?,
        Command::Score { verifier, counts } => {
            json!({"score":verifier.score(&counts)?,"rejected":verifier.rejects(&counts)?})
        }
        Command::Protect {
            approved,
            submitted,
            request,
            response,
            policy,
        } => {
            let path = args
                .get(1)
                .ok_or("protect requires an explicit persistent ledger path")?;
            if path == ":memory:" {
                return Err("CLI requires a persistent ledger".into());
            }
            let key = hex::decode(std::env::var("RASQP_KEY_HEX")?)?;
            let mut ledger = SqliteLedger::open(path)?;
            serde_json::to_value(protect(
                &approved,
                &submitted,
                &request,
                &response,
                &policy,
                &key,
                &mut ledger,
            )?)?
        }
    })
}
fn main() {
    match run() {
        Ok(value) => println!("{value}"),
        Err(error) => {
            eprintln!(
                "{}",
                json!({"error":error.to_string(),"accepted_counts":null})
            );
            std::process::exit(2);
        }
    }
}
