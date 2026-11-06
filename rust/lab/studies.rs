use super::experiment::ResearchLab;
use crate::{
    Circuit, Gate, Result,
    adapters::checks::PauliSandwich,
    adapters::simulator::{NativeExecutor, Statevector},
    domain::{EquivalenceOracle, ProbabilityOracle, workload::Family},
    research::{
        genome::{Alignment, Lineage, Stage, VariantCatalog},
        normalization::{Normalizer, Resources},
        probes::{Probe, ProbeSelector},
        statistics::{PairedComparison, Rate, conformal_threshold, holm, probabilities, tvd},
        verification::{CalibratedVerifier, JointVerifier},
    },
};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn evaluation(flags: &[bool], clean: &[bool]) -> Result<Value> {
    let a = Rate::from_flags(flags)?;
    let c = Rate::from_flags(clean)?;
    Ok(json!({"attack":a,"clean":c,"practical_pass":a.wilson95[0]>=0.90&&c.wilson95[1]<=0.10}))
}
fn add_comparison(
    comparisons: &mut Vec<Value>,
    label: Value,
    a: &[bool],
    b: &[bool],
) -> Result<()> {
    let mut value = serde_json::to_value(PairedComparison::evaluate(a, b)?)?;
    value["condition"] = label;
    comparisons.push(value);
    Ok(())
}
fn adjust(comparisons: &mut [Value]) -> Result<()> {
    let ps = comparisons
        .iter()
        .map(|c| c["p_one_sided"].as_f64().unwrap())
        .collect::<Vec<_>>();
    for (c, p) in comparisons.iter_mut().zip(holm(&ps)?) {
        c["p_holm"] = json!(p);
    }
    Ok(())
}
impl ResearchLab {
    pub fn extension(&mut self) -> Result<Value> {
        let training = Circuit::identity(2)?;
        let suspects = vec![
            training.appended([Gate::x(0)])?,
            training.appended([Gate::z(0)])?,
            training.appended([Gate::cx(0, 1)])?,
        ];
        let selector = ProbeSelector {
            oracle: &Statevector,
        };
        let selected = selector.portfolio(&training, &suspects, 3)?;
        let single = selector.single(&training, &suspects)?;
        let lift = |p: &Probe| {
            Probe::global(
                3,
                p.input_state.chars().next().unwrap(),
                p.basis.chars().next().unwrap(),
            )
        };
        let plans = vec![
            ("fixed_0Z", vec![Probe::global(3, '0', 'Z')]),
            ("trained_single", single.probes.iter().map(lift).collect()),
            (
                "portfolio_joint",
                selected.probes.iter().map(lift).collect(),
            ),
        ];
        let base = Circuit::identity(3)?;
        let mut conditions = vec![];
        let mut comparisons = vec![];
        for noise in [0., 0.005] {
            let mut ex = NativeExecutor::new(noise)?;
            let mut drift = NativeExecutor::new(0.04)?;
            let mut methods = BTreeMap::new();
            let mut flags = BTreeMap::new();
            for (name, probes) in &plans {
                let shots = 768 / probes.len() as u64;
                let mut refs = vec![];
                let mut calibration = vec![];
                for p in probes {
                    let c = p.apply(&base)?;
                    refs.push(Statevector.probabilities(&c)?);
                    calibration.push(self.draw(&mut ex, &c, 127, shots, "extension/cal")?);
                }
                let batches = (0..127)
                    .map(|i| calibration.iter().map(|p| p[i].clone()).collect())
                    .collect::<Vec<Vec<Vec<u64>>>>();
                let verifier = JointVerifier::calibrate(refs.clone(), &batches, 0.05)?;
                let individual = refs
                    .iter()
                    .zip(&calibration)
                    .map(|(r, s)| CalibratedVerifier::calibrate(r.clone(), s, 0.05))
                    .collect::<Result<Vec<_>>>()?;
                let mut cases = BTreeMap::new();
                let mut naive_cases = BTreeMap::new();
                for (case, payload) in extension_payloads(3, true) {
                    let c = base.appended(payload)?;
                    let source = if case == "benign_noise_drift" {
                        &mut drift
                    } else {
                        &mut ex
                    };
                    let samples = probes
                        .iter()
                        .map(|p| {
                            self.draw(
                                source,
                                &p.apply(&c)?,
                                128,
                                shots,
                                &format!("extension/{name}/{case}"),
                            )
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let mut decisions = vec![];
                    let mut naive = vec![];
                    for i in 0..128 {
                        let counts = samples.iter().map(|p| p[i].clone()).collect::<Vec<_>>();
                        decisions.push(verifier.rejects(&counts)?);
                        naive.push(
                            individual
                                .iter()
                                .zip(&counts)
                                .map(|(v, c)| v.rejects(c))
                                .collect::<Result<Vec<_>>>()?
                                .iter()
                                .any(|v| *v),
                        );
                    }
                    self.log(json!({"kind":"extension_decisions","noise":noise,"method":name,"case":case,"threshold":verifier.threshold,"references":refs,"decisions":decisions}))?;
                    cases.insert(case.clone(), Rate::from_flags(&decisions)?);
                    flags.insert((name.to_string(), case.clone()), decisions);
                    if *name == "portfolio_joint" {
                        naive_cases.insert(case.clone(), Rate::from_flags(&naive)?);
                        flags.insert(("portfolio_naive_or".into(), case), naive);
                    }
                }
                methods.insert(name.to_string(),json!({"threshold":verifier.threshold,"shots_per_decision":shots*probes.len() as u64,"cases":cases}));
                if *name == "portfolio_joint" {
                    methods.insert(
                        "portfolio_naive_or".into(),
                        json!({"cases":naive_cases,"shots_per_decision":768}),
                    );
                }
            }
            let strong = ["x", "z", "cx", "xz", "cxz"];
            let joint = &methods["portfolio_joint"]["cases"];
            let min_tpr = strong
                .iter()
                .map(|k| joint[*k]["rate"].as_f64().unwrap())
                .fold(1_f64, f64::min);
            let gains = ["fixed_0Z", "trained_single"].map(|m| {
                min_tpr
                    - strong
                        .iter()
                        .map(|k| methods[m]["cases"][*k]["rate"].as_f64().unwrap())
                        .fold(1_f64, f64::min)
            });
            let clean_ok = ["clean", "equivalent_x_pair"]
                .iter()
                .all(|k| joint[*k]["wilson95"][1].as_f64().unwrap() <= 0.10);
            let h41 = clean_ok
                && strong
                    .iter()
                    .all(|k| joint[*k]["wilson95"][0].as_f64().unwrap() >= 0.90)
                && gains.iter().all(|g| *g >= 0.20);
            let h42 = joint["clean"]["wilson95"][1].as_f64().unwrap() <= 0.10
                && strong.iter().all(|k| {
                    joint[*k]["rate"].as_f64().unwrap() + 0.05
                        >= methods["portfolio_naive_or"]["cases"][*k]["rate"]
                            .as_f64()
                            .unwrap()
                });
            for case in ["x", "z", "cx", "xz", "cxz", "weak_rotation"] {
                for baseline in ["fixed_0Z", "trained_single", "portfolio_naive_or"] {
                    add_comparison(
                        &mut comparisons,
                        json!({"noise":noise,"case":case,"baseline":baseline}),
                        &flags[&(baseline.into(), case.into())],
                        &flags[&("portfolio_joint".into(), case.into())],
                    )?;
                }
            }
            conditions.push(json!({"noise":noise,"methods":methods,"H41_pass":h41,"H42_pass":h42,"worst_strong_tpr":min_tpr,"gains":gains}));
        }
        adjust(&mut comparisons)?;
        Ok(
            json!({"training_selection":selected,"single_selection":single,"H41_pass":conditions.iter().all(|c|c["H41_pass"]==true),"H42_pass":conditions.iter().all(|c|c["H42_pass"]==true),"conditions":conditions,"comparisons":comparisons}),
        )
    }
    pub fn context_extension(&mut self) -> Result<Value> {
        let mut conditions = vec![];
        let mut comparisons = vec![];
        let probes = [
            Probe::global(4, '0', 'Z'),
            Probe::global(4, '1', 'Z'),
            Probe::global(4, '+', 'X'),
        ];
        for noise in [0., 0.005, 0.01] {
            for padded in [false, true] {
                let approved = Circuit::new(
                    4,
                    if padded {
                        vec![Gate::x(3), Gate::x(3)]
                    } else {
                        vec![]
                    },
                )?;
                let mut ex = NativeExecutor::new(noise)?;
                let mut drift = NativeExecutor::new(0.04)?;
                let mut ideal = vec![];
                let mut learned = vec![];
                let mut calibration = vec![];
                for p in &probes {
                    let c = p.apply(&approved)?;
                    ideal.push(Statevector.probabilities(&c)?);
                    learned.push(probabilities(
                        &self.draw(&mut ex, &c, 1, 8192, "context/reference")?[0],
                    )?);
                    calibration.push(self.draw(&mut ex, &c, 511, 256, "context/cal")?);
                }
                let batches = (0..511)
                    .map(|i| calibration.iter().map(|p| p[i].clone()).collect())
                    .collect::<Vec<Vec<Vec<u64>>>>();
                let mut verifiers = BTreeMap::new();
                let mut methods: BTreeMap<String, Value> = BTreeMap::new();
                let mut flags = BTreeMap::new();
                for (model, refs) in [("ideal", ideal), ("trusted_noisy", learned)] {
                    for alpha in [0.05, 0.01] {
                        let name = format!("{model}_{alpha}");
                        let verifier = JointVerifier::calibrate(refs.clone(), &batches, alpha)?;
                        methods.insert(
                            name.clone(),
                            json!({"threshold":verifier.threshold,"reference":refs,"cases":{}}),
                        );
                        verifiers.insert(name, verifier);
                    }
                }
                for (case, payload) in extension_payloads(4, false) {
                    let c = approved.appended(payload)?;
                    let source = if case == "benign_noise_drift" {
                        &mut drift
                    } else {
                        &mut ex
                    };
                    let samples = probes
                        .iter()
                        .map(|p| {
                            self.draw(source, &p.apply(&c)?, 256, 256, &format!("context/{case}"))
                        })
                        .collect::<Result<Vec<_>>>()?;
                    for (name, verifier) in &verifiers {
                        let decisions = (0..256)
                            .map(|i| {
                                verifier.rejects(
                                    &samples.iter().map(|p| p[i].clone()).collect::<Vec<_>>(),
                                )
                            })
                            .collect::<Result<Vec<_>>>()?;
                        methods.get_mut(name).unwrap()["cases"][&case] =
                            serde_json::to_value(Rate::from_flags(&decisions)?)?;
                        self.log(json!({"kind":"context_decisions","noise":noise,"padded":padded,"case":case,"method":name,"threshold":verifier.threshold,"decisions":decisions}))?;
                        flags.insert((name.clone(), case.clone()), decisions);
                    }
                }
                let candidate = "trusted_noisy_0.01";
                let cases = &methods[candidate]["cases"];
                let pass = cases["clean"]["wilson95"][1].as_f64().unwrap() <= 0.10
                    && ["x", "z", "cx", "xz", "cxz"]
                        .iter()
                        .all(|k| cases[*k]["wilson95"][0].as_f64().unwrap() >= 0.90);
                for case in ["x", "z", "cx", "xz", "cxz", "weak_rotation"] {
                    for baseline in verifiers.keys().filter(|k| *k != candidate) {
                        add_comparison(
                            &mut comparisons,
                            json!({"noise":noise,"padded":padded,"case":case,"baseline":baseline}),
                            &flags[&(baseline.clone(), case.into())],
                            &flags[&(candidate.into(), case.into())],
                        )?;
                    }
                }
                conditions
                    .push(json!({"noise":noise,"padded":padded,"methods":methods,"H44_pass":pass}));
            }
        }
        adjust(&mut comparisons)?;
        Ok(
            json!({"H44_pass":conditions.iter().all(|c|c["H44_pass"]==true),"conditions":conditions,"comparisons":comparisons,"warning":"Noisy-reference advantage must be judged against all three ablations; passing the candidate alone is insufficient."}),
        )
    }
    pub fn hypotheses(&mut self) -> Result<Value> {
        let mut normalization = vec![];
        let mut originals = vec![];
        let mut mutations = vec![];
        for family in [Family::Basis, Family::Ghz, Family::Variational] {
            for width in [2, 3, 4] {
                for seed in [17, 23, 41, 47, 53, 59, 61, 67] {
                    let c = family.circuit(width, seed)?;
                    let tails = [
                        vec![Gate::h(0), Gate::h(0)],
                        vec![Gate::x(0), Gate::x(0)],
                        vec![
                            Gate::rotation("rz", 0, 0.71),
                            Gate::rotation("rz", 0, -0.71),
                        ],
                        vec![Gate::h(0), Gate::z(0), Gate::h(0), Gate::x(0)],
                        vec![Gate::x(1), Gate::h(0), Gate::x(1), Gate::h(0)],
                    ];
                    for tail in tails {
                        let candidate = c.appended(tail)?;
                        let normalized = Normalizer::default().run(&candidate)?;
                        let row = json!({"family":family,"width":width,"seed":seed,"normalized_match":c.normalize()?==normalized.circuit,"operator_preserved":Statevector.equivalent(&candidate,&normalized.circuit)?,"idempotent":normalized.circuit.normalize()?==normalized.circuit,"rewrite_count":normalized.trace.len(),"resources":Resources::measure(&candidate)?});
                        self.log(json!({"kind":"normalization","value":row}))?;
                        normalization.push(row);
                    }
                    if width <= 3 {
                        let probes = [
                            Probe::global(width, '0', 'Z'),
                            Probe::global(width, '+', 'X'),
                            Probe::global(width, '1', 'Z'),
                        ];
                        let refs = probes
                            .iter()
                            .map(|p| Statevector.probabilities(&p.apply(&c)?))
                            .collect::<Result<Vec<_>>>()?;
                        for gate in [Gate::x(0), Gate::z(0), Gate::rotation("ry", 0, 0.15)] {
                            let mut positions = vec![0, c.gates.len() / 2, c.gates.len()];
                            positions.sort();
                            positions.dedup();
                            for pos in positions {
                                let mut changed = c.clone();
                                changed.gates.insert(pos, gate.clone());
                                let mut effects = vec![];
                                for (p, r) in probes.iter().zip(&refs) {
                                    effects.push(tvd(
                                        r,
                                        &Statevector.probabilities(&p.apply(&changed)?)?,
                                    )?);
                                }
                                let mut reverted = changed.clone();
                                reverted.gates.remove(pos);
                                let row = json!({"family":family,"width":width,"seed":seed,"gate":gate.name,"position":pos,"effects":effects,"harmful":effects.iter().any(|e|*e>0.05),"distance":Alignment::distance(&c,&changed)?,"reversal_ok":Statevector.equivalent(&c,&reverted)?});
                                self.log(json!({"kind":"mutation","value":row}))?;
                                mutations.push(row);
                            }
                        }
                    }
                    originals.push(c);
                }
            }
        }
        let mut holdouts = vec![];
        for family in ["basis", "ghz", "variational"] {
            let train = mutations
                .iter()
                .filter(|r| {
                    r["family"] != family && [17, 23, 41, 47].contains(&r["seed"].as_u64().unwrap())
                })
                .collect::<Vec<_>>();
            let test = mutations
                .iter()
                .filter(|r| {
                    r["family"] == family && [53, 59, 61, 67].contains(&r["seed"].as_u64().unwrap())
                })
                .collect::<Vec<_>>();
            let model = ["x", "z", "ry"]
                .map(|g| {
                    let rows = train.iter().filter(|r| r["gate"] == g).collect::<Vec<_>>();
                    (
                        g,
                        rows.iter().filter(|r| r["harmful"] == true).count() * 2 >= rows.len(),
                    )
                })
                .into_iter()
                .collect::<BTreeMap<_, _>>();
            let positive = test
                .iter()
                .filter(|r| r["harmful"] == true)
                .collect::<Vec<_>>();
            let negative = test
                .iter()
                .filter(|r| r["harmful"] == false)
                .collect::<Vec<_>>();
            let tpr = positive
                .iter()
                .filter(|r| model[r["gate"].as_str().unwrap()])
                .count() as f64
                / positive.len() as f64;
            let tnr = negative
                .iter()
                .filter(|r| !model[r["gate"].as_str().unwrap()])
                .count() as f64
                / negative.len() as f64;
            holdouts.push(json!({"family":family,"train_n":train.len(),"test_n":test.len(),"balanced_accuracy":(tpr+tnr)/2.,"model":model}));
        }
        let catalogue = VariantCatalog::new(
            &originals
                .iter()
                .take(12)
                .enumerate()
                .map(|(i, c)| (i.to_string(), vec![c.clone()]))
                .collect(),
        )?;
        let recognized = originals
            .iter()
            .take(12)
            .map(|c| {
                Ok(!catalogue
                    .classify(&c.appended([Gate::h(0), Gate::h(0)])?)?
                    .is_empty())
            })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .filter(|x| **x)
            .count();
        let stages = ["build", "compile", "submit"]
            .iter()
            .enumerate()
            .map(|(i, n)| Stage {
                name: n.to_string(),
                digest: i.to_string(),
            })
            .collect::<Vec<_>>();
        let mut located = vec![];
        for i in 0..3 {
            let mut changed = stages.clone();
            changed[i].digest = "tampered".into();
            located.push(Lineage::first_divergence(&stages, &changed));
        }
        let identity = Circuit::identity(1)?;
        let a = identity.appended([Gate::rotation("ry", 0, 0.15)])?;
        let ab = a.appended(a.gates.clone())?;
        let effects = [&identity, &a, &a, &ab]
            .iter()
            .map(|c| {
                tvd(
                    &Statevector.probabilities(&identity)?,
                    &Statevector.probabilities(c)?,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let split =
            identity.appended([Gate::rotation("rz", 0, 0.2), Gate::rotation("rz", 0, 0.3)])?;
        let merged = identity.appended([Gate::rotation("rz", 0, 0.5)])?;
        let fixtures = vec![
            ("phase", identity.clone(), identity.appended([Gate::z(0)])?),
            (
                "bitflip",
                identity.clone(),
                identity.appended([Gate::x(0)])?,
            ),
            ("small_rotation", identity.clone(), a),
            (
                "dormant_cx",
                Circuit::identity(2)?,
                Circuit::new(2, vec![Gate::cx(0, 1)])?,
            ),
            (
                "legit_hh",
                identity.clone(),
                identity.appended([Gate::h(0), Gate::h(0)])?,
            ),
            ("legit_rz_composition", split.clone(), merged.clone()),
        ];
        let mut probes_output = vec![];
        let mut comparisons = vec![];
        for noise in [0., 0.005] {
            let mut ex = NativeExecutor::new(noise)?;
            for (name, reference, changed) in &fixtures {
                let selected = ProbeSelector {
                    oracle: &Statevector,
                }
                .single(reference, std::slice::from_ref(changed))?
                .probes;
                let plans = vec![
                    ("fixed", vec![Probe::global(reference.width, '0', 'Z')]),
                    ("selected", selected),
                    (
                        "multiple",
                        vec![
                            Probe::global(reference.width, '0', 'Z'),
                            Probe::global(reference.width, '1', 'Z'),
                            Probe::global(reference.width, '+', 'X'),
                            Probe::global(reference.width, 'i', 'Y'),
                        ],
                    ),
                ];
                let mut methods = BTreeMap::new();
                let mut flags = BTreeMap::new();
                for (method, probes) in plans {
                    let shots = 1024 / probes.len() as u64;
                    let mut refs = vec![];
                    let mut calibration = vec![];
                    let mut clean = vec![];
                    let mut attack = vec![];
                    for p in &probes {
                        let c = p.apply(reference)?;
                        refs.push(probabilities(
                            &self.draw(&mut ex, &c, 1, 16384, "hypotheses/reference")?[0],
                        )?);
                        calibration.push(self.draw(&mut ex, &c, 79, shots, "hypotheses/cal")?);
                        clean.push(self.draw(&mut ex, &c, 128, shots, "hypotheses/clean")?);
                        attack.push(self.draw(
                            &mut ex,
                            &p.apply(changed)?,
                            128,
                            shots,
                            "hypotheses/attack",
                        )?);
                    }
                    let batches = (0..79)
                        .map(|i| calibration.iter().map(|p| p[i].clone()).collect())
                        .collect::<Vec<_>>();
                    let v = JointVerifier::calibrate(refs, &batches, 0.05)?;
                    let decide = |samples: &Vec<Vec<Vec<u64>>>| {
                        (0..128)
                            .map(|i| {
                                v.rejects(&samples.iter().map(|p| p[i].clone()).collect::<Vec<_>>())
                            })
                            .collect::<Result<Vec<_>>>()
                    };
                    let good = decide(&clean)?;
                    let bad = decide(&attack)?;
                    methods.insert(method,json!({"rates":evaluation(&bad,&good)?,"threshold":v.threshold,"probes":probes,"reference_shots":16384*v.references.len()}));
                    flags.insert(method, bad);
                }
                for baseline in ["fixed", "multiple"] {
                    add_comparison(
                        &mut comparisons,
                        json!({"name":name,"noise":noise,"baseline":baseline}),
                        &flags[baseline],
                        &flags["selected"],
                    )?;
                }
                probes_output.push(json!({"fixture":name,"noise":noise,"methods":methods,"structural_unknown":reference.normalize()?!=changed.normalize()?,"exact_operator_changed":!Statevector.equivalent(reference,changed)?,"combined_rejections":if reference.normalize()?!=changed.normalize()?{128}else{flags["selected"].iter().filter(|x|**x).count()}}));
            }
        }
        let checks = self.checks_study()?;
        let supplements = self.hypothesis_supplements()?;
        comparisons.extend(
            supplements["shot_comparisons"]
                .as_array()
                .unwrap()
                .iter()
                .cloned(),
        );
        adjust(&mut comparisons)?;
        Ok(
            json!({"normalization":{"n":normalization.len(),"matches":normalization.iter().filter(|r|r["normalized_match"]==true).count(),"operator_preserved":normalization.iter().filter(|r|r["operator_preserved"]==true).count(),"incomplete_counterexample":Statevector.equivalent(&split,&merged)?&&split.normalize()?!=merged.normalize()?},"genome":{"recognized":recognized,"located":located,"dependencies":Alignment::dependencies(&Family::Ghz.circuit(3,1)?)?},"mutations":{"n":mutations.len(),"reversals":mutations.iter().filter(|r|r["reversal_ok"]==true).count(),"holdouts":holdouts},"factorial":{"effects":effects,"interaction":effects[3]-effects[1]-effects[2]+effects[0]},"probes":probes_output,"comparisons":comparisons,"checks":checks,"supplements":supplements,"deferred":{"H33":"private watermark not implemented","H35":"quantum authentication protocol not implemented","H36":"not applicable to classical API","H37":"quantum advantage not demonstrated"}}),
        )
    }
    fn checks_study(&mut self) -> Result<Value> {
        let mut rows = vec![];
        for noise in [0., 0.005] {
            let mut ex = NativeExecutor::new(noise)?;
            for layers in [1, 2] {
                let clean = PauliSandwich::new("i", layers, false)?;
                let calibration = self.draw(&mut ex, &clean.circuit, 79, 1024, "checks/cal")?;
                let scores = calibration
                    .iter()
                    .map(|c| PauliSandwich::flag_probability(c))
                    .collect::<Result<Vec<_>>>()?;
                let threshold = conformal_threshold(&scores, 0.05)?;
                let good = self
                    .draw(&mut ex, &clean.circuit, 128, 1024, "checks/clean")?
                    .iter()
                    .map(|c| Ok(PauliSandwich::flag_probability(c)? > threshold + 1e-12))
                    .collect::<Result<Vec<_>>>()?;
                for payload in ["i", "x", "y", "z"] {
                    let c = PauliSandwich::new(payload, layers, false)?;
                    let bad = self
                        .draw(&mut ex, &c.circuit, 128, 1024, "checks/payload")?
                        .iter()
                        .map(|c| Ok(PauliSandwich::flag_probability(c)? > threshold + 1e-12))
                        .collect::<Result<Vec<_>>>()?;
                    rows.push(json!({"noise":noise,"layers":layers,"payload":payload,"threshold":threshold,"rates":evaluation(&bad,&good)?,"resources":Resources::measure(&c.circuit)?}));
                }
            }
        }
        let dynamic = PauliSandwich::new("z", 2, true)?;
        let mut ex = NativeExecutor::new(0.)?;
        self.seed += 1;
        let counts = dynamic.sample(&mut ex, 1, 1024, self.seed)?;
        self.circuits += 1;
        self.shots += 1024;
        self.jobs += 1;
        self.log(json!({"kind":"dynamic_check","circuit":dynamic.circuit,"seed":self.seed,"counts":counts}))?;
        Ok(
            json!({"conditions":rows,"dynamic":{"counts":counts,"flag_preserved":PauliSandwich::flag_probability(&counts[0])?==1.,"all_data_reset":counts[0].iter().enumerate().all(|(i,n)|i&1==0||*n==0)},"note":"CZ and Y lowered to supported primitive gates; terminal reset mapping exact for this restricted gadget, not arbitrary dynamic circuits."}),
        )
    }
}
fn extension_payloads(width: u8, equivalent: bool) -> Vec<(String, Vec<Gate>)> {
    let q = width - 1;
    let mut cases = vec![
        ("clean", vec![]),
        ("x", vec![Gate::x(q)]),
        ("z", vec![Gate::z(q)]),
        ("cx", vec![Gate::cx(q - 1, q)]),
        ("xz", vec![Gate::x(q), Gate::z(q)]),
        ("cxz", vec![Gate::cx(q - 1, q), Gate::z(q)]),
        ("weak_rotation", vec![Gate::rotation("rz", q, 0.12)]),
        ("benign_noise_drift", vec![]),
        ("public_probe_adaptive_fabrication", vec![]),
    ];
    if equivalent {
        cases.push(("equivalent_x_pair", vec![Gate::x(q), Gate::x(q)]));
    }
    cases.into_iter().map(|(s, g)| (s.into(), g)).collect()
}
