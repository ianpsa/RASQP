//! Native experiment orchestration (no Python runtime).
use crate::{
    Circuit, Error, Request, Result,
    adapters::simulator::{NativeExecutor, Statevector},
    artifact_digest,
    classical::transcript::{MemoryLedger, verify},
    domain::{EquivalenceOracle, ProbabilityOracle, QuantumExecutor, workload::Family},
    research::{attacks::Mutation, statistics::Rate, verification::CalibratedVerifier},
    sign,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    time::Instant,
};

pub struct ResearchLab {
    pub(crate) trials: BufWriter<File>,
    summary: BufWriter<File>,
    report: BufWriter<File>,
    pub(crate) seed: u64,
    pub(crate) circuits: u64,
    pub(crate) shots: u64,
    pub(crate) jobs: u64,
}
impl ResearchLab {
    /// Open every writable output before applying the Linux experiment sandbox.
    pub fn open(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path)?;
        let protocol = super::protocol::ProtocolRegistry::native()?;
        std::fs::write(
            path.join("native_protocol.json"),
            serde_json::to_vec_pretty(&protocol)?,
        )?;
        Ok(Self {
            trials: BufWriter::new(File::create(path.join("native_trials.jsonl"))?),
            summary: BufWriter::new(File::create(path.join("native_summary.json"))?),
            report: BufWriter::new(File::create(path.join("native_report.md"))?),
            seed: 710000,
            circuits: 0,
            shots: 0,
            jobs: 0,
        })
    }
    pub(crate) fn log(&mut self, value: Value) -> Result<()> {
        serde_json::to_writer(&mut self.trials, &value)?;
        writeln!(self.trials)?;
        Ok(())
    }
    pub(crate) fn draw(
        &mut self,
        ex: &mut NativeExecutor,
        c: &Circuit,
        n: usize,
        shots: u64,
        label: &str,
    ) -> Result<Vec<Vec<u64>>> {
        self.seed += 1;
        let rows = ex.sample(&vec![c.clone(); n], shots, self.seed)?;
        self.jobs += 1;
        self.circuits += n as u64;
        self.shots += n as u64 * shots;
        self.log(json!({"kind":"samples","label":label,"seed":self.seed,"circuit":c,"shots":shots,"counts":rows}))?;
        Ok(rows)
    }
    pub(crate) fn flags(v: &CalibratedVerifier, rows: &[Vec<u64>]) -> Result<Vec<bool>> {
        rows.iter().map(|r| v.rejects(r)).collect()
    }
    pub fn run(mut self, mode: &str, sandboxed: bool) -> Result<Value> {
        let start = Instant::now();
        let mut out = json!({"version":env!("CARGO_PKG_VERSION"),"protocol":super::protocol::ProtocolRegistry::native()?,"protocol_sha256":artifact_digest(include_bytes!("../../docs/NATIVE_PROTOCOL_V1.json")),"sandboxed":sandboxed,"qpu_shots":0,"rng":"ChaCha8Rng 0.3","studies":{}});
        if ![
            "all",
            "batch",
            "hypotheses",
            "trojans",
            "extensions",
            "context",
        ]
        .contains(&mode)
        {
            return Err(Error::Invalid("lab mode"));
        }
        for name in ["batch", "hypotheses", "trojans", "extensions", "context"] {
            if mode != "all" && mode != name {
                continue;
            }
            eprintln!("Running native {name}");
            let value = match name {
                "batch" => self.batch()?,
                "hypotheses" => self.hypotheses()?,
                "trojans" => self.trojans()?,
                "extensions" => self.extension()?,
                "context" => self.context_extension()?,
                _ => unreachable!(),
            };
            out["studies"][name] = value;
        }
        if mode == "all" || mode == "batch" {
            out["studies"]["supplemental"] = self.supplemental()?;
        }
        out["cost"] = json!({"circuits":self.circuits,"shots":self.shots,"jobs":self.jobs,"seconds":start.elapsed().as_secs_f64(),"includes":"all logged native samples including calibration/reference; exact oracle computation not shots"});
        serde_json::to_writer_pretty(&mut self.summary, &out)?;
        self.summary.flush()?;
        self.trials.flush()?;
        self.report
            .write_all(super::report::ResearchReport::render(&out)?.as_bytes())?;
        self.report.flush()?;
        Ok(out)
    }
    pub fn batch(&mut self) -> Result<Value> {
        let mutations = [
            Mutation::AppendX,
            Mutation::AppendZ,
            Mutation::SmallRotation,
            Mutation::DropGate,
            Mutation::ParameterShift,
            Mutation::EquivalentRewrite,
        ];
        let names = [
            "clean",
            "append_x",
            "append_z",
            "small_rotation",
            "drop_gate",
            "parameter_shift",
            "equivalent_rewrite",
            "forged_uniform",
            "forged_zero",
            "biased_counts",
            "passive_hook",
            "network_tamper",
            "replay",
            "adaptive_distribution",
        ];
        let mut groups: BTreeMap<String, Vec<bool>> = BTreeMap::new();
        let mut config = 0;
        let mut decisions = 0;
        for width in [2, 3, 4] {
            for family in [Family::Basis, Family::Ghz, Family::Variational] {
                for seed in [17, 71] {
                    let c = family.circuit(width, seed)?;
                    let reference = Statevector.probabilities(&c)?;
                    for noise in [0., 0.005] {
                        let mut ex = NativeExecutor::new(noise)?;
                        for shots in [256, 1024] {
                            config += 1;
                            let clean_cal =
                                self.draw(&mut ex, &c, 59, shots, "batch/calibration")?;
                            let verifier =
                                CalibratedVerifier::calibrate(reference.clone(), &clean_cal, 0.05)?;
                            let baseline = self.draw(&mut ex, &c, 24, shots, "batch/clean")?;
                            for (index, name) in names.iter().enumerate() {
                                let changed = if (1..=6).contains(&index) {
                                    mutations[index - 1].apply(&c)?
                                } else {
                                    c.clone()
                                };
                                let mut rows = if (1..=6).contains(&index) {
                                    self.draw(&mut ex, &changed, 24, shots, name)?
                                } else {
                                    baseline.clone()
                                };
                                match *name {
                                    "forged_uniform" => {
                                        let p = vec![1. / (1 << width) as f64; 1 << width];
                                        let mut rng = ChaCha8Rng::seed_from_u64(self.seed + 1);
                                        rows = (0..24)
                                            .map(|_| {
                                                NativeExecutor::sample_distribution(
                                                    &p, shots, &mut rng,
                                                )
                                            })
                                            .collect::<Result<_>>()?;
                                    }
                                    "forged_zero" => {
                                        for row in &mut rows {
                                            row.fill(0);
                                            row[0] = shots;
                                        }
                                    }
                                    "biased_counts" => {
                                        for row in &mut rows {
                                            let donor = row
                                                .iter()
                                                .enumerate()
                                                .max_by_key(|(_, v)| **v)
                                                .unwrap()
                                                .0;
                                            let dest = (reference
                                                .iter()
                                                .enumerate()
                                                .max_by(|a, b| a.1.total_cmp(b.1))
                                                .unwrap()
                                                .0
                                                + 1)
                                                % row.len();
                                            let n = row[donor].min(shots / 10);
                                            row[donor] -= n;
                                            row[dest] += n;
                                        }
                                    }
                                    "adaptive_distribution" => {
                                        let target = reference
                                            .iter()
                                            .enumerate()
                                            .min_by(|a, b| a.1.total_cmp(b.1))
                                            .unwrap()
                                            .0;
                                        let mut chosen = baseline[0].clone();
                                        let mut best = -1.;
                                        for k in 0..=100 {
                                            let weight = k as f64 / 200.;
                                            let p: Vec<_> = reference
                                                .iter()
                                                .enumerate()
                                                .map(|(i, p)| {
                                                    (1. - weight) * p
                                                        + if i == target { weight } else { 0. }
                                                })
                                                .collect();
                                            let mut counts: Vec<_> = p
                                                .iter()
                                                .map(|p| (p * shots as f64).floor() as u64)
                                                .collect();
                                            let top = p
                                                .iter()
                                                .enumerate()
                                                .max_by(|a, b| a.1.total_cmp(b.1))
                                                .unwrap()
                                                .0;
                                            counts[top] += shots - counts.iter().sum::<u64>();
                                            let score = verifier.score(&counts)?;
                                            if !verifier.rejects(&counts)? && score > best {
                                                best = score;
                                                chosen = counts;
                                            }
                                        }
                                        rows = vec![chosen; 24];
                                    }
                                    _ => {}
                                }
                                let semantic = !Statevector.equivalent(&c, &changed)?;
                                for (trial, counts) in rows.iter().enumerate() {
                                    let req = Request {
                                        job_id: format!("{config}-{name}-{trial}"),
                                        nonce: format!("{config}-{name}-{trial}"),
                                        circuit_digest: c.digest()?,
                                        backend: "native-density".into(),
                                        width,
                                        shots,
                                        artifact_digest: "a".repeat(64),
                                        context_digest: "b".repeat(64),
                                    };
                                    let mut response = sign(req.clone(), counts.clone(), &[7; 32])?;
                                    let mut ledger = MemoryLedger::default();
                                    if *name == "network_tamper" {
                                        response.tag = "00".repeat(32);
                                    }
                                    if *name == "replay" {
                                        verify(&req, &response, &[7; 32], &mut ledger)?;
                                    }
                                    let classical_rejected =
                                        !verify(&req, &response, &[7; 32], &mut ledger)?;
                                    let rejected =
                                        classical_rejected || verifier.rejects(counts)?;
                                    groups
                                        .entry(format!("{name}/noise={noise}/shots={shots}"))
                                        .or_default()
                                        .push(rejected);
                                    decisions += 1;
                                    self.log(json!({"kind":"batch_decision","config":config,"family":family,"width":width,"circuit_seed":seed,"noise":noise,"shots":shots,"attack":name,"trial":trial,"counts":counts,"reference":reference,"threshold":verifier.threshold,"score":verifier.score(counts)?,"rejected":rejected,"classical_rejected":classical_rejected,"unitary_mismatch":semantic,"digest_mismatch":c!=changed}))?;
                                }
                            }
                        }
                    }
                }
            }
        }
        let groups = groups
            .into_iter()
            .map(|(name, flags)| Ok((name, Rate::from_flags(&flags)?)))
            .collect::<Result<BTreeMap<_, _>>>()?;
        Ok(
            json!({"configurations":config,"decisions":decisions,"groups":groups,"note":"Native RNG differs from archived Python run; conservative native optimizer is not Qiskit transpiler."}),
        )
    }
}
