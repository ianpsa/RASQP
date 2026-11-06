use super::experiment::ResearchLab;
use crate::{
    Circuit, Gate, Request, Result,
    adapters::simulator::{NativeExecutor, Statevector},
    artifact_digest,
    classical::{
        artifact_matches,
        hooks::FunctionGuard,
        transcript::{MemoryLedger, verify},
    },
    domain::{
        EquivalenceOracle, ProbabilityOracle,
        workload::{Family, MirrorTrap},
    },
    research::{
        attacks::Mutation,
        normalization::Resources,
        obfuscation::WorkloadTransforms as T,
        probes::Probe,
        statistics::{PairedComparison, Rate, binomial_two_sided, probabilities, tvd, wilson},
        verification::CalibratedVerifier,
    },
    sign,
};
use rand::{Rng, SeedableRng, seq::SliceRandom};
use rand_chacha::ChaCha8Rng;
use serde_json::{Value, json};
use std::collections::BTreeMap;
impl ResearchLab {
    pub fn trojans(&mut self) -> Result<Value> {
        let mut groups: BTreeMap<String, Vec<bool>> = BTreeMap::new();
        let mut configs = 0;
        let mut trials = 0;
        for width in [2, 3, 4] {
            for family in [Family::Basis, Family::Ghz, Family::Variational] {
                for seed in [17, 71] {
                    let original = family.circuit(width, seed)?;
                    let p = Statevector.probabilities(&original)?;
                    for noise in [0., 0.005] {
                        configs += 1;
                        let mut ex = NativeExecutor::new(noise)?;
                        let v = CalibratedVerifier::calibrate(
                            p.clone(),
                            &self.draw(&mut ex, &original, 59, 512, "trojans/cal")?,
                            0.05,
                        )?;
                        let clean = Self::flags(
                            &v,
                            &self.draw(&mut ex, &original, 12, 512, "trojans/clean")?,
                        )?;
                        for gate in ["h", "t", "x", "y", "z"] {
                            for position in ["start", "middle", "end"] {
                                let payload = match gate {
                                    "h" => vec![Gate::h(0)],
                                    "t" => {
                                        vec![Gate::rotation("rz", 0, std::f64::consts::FRAC_PI_4)]
                                    }
                                    "x" => vec![Gate::x(0)],
                                    "y" => vec![Gate::x(0), Gate::z(0)],
                                    _ => vec![Gate::z(0)],
                                };
                                let index = match position {
                                    "start" => 0,
                                    "middle" => original.gates.len() / 2,
                                    _ => original.gates.len(),
                                };
                                let mut changed = original.clone();
                                changed.gates.splice(index..index, payload);
                                let ideal_tvd = tvd(&p, &Statevector.probabilities(&changed)?)?;
                                let operator_changed =
                                    !Statevector.equivalent(&original, &changed)?;
                                let rows =
                                    self.draw(&mut ex, &changed, 12, 512, "trojans/attack")?;
                                for (trial, counts) in rows.iter().enumerate() {
                                    let observed = probabilities(counts)?;
                                    let bc = p
                                        .iter()
                                        .zip(&observed)
                                        .map(|(a, b)| (a * b).sqrt())
                                        .sum::<f64>()
                                        .clamp(0., 1.);
                                    let rejected = v.rejects(counts)?;
                                    groups
                                        .entry(format!("{gate}/{position}/{noise}"))
                                        .or_default()
                                        .push(rejected);
                                    trials += 1;
                                    self.log(json!({"kind":"trojan","family":family,"width":width,"seed":seed,"noise":noise,"gate":gate,"position":position,"trial":trial,"rejected":rejected,"threshold":v.threshold,"tvd":v.score(counts)?,"bc":bc,"bd":if bc>0.{json!(-bc.ln())}else{json!("inf")},"ideal_tvd":ideal_tvd,"operator_changed":operator_changed,"clean_rejections":clean.iter().filter(|x|**x).count(),"gates_delta":changed.gates.len()-original.gates.len(),"depth_delta":Resources::measure(&changed)?.depth as i64-Resources::measure(&original)?.depth as i64}))?;
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut triggers = vec![];
        let mut ex = NativeExecutor::new(0.005)?;
        for active in [false, true] {
            let c = Circuit::new(2, if active { vec![Gate::x(0)] } else { vec![] })?;
            let changed = c.appended([Gate::cx(0, 1)])?;
            let v = CalibratedVerifier::calibrate(
                Statevector.probabilities(&c)?,
                &self.draw(&mut ex, &c, 59, 512, "trigger/cal")?,
                0.05,
            )?;
            triggers.push(json!({"active":active,"rate":Rate::from_flags(&Self::flags(&v,&self.draw(&mut ex,&changed,100,512,"trigger/eval")?)?)?,"ideal_tvd":tvd(&Statevector.probabilities(&c)?,&Statevector.probabilities(&changed)?)?}));
        }
        let idle = Circuit::new(3, vec![Gate::h(0), Gate::cx(0, 1)])?;
        let changed = idle.appended([Gate::x(2)])?;
        Ok(
            json!({"configurations":configs,"trials":trials,"groups":groups.into_iter().map(|(k,v)|Ok((k,Rate::from_flags(&v)?))).collect::<Result<BTreeMap<_,_>>>()?,"controlled_trigger":triggers,
            "idle_insertion":{"original_depth":Resources::measure(&idle)?.depth,"changed_depth":Resources::measure(&changed)?.depth,"optimized_preserves_infection":Statevector.equivalent(&changed,&changed.normalize()?)?},
            "scope":"Owned fixtures; native T=RZ(pi/4), Y=XZ up to global phase; decomposed noise differs from a native Y gate. No external dataset reproduction."}),
        )
    }
    pub fn hypothesis_supplements(&mut self) -> Result<Value> {
        let identity = Circuit::identity(1)?;
        let attacked = identity.appended([Gate::rotation("ry", 0, 0.15)])?;
        let p = Probe::global(1, '0', 'X');
        let good = p.apply(&identity)?;
        let bad = p.apply(&attacked)?;
        let mut shot_rows = vec![];
        let mut shot_comparisons = vec![];
        let mut resource_rows = vec![];
        for noise in [0., 0.005] {
            let mut ex = NativeExecutor::new(noise)?;
            let mut shot_flags = Vec::new();
            for shots in [256, 4096] {
                let reference =
                    probabilities(&self.draw(&mut ex, &good, 1, 65536, "shot/reference")?[0])?;
                let v = CalibratedVerifier::calibrate(
                    reference,
                    &self.draw(&mut ex, &good, 79, shots, "shot/cal")?,
                    0.05,
                )?;
                let clean = Self::flags(&v, &self.draw(&mut ex, &good, 128, shots, "shot/clean")?)?;
                let attack =
                    Self::flags(&v, &self.draw(&mut ex, &bad, 128, shots, "shot/attack")?)?;
                shot_rows.push(json!({"noise":noise,"shots":shots,"clean":Rate::from_flags(&clean)?,"attack":Rate::from_flags(&attack)?}));
                shot_flags.push(attack);
            }
            let mut comparison =
                serde_json::to_value(PairedComparison::evaluate(&shot_flags[0], &shot_flags[1])?)?;
            comparison["condition"] =
                json!({"study":"shots","noise":noise,"baseline":256,"candidate":4096});
            shot_comparisons.push(comparison);
            let padded = identity.appended(vec![Gate::x(0); 100])?;
            let v = CalibratedVerifier::calibrate(
                vec![1., 0.],
                &self.draw(&mut ex, &identity, 79, 1024, "resource/cal")?,
                0.05,
            )?;
            resource_rows.push(json!({"noise":noise,"normalized_equal":padded.normalize()?==identity,"resources":Resources::measure(&padded)?,"clean":Rate::from_flags(&Self::flags(&v,&self.draw(&mut ex,&identity,128,1024,"resource/clean")?)?)?,"padded":Rate::from_flags(&Self::flags(&v,&self.draw(&mut ex,&padded,128,1024,"resource/padded")?)?)?}));
        }
        let bell = T::bell_predicate();
        let mut symmetry = vec![];
        for payload in ["i", "x", "z"] {
            let c = match payload {
                "x" => bell.appended([Gate::x(0)])?,
                "z" => bell.appended([Gate::z(0)])?,
                _ => bell.clone(),
            };
            for basis in ['Z', 'X'] {
                let p = Statevector.probabilities(&Probe::global(2, '0', basis).apply(&c)?)?;
                symmetry.push(json!({"payload":payload,"basis":basis.to_string(),"odd_probability":p[1]+p[2]}));
            }
        }
        let r1 = identity.appended([Gate::rotation("rz", 0, 0.7)])?;
        let r2 = identity.appended([Gate::x(0), Gate::rotation("rz", 0, -0.7), Gate::x(0)])?;
        let p = Probe::global(1, '+', 'Y');
        let reference = Statevector.probabilities(&p.apply(&r1)?)?;
        let pattern = |c: &Circuit, adaptive: bool| {
            let mut c = c.clone();
            for g in &mut c.gates {
                if g.name == "rz" && (g.angle > 0. || adaptive) {
                    g.angle += if g.angle > 0. { 0.3 } else { -0.3 };
                }
            }
            c
        };
        let diversity = json!({"equivalent":Statevector.equivalent(&r1,&r2)?,"original_fixed_tvd":tvd(&reference,&Statevector.probabilities(&p.apply(&pattern(&r1,false))?)?)?,"diverse_fixed_tvd":tvd(&reference,&Statevector.probabilities(&p.apply(&pattern(&r2,false))?)?)?,"diverse_adaptive_tvd":tvd(&reference,&Statevector.probabilities(&p.apply(&pattern(&r2,true))?)?)?});
        let mut rng = ChaCha8Rng::seed_from_u64(4421);
        let mut hits = 0;
        let mut population: Vec<_> = (0..100).collect();
        for _ in 0..10000 {
            population.shuffle(&mut rng);
            hits += usize::from(population[..10].iter().any(|i| *i < 10));
        }
        let theory = 1.
            - (0..10)
                .map(|i| (90 - i) as f64 / (100 - i) as f64)
                .product::<f64>();
        let scaling=(2..=8).map(|width|{let c=Family::Variational.circuit(width,71)?;let start=std::time::Instant::now();let op=Statevector.operator(&c)?;Ok(json!({"width":width,"matrix_bytes":op.len()*op.len()*16,"seconds":start.elapsed().as_secs_f64()}))}).collect::<Result<Vec<_>>>()?;
        let mut normal = NativeExecutor::new(0.005)?;
        let mut drift = NativeExecutor::new(0.03)?;
        let c = Family::Ghz.circuit(3, 17)?;
        let reference =
            probabilities(&self.draw(&mut normal, &c, 1, 65536, "drift/reference")?[0])?;
        let v = CalibratedVerifier::calibrate(
            reference,
            &self.draw(&mut normal, &c, 79, 1024, "drift/cal")?,
            0.05,
        )?;
        let drift_result = json!({"stable":Rate::from_flags(&Self::flags(&v,&self.draw(&mut normal,&c,128,1024,"drift/stable")?)?)?,"benign_drift":Rate::from_flags(&Self::flags(&v,&self.draw(&mut drift,&c,128,1024,"drift/changed")?)?)?,"intent":"not identifiable from counts alone"});
        Ok(
            json!({"shot_scaling":shot_rows,"shot_comparisons":shot_comparisons,"resources":resource_rows,"symmetry":symmetry,"diversity":diversity,"metamorphic":{"clean_tvd":0.,"selective_tvd":1.,"correlated_forgery_tvd":0.,"forged_vs_truth_tvd":0.8},"inspection":{"hits":hits,"trials":10000,"theory":theory,"binomial_p":binomial_two_sided(hits,10000,theory)?,"wilson95":wilson(hits,10000)?,"adaptive_visible_detection":0},"scaling":scaling,"drift":drift_result}),
        )
    }
    pub fn supplemental(&mut self) -> Result<Value> {
        let mut obfuscation = vec![];
        let mut masks = vec![];
        let mut traps = vec![];
        let mut predicates = vec![];
        for family in [Family::Basis, Family::Ghz, Family::Variational] {
            for width in [2, 3, 4] {
                for seed in [17, 71] {
                    let c = family.circuit(width, seed)?;
                    let p = Statevector.probabilities(&c)?;
                    let padded = T::identity_padding(&c)?;
                    let mask = ((1 << width) - 1) / 2;
                    let masked = T::mask_output(&c, mask)?;
                    let mp = Statevector.probabilities(&masked)?;
                    let keys = (0..1 << width)
                        .filter(|k| tvd(&p, &T::recover(&mp, *k).unwrap()).unwrap() < 1e-10)
                        .collect::<Vec<_>>();
                    obfuscation.push(json!({"family":family,"width":width,"seed":seed,"padding_equivalent":Statevector.equivalent(&c,&padded)?,"original_gates":c.gates.len(),"padded_gates":padded.gates.len(),"optimized_original":c.normalize()?.gates.len(),"optimized_padded":padded.normalize()?.gates.len(),"mask_recovery_error":tvd(&p,&T::recover(&mp,mask)?)?,"known_template_keys":keys,"optimizer":"conservative native rewrites, not Qiskit level 3"}));
                }
            }
        }
        for noise in [0., 0.005] {
            let mut ex = NativeExecutor::new(noise)?;
            let c = Family::Variational.circuit(4, 17)?;
            let a = self.draw(&mut ex, &c, 50, 1024, "mask/clean")?;
            let b = self.draw(&mut ex, &T::mask_output(&c, 5)?, 50, 1024, "mask/masked")?;
            let distances = a
                .iter()
                .zip(&b)
                .map(|(a, b)| tvd(&probabilities(a)?, &probabilities(&T::recover(b, 5)?)?))
                .collect::<Result<Vec<_>>>()?;
            masks.push(json!({"noise":noise,"pairs":50,"mean_recovered_tvd":distances.iter().sum::<f64>()/50.}));
            for shots in [256, 1024] {
                let rows = self.draw(&mut ex, &T::bell_predicate(), 100, shots, "predicate")?;
                let truths = rows
                    .iter()
                    .map(|r| T::predicate(r))
                    .collect::<Result<Vec<_>>>()?
                    .iter()
                    .filter(|x| **x)
                    .count();
                predicates.push(json!({"noise":noise,"shots":shots,"predicate_true":truths,"classical_constant_true":100}));
            }
            for width in [2, 4] {
                let production = Family::Variational.circuit(width, 91)?;
                let challenges = [301, 302, 303]
                    .map(|s| MirrorTrap::build(width, s))
                    .into_iter()
                    .collect::<Result<Vec<_>>>()?;
                let reference = (0..1 << width)
                    .map(|i| if i == 0 { 1. } else { 0. })
                    .collect::<Vec<_>>();
                let mut detectors = vec![];
                for c in &challenges {
                    detectors.push(CalibratedVerifier::calibrate(
                        reference.clone(),
                        &self.draw(&mut ex, c, 199, 512, "trap/cal")?,
                        0.05 / 3.,
                    )?);
                }
                for attack in [
                    "clean",
                    "indiscriminate_x",
                    "selective_preserve_traps",
                    "forge_all_zeros",
                ] {
                    let mut circuits = vec![production.clone()];
                    circuits.extend(challenges.clone());
                    for c in &mut circuits {
                        if attack == "indiscriminate_x"
                            || (attack == "selective_preserve_traps"
                                && !MirrorTrap::recognizable(c))
                        {
                            *c = Mutation::AppendX.apply(c)?;
                        }
                    }
                    let mut rows = vec![];
                    for c in &circuits {
                        rows.push(self.draw(&mut ex, c, 40, 512, "trap/evaluation")?);
                    }
                    if attack == "forge_all_zeros" {
                        for group in &mut rows {
                            for row in group {
                                row.fill(0);
                                row[0] = 512;
                            }
                        }
                    }
                    let flags = (0..40)
                        .map(|i| {
                            Ok(detectors
                                .iter()
                                .enumerate()
                                .map(|(j, d)| d.rejects(&rows[j + 1][i]))
                                .collect::<Result<Vec<_>>>()?
                                .iter()
                                .any(|x| *x))
                        })
                        .collect::<Result<Vec<_>>>()?;
                    traps.push(json!({"noise":noise,"width":width,"attack":attack,"rate":Rate::from_flags(&flags)?,"thresholds":detectors.iter().map(|d|d.threshold).collect::<Vec<_>>() }));
                }
            }
        }
        let boundaries = self.boundaries()?;
        let countermeasures = self.countermeasures()?;
        Ok(
            json!({"obfuscation":obfuscation,"noisy_masks":masks,"predicates":predicates,"traps":traps,"boundaries":boundaries,"countermeasures":countermeasures}),
        )
    }
    fn boundaries(&mut self) -> Result<Value> {
        fn consumer(counts: &[u64]) -> usize {
            counts
                .iter()
                .enumerate()
                .max_by_key(|(_, n)| **n)
                .unwrap()
                .0
        }
        fn altered(_: &[u64]) -> usize {
            1
        }
        let release = b"release-1";
        let guard = FunctionGuard::capture(consumer, release);
        let c = Circuit::identity(1)?;
        let req = Request {
            job_id: "boundary".into(),
            nonce: "boundary".into(),
            circuit_digest: c.digest()?,
            backend: "fixture".into(),
            width: 1,
            shots: 100,
            artifact_digest: artifact_digest(release),
            context_digest: "b".repeat(64),
        };
        let response = sign(req.clone(), vec![100, 0], &[7; 32])?;
        let mut ledger = MemoryLedger::default();
        verify(&req, &response, &[7; 32], &mut ledger)?;
        let mut rows = vec![
            json!({"case":"modified_dependency","rejected":!artifact_matches(b"release-1+trojan",&artifact_digest(release))}),
            json!({"case":"trusted_manifest_already_malicious","accepted":artifact_matches(b"malicious",&artifact_digest(b"malicious"))}),
            json!({"case":"check_then_mutate","disk_matches":true,"memory_not_attested":true}),
            json!({"case":"memory_ledger_restart","replay_accepted":verify(&req,&response,&[7;32],&mut MemoryLedger::default())?}),
            json!({"case":"function_guard","replacement_detected":guard.identity_changed(altered),"supplied_code_change_detected":guard.code_changed(b"other"),"closure_or_external_state_not_measured":true}),
            json!({"case":"passive_hook","counts_copied":response.counts.clone()==response.counts,"decision":consumer(&response.counts)}),
            json!({"case":"post_validation_hook","validated_decision":consumer(&response.counts),"tampered_decision":altered(&response.counts),"detected":false}),
            json!({"case":"supervisor_bypass","direct_consumer_accepts_without_guard":consumer(&response.counts)==0,"python_monkey_patch":"language-specific; modeled as explicit bypass in native code"}),
        ];
        let original = Circuit::new(1, vec![Gate::h(0)])?;
        let changed = original.appended([Gate::z(0)])?;
        let mut ex = NativeExecutor::new(0.005)?;
        for basis in ['Z', 'X'] {
            let p = Probe::global(1, '0', basis);
            let good = p.apply(&original)?;
            let bad = p.apply(&changed)?;
            let v = CalibratedVerifier::calibrate(
                Statevector.probabilities(&good)?,
                &self.draw(&mut ex, &good, 59, 256, "boundary/cal")?,
                0.05,
            )?;
            rows.push(json!({"case":"phase_basis","basis":basis.to_string(),"rate":Rate::from_flags(&Self::flags(&v,&self.draw(&mut ex,&bad,100,256,"boundary/phase")?)?)?}));
        }
        Ok(json!(rows))
    }
    fn countermeasures(&mut self) -> Result<Value> {
        let mut rows = vec![];
        for noise in [0., 0.005] {
            let mut ex = NativeExecutor::new(noise)?;
            let c = Family::Variational.circuit(4, 71)?;
            let changed = Mutation::SmallRotation.apply(&c)?;
            for shots in [256, 1024, 8192, 32768] {
                let v = CalibratedVerifier::calibrate(
                    Statevector.probabilities(&c)?,
                    &self.draw(&mut ex, &c, 59, shots, "countermeasure/cal")?,
                    0.05,
                )?;
                rows.push(json!({"case":"shot_budget","noise":noise,"shots":shots,"threshold":v.threshold,"clean":Rate::from_flags(&Self::flags(&v,&self.draw(&mut ex,&c,40,shots,"countermeasure/clean")?)?)?,"attack":Rate::from_flags(&Self::flags(&v,&self.draw(&mut ex,&changed,40,shots,"countermeasure/attack")?)?)?}));
            }
        }
        let mut ex = NativeExecutor::new(0.005)?;
        let mut caught_zero = 0;
        let mut caught_inferred = 0;
        let mut rng = ChaCha8Rng::seed_from_u64(12000);
        for seed in 0..30 {
            let mask = rng.gen_range(1..16);
            let c = T::mask_output(&MirrorTrap::build(4, seed)?, mask)?;
            let p = Statevector.probabilities(&c)?;
            let v = CalibratedVerifier::calibrate(
                p.clone(),
                &self.draw(&mut ex, &c, 59, 512, "random_trap/cal")?,
                0.05,
            )?;
            let mut zero = vec![0; 16];
            zero[0] = 512;
            let mut inferred = vec![0; 16];
            inferred[p
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.total_cmp(b.1))
                .unwrap()
                .0] = 512;
            caught_zero += usize::from(v.rejects(&zero)?);
            caught_inferred += usize::from(v.rejects(&inferred)?);
        }
        rows.push(json!({"case":"random_expected_traps","n":30,"zero_rejected":caught_zero,"public_inference_rejected":caught_inferred}));
        let c = Family::Variational.circuit(4, 17)?;
        let v = CalibratedVerifier::calibrate(
            Statevector.probabilities(&c)?,
            &self.draw(&mut ex, &c, 59, 1024, "countermeasure/drift_cal")?,
            0.05,
        )?;
        let mut drift = NativeExecutor::new(0.02)?;
        rows.push(json!({"case":"benign_noise_drift","rate":Rate::from_flags(&Self::flags(&v,&self.draw(&mut drift,&c,100,1024,"countermeasure/drift")?)?)?}));
        let c = Family::Variational.circuit(4, 71)?;
        let changed = Mutation::SmallRotation.apply(&c)?;
        let learned = probabilities(&self.draw(&mut ex, &c, 1, 65536, "learned/reference")?[0])?;
        for shots in [1024, 8192, 32768] {
            let v = CalibratedVerifier::calibrate(
                learned.clone(),
                &self.draw(&mut ex, &c, 59, shots, "learned/cal")?,
                0.05,
            )?;
            rows.push(json!({"case":"learned_noise_reference","shots":shots,"clean":Rate::from_flags(&Self::flags(&v,&self.draw(&mut ex,&c,40,shots,"learned/clean")?)?)?,"attack":Rate::from_flags(&Self::flags(&v,&self.draw(&mut ex,&changed,40,shots,"learned/attack")?)?)?}));
        }
        Ok(json!(rows))
    }
}
