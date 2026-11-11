//! Frozen external-dataset evaluation. Does not alter native-v1 or use attack
//! metadata for reference acquisition, calibration, or probe selection.
use flate2::{Compression, write::GzEncoder};
use rasqp::{
    Circuit, Gate, Result,
    adapters::simulator::{NativeExecutor, Statevector},
    artifact_digest,
    domain::{ProbabilityOracle, QuantumExecutor},
    research::{
        probes::Probe,
        statistics::{Rate, tvd},
        verification::{CalibratedVerifier, JointVerifier},
    },
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

#[derive(Deserialize)]
struct Sample {
    sample_id: String,
    label: u8,
    gate: String,
    mode: String,
    circuit: Circuit,
}
#[derive(Deserialize)]
struct Group {
    base_circuit_id: String,
    fold: usize,
    samples: Vec<Sample>,
}
struct Recorder {
    raw: GzEncoder<BufWriter<File>>,
    seed: u64,
    circuits: u64,
    shots: u64,
    calls: u64,
}
struct Observation<'a> {
    context: &'a Value,
    stage: &'a str,
    probe: usize,
    sample_id: &'a str,
}
impl Recorder {
    fn log(&mut self, value: Value) -> Result<()> {
        serde_json::to_writer(&mut self.raw, &value)?;
        writeln!(self.raw)?;
        Ok(())
    }
    fn draw(
        &mut self,
        executor: &mut NativeExecutor,
        circuit: &Circuit,
        repetitions: usize,
        shots: u64,
        observation: Observation<'_>,
    ) -> Result<Vec<Vec<u64>>> {
        let Observation {
            context,
            stage,
            probe,
            sample_id,
        } = observation;
        self.seed += 1;
        let counts = executor.sample(&vec![circuit.clone(); repetitions], shots, self.seed)?;
        self.circuits += repetitions as u64;
        self.shots += repetitions as u64 * shots;
        self.calls += 1;
        self.log(json!({"kind":"samples", "context":context,"stage":stage,"probe":probe,
            "sample_id":sample_id,"seed":self.seed,"circuit_digest":circuit.digest()?,"shots":shots,"counts":counts}))?;
        Ok(counts)
    }
}
fn decide(samples: &[Vec<Vec<u64>>], verifier: &JointVerifier) -> Result<Vec<bool>> {
    (0..samples[0].len())
        .map(|i| verifier.rejects(&samples.iter().map(|p| p[i].clone()).collect::<Vec<_>>()))
        .collect()
}
fn independent_or(
    samples: &[Vec<Vec<u64>>],
    verifiers: &[CalibratedVerifier],
) -> Result<Vec<bool>> {
    (0..samples[0].len())
        .map(|i| {
            let flags = samples
                .iter()
                .zip(verifiers)
                .map(|(p, v)| v.rejects(&p[i]))
                .collect::<Result<Vec<_>>>()?;
            Ok(flags.iter().any(|v| *v))
        })
        .collect()
}
fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if ![3, 4].contains(&args.len()) {
        return Err(rasqp::Error::Invalid(
            "usage: rasqp-external PREPARED_DIRECTORY OUTPUT_DIRECTORY [PROTOCOL_JSON]",
        ));
    }
    let input = Path::new(&args[1]);
    let output = Path::new(&args[2]);
    let protocol_storage = if args.len() == 4 {
        std::fs::read(&args[3])?
    } else {
        include_bytes!("../docs/WILDISLEY_PROTOCOL_V1.json").to_vec()
    };
    let protocol_bytes = protocol_storage.as_slice();
    let protocol: Value = serde_json::from_slice(protocol_bytes)?;
    let preparation: Value =
        serde_json::from_slice(&std::fs::read(input.join("preparation.json"))?)?;
    let input_bytes = std::fs::read(input.join("input.jsonl"))?;
    if preparation["protocol_sha256"] != artifact_digest(protocol_bytes)
        || preparation["input_sha256"] != artifact_digest(&input_bytes)
    {
        return Err(rasqp::Error::Invalid(
            "preparation/protocol/input hash mismatch",
        ));
    }
    let groups: Vec<Group> = BufReader::new(input_bytes.as_slice())
        .lines()
        .map(|line| Ok(serde_json::from_str(&line?)?))
        .collect::<Result<_>>()?;
    // The original paired-dataset schema stays strict. A separately versioned
    // control study may contain several benign variants and no infected rows.
    let mixed_controls = protocol["sample_policy"] == "mixed_controls";
    for group in &groups {
        let valid = if mixed_controls {
            !group.samples.is_empty()
                && group.samples[0].label == 0
                && group.samples.iter().all(|s| s.label <= 1)
        } else {
            group.samples.len() == 11
                && group.samples[0].label == 0
                && group.samples[1..].iter().all(|s| s.label == 1)
        };
        if !valid {
            return Err(rasqp::Error::Invalid("external base group"));
        }
        for sample in &group.samples {
            sample.circuit.validate()?;
            if sample.circuit.width > protocol["max_qubits"].as_u64().unwrap() as u8
                || sample.circuit.gates.len()
                    > protocol["max_lowered_gates"].as_u64().unwrap() as usize
            {
                return Err(rasqp::Error::Invalid("external resource bound"));
            }
        }
    }
    if output.exists() && output.read_dir()?.next().is_some() {
        return Err(rasqp::Error::Invalid("output directory must be fresh"));
    }
    std::fs::create_dir_all(output)?;
    let summary_file = File::create(output.join("native_summary.json"))?;
    let raw = File::create(output.join("native_trials.jsonl.gz"))?;
    let protocol_path = output.join("protocol.json");
    std::fs::write(protocol_path, protocol_bytes)?;
    // Outputs are open before Linux filtering. No network or upstream code execution.
    rasqp::lab::sandbox::restrict()?;
    let mut recorder = Recorder {
        raw: GzEncoder::new(BufWriter::new(raw), Compression::fast()),
        seed: protocol["seed"].as_u64().unwrap(),
        circuits: 0,
        shots: 0,
        calls: 0,
    };
    let cal = protocol["calibration_repetitions"].as_u64().unwrap() as usize;
    let reps = protocol["evaluation_repetitions"].as_u64().unwrap() as usize;
    let shots = protocol["shots_per_decision"].as_u64().unwrap();
    let ref_shots = protocol["reference_shots_per_probe"].as_u64().unwrap();
    let alpha = protocol["alpha"].as_f64().unwrap();
    let mut outcomes = Vec::new();
    for (group_index, group) in groups.iter().enumerate() {
        let golden = &group.samples[0];
        let width = golden.circuit.width;
        let probes = [('0', 'Z'), ('1', 'Z'), ('+', 'X')].map(|(s, b)| Probe::global(width, s, b));
        let ideal_clean = probes
            .iter()
            .map(|p| Statevector.probabilities(&p.apply(&golden.circuit)?))
            .collect::<Result<Vec<_>>>()?;
        for noise in protocol["noise"].as_array().unwrap() {
            let noise = noise.as_f64().unwrap();
            let context = json!({"base_circuit_id":group.base_circuit_id,"fold":group.fold,"noise":noise,
                "raw_approved_digest":golden.circuit.digest()?,"width":width});
            let mut executor = NativeExecutor::new(noise)?;
            let mut references = Vec::new();
            let mut calibration = Vec::new();
            for (i, probe) in probes.iter().enumerate() {
                let circuit = probe.apply(&golden.circuit)?;
                let counts = recorder.draw(
                    &mut executor,
                    &circuit,
                    1,
                    ref_shots,
                    Observation {
                        context: &context,
                        stage: "reference",
                        probe: i,
                        sample_id: &golden.sample_id,
                    },
                )?;
                references.push(rasqp::research::statistics::probabilities(&counts[0])?);
                calibration.push(recorder.draw(
                    &mut executor,
                    &circuit,
                    cal,
                    shots / 3,
                    Observation {
                        context: &context,
                        stage: "portfolio_calibration",
                        probe: i,
                        sample_id: &golden.sample_id,
                    },
                )?);
            }
            let batches = (0..cal)
                .map(|i| calibration.iter().map(|p| p[i].clone()).collect())
                .collect::<Vec<Vec<Vec<u64>>>>();
            let joint = JointVerifier::calibrate(references.clone(), &batches, alpha)?;
            let individual = references
                .iter()
                .zip(&calibration)
                .map(|(r, c)| CalibratedVerifier::calibrate(r.clone(), c, alpha))
                .collect::<Result<Vec<_>>>()?;
            let corrected = references
                .iter()
                .zip(&calibration)
                .map(|(r, c)| CalibratedVerifier::calibrate(r.clone(), c, alpha / 3.))
                .collect::<Result<Vec<_>>>()?;
            let fixed_cal = recorder.draw(
                &mut executor,
                &probes[0].apply(&golden.circuit)?,
                cal,
                shots,
                Observation {
                    context: &context,
                    stage: "fixed_calibration",
                    probe: 0,
                    sample_id: &golden.sample_id,
                },
            )?;
            let fixed = CalibratedVerifier::calibrate(references[0].clone(), &fixed_cal, alpha)?;
            recorder.log(
                json!({"kind":"calibration","context":context,"references":references,
                "joint_threshold":joint.threshold,"fixed_threshold":fixed.threshold,
                "individual_thresholds":individual.iter().map(|v|v.threshold).collect::<Vec<_>>(),
                "bonferroni_thresholds":corrected.iter().map(|v|v.threshold).collect::<Vec<_>>() }),
            )?;
            for case_index in 0..=group.samples.len() {
                let (sample, circuit, case) = if case_index < group.samples.len() {
                    let sample = &group.samples[case_index];
                    (
                        sample,
                        sample.circuit.clone(),
                        if sample.label == 0 {
                            "clean"
                        } else {
                            "infected"
                        },
                    )
                } else {
                    (
                        golden,
                        golden
                            .circuit
                            .appended([Gate::x(width - 1), Gate::x(width - 1)])?,
                        "equivalent_x_pair",
                    )
                };
                let mut observed = Vec::new();
                let mut ideal_distances = Vec::new();
                for (i, probe) in probes.iter().enumerate() {
                    let measured = probe.apply(&circuit)?;
                    observed.push(recorder.draw(
                        &mut executor,
                        &measured,
                        reps,
                        shots / 3,
                        Observation {
                            context: &context,
                            stage: case,
                            probe: i,
                            sample_id: &sample.sample_id,
                        },
                    )?);
                    ideal_distances.push(tvd(
                        &ideal_clean[i],
                        &Statevector.probabilities(&measured)?,
                    )?);
                }
                let fixed_observed = recorder.draw(
                    &mut executor,
                    &probes[0].apply(&circuit)?,
                    reps,
                    shots,
                    Observation {
                        context: &context,
                        stage: &format!("{case}_fixed"),
                        probe: 0,
                        sample_id: &sample.sample_id,
                    },
                )?;
                let flags: BTreeMap<&str, Vec<bool>> = BTreeMap::from([
                    (
                        "fixed_0Z",
                        fixed_observed
                            .iter()
                            .map(|c| fixed.rejects(c))
                            .collect::<Result<_>>()?,
                    ),
                    ("portfolio_joint", decide(&observed, &joint)?),
                    (
                        "portfolio_naive_or",
                        independent_or(&observed, &individual)?,
                    ),
                    (
                        "portfolio_bonferroni_or",
                        independent_or(&observed, &corrected)?,
                    ),
                ]);
                let methods = flags
                    .iter()
                    .map(|(name, flags)| Ok((*name, Rate::from_flags(flags)?)))
                    .collect::<Result<BTreeMap<_, _>>>()?;
                let inspection = match rasqp::inspect(&golden.circuit, &circuit, 4096) {
                    Ok(value) => serde_json::to_value(value)?,
                    Err(error) => {
                        json!({"decision":"inconclusive", "status":"analysis_error_withheld",
                        "error":error.to_string(),"extra_gates":circuit.gates.len() as i64-golden.circuit.gates.len() as i64})
                    }
                };
                let row = json!({"context":context,"sample_id":sample.sample_id,"case":case,"gate":sample.gate,"mode":sample.mode,
                    "ideal_probe_tvd":ideal_distances,"raw_changed":golden.circuit.digest()?!=circuit.digest()?,
                    "inspection":inspection,"methods":methods});
                recorder.log(json!({"kind":"decision","outcome":row,"flags":flags}))?;
                outcomes.push(row);
            }
        }
        eprintln!(
            "External dataset: {}/{} base groups complete ({})",
            group_index + 1,
            groups.len(),
            group.base_circuit_id
        );
    }
    let result = json!({"version":protocol["version"],"protocol_sha256":artifact_digest(protocol_bytes),
        "input_sha256":preparation["input_sha256"],"preparation_sha256":artifact_digest(&serde_json::to_vec(&preparation)?),
        "dataset_commit":protocol["source"]["commit"],"eligible_bases":groups.len(),"sandboxed":true,
        "cost":{"circuits":recorder.circuits,"shots":recorder.shots,"executor_calls":recorder.calls,"qpu_shots":0},
        "outcomes":outcomes});
    serde_json::to_writer_pretty(BufWriter::new(summary_file), &result)?;
    recorder.raw.finish()?.flush()?;
    println!("{}", result["cost"]);
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
