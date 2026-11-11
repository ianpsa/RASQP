//! Explanations are separate from measured evidence and frozen research criteria.
pub struct Topic {
    pub title: &'static str,
    pub body: &'static str,
    pub art: &'static str,
    pub source: &'static str,
}

pub const LESSONS: &[Topic] = &[
    Topic {
        title: "Start here: what is RASQP?",
        body: "A quantum workflow still depends on ordinary software: it builds a circuit, sends a job, and decides whether to use the answer. RASQP studies checks at these boundaries.\n\nThink of a circuit as a recipe, a backend as the kitchen, and measurement counts as a tally of dishes served. Checking the recipe and checking the tally answer different questions.\n\nThe runtime guard compares the submitted recipe with an approved one, checks a classical artifact, runs a backend, and compares measurements with an approved distribution. Optional probe jobs ask extra questions.\n\nThe signed-result path checks an already collected response, its request, authentication tag, context, and replay history before releasing counts.\n\nTry this tour, then open Workbench. Start with the clean control, then compare a bit flip and a phase flip. Studies runs the larger frozen experiments; Code map explains the modules and every JSON operation.",
        art: "        Approved recipe\n       ┌───────────────┐\n       │    circuit    │\n       └───────┬───────┘\n               ▼\n       ┌───────────────┐\n       │ inspect / bind│\n       └───────┬───────┘\n               ▼\n       ┌───────────────┐\n       │backend → tally│\n       └───────┬───────┘\n               ▼\n       ┌───────────────┐\n       │ verify / gate │\n       └───┬───────┬───┘\n           ▼       ▼\n        release  withhold",
        source: "rust/application/runtime.rs; rust/application/guard.rs",
    },
    Topic {
        title: "A qubit is not a tiny classical coin",
        body: "A classical bit has value 0 or 1. A qubit has a quantum state; measuring it gives a classical outcome with probabilities determined by the state and measurement basis.\n\nA coin is useful only as an analogy for repeated measurements. A quantum state also carries phase: information that can affect interference even when one measurement view looks identical.\n\nA circuit diagram reads left to right. Each wire is a qubit. X flips 0 and 1. H changes the measurement view and can prepare an equal superposition from 0. Z changes relative phase. CX flips its target only when its control is 1. Rotations make continuous changes.\n\nA shot is one execution and measurement. Run the same circuit many times and count the outcomes. The bars in Workbench show a reference probability beside the sampled candidate frequency. They are different kinds of evidence.\n\nBit strings use q0 on the RIGHT: 01 means qubit 0 is 1. Two qubits have four possible outcomes: 00, 01, 10, 11.",
        art: " q0 |0> ---[H]---o---- measure\n                  |\n q1 |0> ---------[X]--- measure\n\n Ideal Bell outcomes (Z view):\n 00  ################  50%\n 01                     0%\n 10                     0%\n 11  ################  50%\n\n Correlation is visible in the joint tally.",
        source: "rust/domain/circuit.rs; rust/adapters/simulator.rs",
    },
    Topic {
        title: "Why one measurement view can miss a change",
        body: "Looking at an object from one angle can hide a difference. In the same spirit, measuring only in Z can miss a phase change. This analogy explains observability; it does not make quantum phase a classical hidden variable.\n\nThe phase-flip experiment starts with an empty one-qubit recipe and appends Z. With input 0 measured in Z, both recipes always return 0. Their full operators differ even though this observation agrees.\n\nPress b in Workbench to use the + / X probe. Preparation adds H before the recipe; measurement adds H after it. Now the clean recipe returns 0 and the Z mutation returns 1 in the ideal simulator.\n\nA dormant CX gives another blind spot: input 00 never activates its control. The 1 / Z probe prepares 11, activating the extra controlled operation.\n\nProbe selection is offline and uses supplied suspect circuits. Seeing those suspects during selection differs from detecting an unknown hidden backend attack.",
        art: " One view: input 0, read Z\n ∣0⟩ ──┤ Z ├── M   same counts\n\n A second view: input +, read X\n ∣0⟩ ─┤ H ├─┤ Z ├─┤ H ├─ M\n                       changed counts\n\n prepare → recipe → change view → count\n\n A blind spot is a result.",
        source: "rust/research/probes.rs; H07, H08, H23, H41",
    },
    Topic {
        title: "How a detector decides to raise an alarm",
        body: "An alarm asks whether the observed tally is too far from a trusted reference. It does not identify the cause. Malicious changes, benign device drift, and an unsuitable reference can all move the tally.\n\nTotal variation distance (TVD) is a number from 0 to 1. Zero means identical distributions; one means disjoint outcomes. It is half the sum of absolute probability differences.\n\nCalibration runs clean circuits first to set a tolerance. Workbench uses 79 clean repetitions and alpha = 0.05, then tests 64 independent clean and candidate repetitions. Its reference matches the selected noise model. The noisy clean control can still alarm by chance.\n\nDetection rate means the fraction of candidate repetitions that alarm. Clean alarm rate means the fraction of clean repetitions that alarm. When the candidate is benign, its alarms are false positives too. Wilson 95% intervals display uncertainty around these measured rates.\n\nMore probes create more chances to alarm. Joint calibration uses the largest TVD across a fixed portfolio. Changing probes, shots, or context requires recalibration. A target alpha is not a promise under drift.",
        art: " clean calibration ──▶ tolerance\n                            │\n observed tally ──▶ distance ┼──▶ compare\n\n distance ≤ tolerance: quiet\n distance > tolerance: alarm\n\n Circuit edits have a separate gate.\n Quiet does not prove honest execution.",
        source: "rust/research/verification.rs; rust/research/statistics.rs",
    },
    Topic {
        title: "The five research questions",
        body: "The manuscript triage groups the work into five retrospective questions. H01-H44 are registry identifiers, not 44 independently validated security claims.\n\n1. Behavioral sensitivity: which interventions produce observable differences? Keep zero-separation cases and weak rotations.\n\n2. Probe selection at a fixed budget: does complementary coverage help on unseen mutations? H41 failed its original conjunction, including its noisy equivalent-control requirement.\n\n3. Alarm calibration: can clean alarms stay within bounds while strong-mutation detection stays high? Native H42 and the original H44 screening passed their recorded criteria; broader external criteria do not all pass.\n\n4. Context transfer: can the check distinguish malicious changes from benign compilation or drift? General discrimination is not established.\n\n5. Security boundary: what does measurement add to visible classical inspection? A plausible public-probe fabrication can pass. Native synthesized combined-rejection counters are not end-to-end runtime executions.\n\nThe tiny Workbench demonstrations teach mechanisms. They do not rerun the frozen hypothesis protocols. Use Studies for those native runs and inspect their original criteria and evidence.",
        art: "             sensitivity\n                  │\n coverage ─── [ evidence ] ── calibration\n                  │              │\n           trust boundary ─── context\n\n Keep the negative controls in view.",
        source: "docs/RESEARCH_V03.txt; docs/NATIVE_PROTOCOL_V1.json",
    },
    Topic {
        title: "What an accepted result actually means",
        body: "Accept means the enabled checks passed within the evaluated policy. Reject means a specified violation was found. Inconclusive means the available evidence does not establish acceptance. Both Reject and Inconclusive withhold counts.\n\nThe trusted application, approved reference, key, policy, and replay ledger must be protected independently. A compromised verifier host is outside the guarantee. Artifact hashes compare file bytes against an approved baseline; they do not attest loaded machine code.\n\nHMAC binds counts to a request for a signer that actually owns the key. It is not an IBM signature and cannot make a dishonest authorized signer honest. The in-memory ledger used in Workbench forgets history when recreated; production paths use a durable SQLite ledger.\n\nPublic probes are separate jobs. A backend can answer them plausibly while treating the workload differently. Distribution-preserving changes can escape behavioral checks. Local operator comparison is limited to small circuits and scales exponentially.\n\nThis interface runs classical simulation. No real QPU execution, quantum advantage, universal tamper detection, private watermarking, or cryptographic quantum authentication is demonstrated.",
        art: " Trusted host\n ┌──────────────────────────────────┐\n │ approved policy → checks → gate  │\n │ key / artifact / replay ledger   │\n └─────────────────┬────────────────┘\n                   │\n            backend boundary\n                   │\n       separate jobs, limited evidence",
        source: "docs/RUST_SDK.txt; docs/RESEARCH_V03.txt",
    },
];

pub const DEMOS: &[Topic] = &[
    Topic {
        title: "Clean control",
        body: "Question: does an unchanged Bell circuit stay within clean calibration?\n\nThe H + CX recipe prepares correlated qubits. Candidate and reference are identical. At zero noise, only 00 and 11 occur; finite samples change their proportions. Candidate alarms here are false positives. Try n for noise and s for more shots.",
        art: "",
        source: "Family::Ghz; NativeExecutor; CalibratedVerifier",
    },
    Topic {
        title: "Bit flip: a visible change",
        body: "Question: does appending X to one Bell qubit move the joint output tally?\n\nThe clean Bell circuit yields 00 / 11; flipping q0 yields 01 / 10 in the Z view. Structural inspection also sees a changed circuit, so behavioral sensitivity alone does not establish added runtime protection.",
        art: "",
        source: "Gate::x; inspect; CalibratedVerifier",
    },
    Topic {
        title: "Phase flip: a blind spot",
        body: "Question: can a changed operator produce the same measured counts?\n\nThe reference is the empty one-qubit recipe; the candidate appends Z. The default 0 / Z probe sees no change. Press b to try + / X: it turns relative phase into an observable difference. A structural decision and a behavioral alarm are different evidence.",
        art: "",
        source: "H07; Probe::apply; Statevector::equivalent",
    },
    Topic {
        title: "Dormant controlled flip",
        body: "Question: can the input hide a controlled operation?\n\nThe reference has no gates; the candidate adds CX(0,1). Input 00 never activates it. Press b twice to choose 1 / Z, prepare 11 and activate the control. Try other probes to discover their blind spots.",
        art: "",
        source: "H08; Gate::cx; Probe::global",
    },
    Topic {
        title: "Small rotation, limited budget",
        body: "Question: how much can a finite measurement budget reveal?\n\nThe candidate adds RY(0.15 radians) to an empty one-qubit circuit. Its effect depends on the probe. Press s to compare 256, 1024 and 4096 shots, or n to add noise. One small example does not establish general sample complexity.",
        art: "",
        source: "H09; Gate::rotation; Rate::from_flags",
    },
    Topic {
        title: "Benign rewrite, physical cost",
        body: "Question: can a logically equivalent rewrite cost more under noise?\n\nTwo appended H gates cancel ideally. Normalization can recognize that identity; the raw circuit still executes both gates in the noisy simulator. Press n to compare. Mathematical equivalence does not imply identical noisy measurements or an unchanged resource budget.",
        art: "",
        source: "H14; Normalizer; DensityMatrix",
    },
    Topic {
        title: "Benign device drift",
        body: "Question: does an alarm identify an attack?\n\nThe recipe is unchanged. Calibration uses the selected noise probability; candidate samples use that probability plus 0.03. The tally may move solely because the device drifted. This is a negative control for malicious-cause attribution.",
        art: "",
        source: "H27; NoiseModel::depolarizing",
    },
    Topic {
        title: "Signed response and replay",
        body: "Question: can the same authenticated response be accepted twice?\n\nA local fixture signs one Bell tally with a demonstration key, then calls the actual protection gate twice using one in-memory replay ledger. A separate tampered-count fixture changes signed counts without updating the tag. Inspect the measured gate outcomes below. This is a local authentication contract, not provider attestation.",
        art: "",
        source: "sign; protect; MemoryLedger (session-only fixture)",
    },
    Topic {
        title: "Guard a backend execution",
        body: "Question: what does the runtime actually release?\n\nThis calls GuardedRuntime with an approved probe-wrapped Bell recipe and a seeded native simulator backend. It checks a self-approved fingerprint of the running executable, executes the approved recipe, and evaluates the result. Then it tries a visible X edit: conservative inspection should withhold it before submitting any shots.\n\nThe selected probe is part of this approved workload; optional separate probe jobs are disabled. The artifact baseline is a local demonstration, not independent attestation. Read the actual decisions, released counts and submitted-shot totals below.",
        art: "",
        source: "GuardedRuntime::execute; RuntimePolicy; QuantumBackend",
    },
];

pub const STUDIES: &[Topic] = &[
    Topic {
        title: "all",
        body: "Run every native study and supplemental demonstration using the frozen native protocol. Outputs include the exact protocol, raw seeded trials, summary JSON and a text report.\n\nThese are simulator experiments, not independent confirmations of earlier work. Selection, clean calibration and evaluation must be interpreted using each study's own information boundary.\n\nOn Linux, the child laboratory uses its existing seccomp sandbox. On other systems press u to explicitly opt into an unconfined child before running. The interactive UI itself must remain outside seccomp so it can read keys and render.\n\nEnter starts a new run; x cancels it. Every run gets a unique directory under results/runs/tui/. Cancellation can leave partial evidence. Press v to inspect the loaded JSON and r to reload its summary.",
        art: "",
        source: "rust/lab/experiment.rs; docs/NATIVE_PROTOCOL_V1.json",
    },
    Topic {
        title: "batch",
        body: "Question: how do detectors behave across circuit families, widths, shot budgets and noise?\n\nUses basis, GHZ and variational recipes; widths 2/3/4; seeds 17/71; noise 0/0.005; shots 256/1024; 59 clean calibration repetitions and 24 evaluation repetitions. Includes mutations, fabrication cases and supplemental demonstrations.\n\nRead detection AND clean-alarm rates. The study note explains where rejection counters are synthesized rather than end-to-end executions.",
        art: "",
        source: "ResearchLab::batch; ResearchLab::supplemental",
    },
    Topic {
        title: "hypotheses",
        body: "Question: which normalization, mutation, probe and restricted-check mechanisms work in the specified fixtures?\n\nIncludes rewrite preservation/incompleteness, lineage, mutation reversal, held-out-family diagnostics, fixed/selected/multiple probes, and Pauli gadgets. Probe trials use 79 clean calibration repetitions, 128 evaluation repetitions and 1024 total shots.\n\nThis is not a success scoreboard for H01-H38. H25 native summary values are not an executed relational test. H33/H35 are deferred, H36 lacks applicable channel assumptions, and H37 has no advantage demonstration. Preserve the negative findings.",
        art: "",
        source: "rust/lab/studies.rs; docs/HYPOTHESIS_PROTOCOL.json",
    },
    Topic {
        title: "trojans",
        body: "Question: which inserted gates, triggers and idle changes affect these bounded simulator workloads?\n\nCompare outcomes by mutation and input context. A dormant trigger or distribution-preserving change may stay invisible to a chosen observation. A visible changed circuit can already be withheld by classical inspection.\n\nSynthetic payloads and small local circuits do not establish coverage of arbitrary malicious backends.",
        art: "",
        source: "ResearchLab::trojans; rust/research/attacks.rs",
    },
    Topic {
        title: "extensions",
        body: "Question: does a training-selected portfolio cover held-out mutations at equal evaluation shots (H41), and does joint calibration control alarms (H42)?\n\nUses width 3, 127 clean calibration repetitions, 128 evaluation repetitions and 768 total shots, under noise 0/0.005. Compare single-selection and complementary-selection outputs, clean/equivalent controls, and per-family Wilson intervals.\n\nNative H41 failed its original conjunction; native H42 passed its recorded screening criterion. These outcomes apply to the loaded run only. Naive OR versus a joint rule is not a superiority result without matched false-alarm rates.",
        art: "",
        source: "docs/EXTENSION_PROTOCOL_V03.json; ResearchLab::extension",
    },
    Topic {
        title: "context",
        body: "Question: does approval of the exact raw circuit and noise context meet bounded clean-alarm and strong-mutation criteria (H44)?\n\nUses width 4; padded/unpadded contexts; noise 0/0.005/0.01; 511 calibration repetitions; 256 evaluation repetitions; 768 total evaluation shots; 8192 reference shots per probe.\n\nThe candidate must have clean Wilson upper bound <= 0.10 and each of five strong-mutation Wilson lower bounds >= 0.90 in every context. Weak rotations and benign drift remain separate.\n\nThe original native six-context screening passed. Passing the candidate alone does not show noisy-reference superiority; inspect all three ablations. Broader external/IBM failures remain separate evidence.",
        art: "",
        source: "docs/EXTENSION_PROTOCOL_V04.json; ResearchLab::context_extension",
    },
];

pub const CODE_MAP: &[Topic] = &[
    Topic {
        title: "Domain: recipes and contracts",
        body: "Circuit and Gate validate widths, operands, supported instructions and finite angles. QuantumExecutor samples counts; ProbabilityOracle returns distributions; EquivalenceOracle compares operators. Family supplies basis, GHZ and variational fixtures.\n\nThese traits let an application replace the native simulator without treating a provider as trusted. Workbench uses the same domain types.",
        art: " circuit -> executor -> counts\n    |          |\n validate   backend adapter",
        source: "rust/domain/{circuit,workload,mod}.rs",
    },
    Topic {
        title: "Runtime: guard actual execution",
        body: "GuardedRuntime binds an independently approved RuntimePolicy to a backend and nonce store. It inspects a candidate, checks the artifact and backend identity, executes, checks count shape and distribution, optionally runs challenge jobs, rechecks the artifact, and releases accepted_counts only after enabled checks pass.\n\nRun the example: cargo run --locked --example runtime",
        art: " inspect -> artifact -> execute -> score\n                                    |\n                           probes -> recheck -> gate",
        source: "rust/application/runtime.rs; examples/runtime.rs",
    },
    Topic {
        title: "Classical: signatures and replay",
        body: "Transcript authentication binds a job ID, nonce, circuit digest, backend, width, shots, artifact, context and exact counts. SqliteLedger atomically persists first-use claims. ArtifactFingerprint measures file bytes. Function guards provide limited supplied-snapshot/replacement checks.\n\nThe signed-result example is separate from guarded backend execution: cargo run --locked --example protect",
        art: " request + counts + key -> authentication tag\n expected fields + tag + fresh nonce -> verify",
        source: "rust/classical/; examples/protect.rs",
    },
    Topic {
        title: "Research: compare and calibrate",
        body: "Normalization has conservative rewrites and traces. Alignment localizes edits after normalization; lineage compares trusted snapshots; a variant catalogue recognizes known forms. Probe selection uses a bounded oracle and supplied suspects. CalibratedVerifier / JointVerifier score distributions. Statistics supplies Wilson intervals, paired comparisons and Holm correction. Mutations and obfuscation are research demonstrators.",
        art: " approved + suspects -> select probes\n approved clean data -> calibrate -> test holdouts",
        source: "rust/research/",
    },
    Topic {
        title: "Adapters: simulation and interop",
        body: "Statevector computes ideal states and bounded full-operator equivalence. DensityMatrix adds gate/readout noise; NativeExecutor samples seeded counts. NativeBackend adapts simulation to GuardedRuntime. Compiler passes transform or inject gates; OpenQasm exports recipes; PauliSandwich implements a restricted check gadget.\n\nMemory grows exponentially. The UI keeps examples at one or two qubits. These adapters do not submit physical hardware jobs.",
        art: " circuit -> native simulation -> probabilities\n                         |\n                     seeded shots -> counts",
        source: "rust/adapters/",
    },
    Topic {
        title: "Laboratory, protocols and releases",
        body: "rasqp-lab runs all/batch/hypotheses/trojans/extensions/context and writes raw trials and reports. ProtocolRegistry embeds versioned frozen criteria. The Linux child uses seccomp by default.\n\nrasqp-release packages native binaries, SDK source, docs and checksums. rasqp-external evaluates the separate external-dataset input contract; run it without arguments for usage. External dataset / clean-control / IBM studies have separate protocols and evidence under research/ and results/. The TUI native runner does not launch physical IBM jobs.",
        art: " frozen protocol -> seeded run -> raw trials\n                                    |\n                              summary + report",
        source: "rust/lab/; rust/{lab,release,external}_main.rs",
    },
    Topic {
        title: "CLI: digest",
        body: "Compute a versioned SHA-256 digest of a validated circuit. It commits to the submitted recipe, not to execution by a device.\n\nJSON example:\n{\"operation\":\"digest\",\"circuit\":{\"width\":1,\"gates\":[]}}\n\nPipe a command file into: cargo run --locked --bin rasqp < command.json",
        art: " recipe bytes -> SHA-256 -> identifier",
        source: "rust/main.rs: Command::Digest",
    },
    Topic {
        title: "CLI: normalize / normalize_trace",
        body: "normalize applies conservative local rewrites. normalize_trace also returns the rewrite offsets and rules. Canceling H,H is supported; arbitrary semantic equivalence is not solved.\n\n{\"operation\":\"normalize_trace\",\"circuit\":{\"width\":1,\"gates\":[{\"name\":\"h\",\"qubits\":[0]},{\"name\":\"h\",\"qubits\":[0]}]}}",
        art: " --[H]--[H]--  =>  ------------",
        source: "rust/main.rs; rust/research/normalization.rs",
    },
    Topic {
        title: "CLI: inspect / equivalent / align",
        body: "inspect compares approved and submitted recipes with max_extra_gates. Unknown normalized equality is inconclusive and counts stay withheld. equivalent uses the ideal bounded full-operator oracle; it is expensive and limited to eight qubits. align reports normalized gate insertions, deletions and substitutions; edit distance is not quantum behavioral distance.\n\nFields: inspect {approved,submitted,max_extra_gates}; equivalent and align {left,right}.",
        art: " raw edit? -> local normalization -> known match?\n full-operator oracle answers a stronger bounded question",
        source: "rust/main.rs; rust/domain/circuit.rs; rust/research/genome.rs",
    },
    Topic {
        title: "CLI: probabilities / qasm",
        body: "probabilities returns exact local simulator probabilities, ideally or with optional depolarizing noise. It does not run the protection gate. qasm exports OpenQASM for interoperability.\n\n{\"operation\":\"probabilities\",\"circuit\":{\"width\":1,\"gates\":[{\"name\":\"h\",\"qubits\":[0]}]},\"noise\":null}\n\nqasm fields: {circuit}.",
        art: " recipe -> simulator -> [0.5, 0.5]\n recipe -> QASM text -> another tool",
        source: "rust/main.rs; rust/adapters/{simulator,qasm}.rs",
    },
    Topic {
        title: "CLI: select_probes",
        body: "Offline portfolio selection maximizes the worst supplied suspect's best probe separation. Inputs: reference, suspects, size. Output includes probes, the training TVD matrix and objective.\n\nSuspects are visible during selection. Use separate training and evaluation sets when studying generalization. A public portfolio is not a secret challenge protocol.",
        art: " supplied suspects -> candidate views -> portfolio",
        source: "rust/main.rs; rust/research/probes.rs",
    },
    Topic {
        title: "CLI: score / joint_score / evaluate_joint",
        body: "score applies a supplied CalibratedVerifier to counts. joint_score applies a supplied JointVerifier to an ordered count batch. evaluate_joint calibrates on supplied clean batches and scores separate batches.\n\nFields: score {verifier:{reference,threshold},counts}; joint_score {verifier:{references,threshold},counts}; evaluate_joint {references,clean,batches,alpha}.\n\nCalibration must match the exact probes, raw variants, shot allocation and context. A clean calibration set must not be retuned on attacks.",
        art: " clean only -> threshold\n held-out counts -> TVD -> alarm?",
        source: "rust/main.rs; rust/research/verification.rs",
    },
    Topic {
        title: "CLI: protect / protect_batch",
        body: "Gate already collected signed results. protect takes approved, submitted, request, response, policy. protect_batch takes entries and a joint policy; it withholds the entire batch if any enabled check fails. Failed attempts can consume nonces.\n\nThese operations require an explicit persistent ledger path and a >=32-byte hex key in RASQP_KEY_HEX.\n\nrasqp path/to/ledger.sqlite < command.json\n\nUse accepted_counts only. A local HMAC fixture is not a signature issued by a remote provider.",
        art: " authenticate -> bind -> replay check -> inspect\n                          -> distribution -> release all",
        source: "rust/application/guard.rs; rust/main.rs",
    },
    Topic {
        title: "CLI: paired / holm",
        body: "paired compares equal-length boolean detection vectors for the same trials using an exact one-sided paired comparison. holm adjusts a family of p-values for multiple comparisons.\n\n{\"operation\":\"paired\",\"baseline\":[false,true,false],\"candidate\":[true,true,false]}\n\n{\"operation\":\"holm\",\"pvalues\":[0.01,0.04,0.2]}\n\nStatistical significance does not remove information leakage, invalid controls or context mismatch.",
        art: " same trial -> baseline / candidate\n discordant outcomes -> paired evidence",
        source: "rust/research/statistics.rs; rust/main.rs",
    },
];

pub const GLOSSARY: &[Topic] = &[
    Topic {
        title: "Circuit / gate",
        body: "A circuit is a sequence of quantum instructions. A gate is one instruction acting on specified qubits. RASQP supports X, H, Z, RX, RY, RZ and CX. A circuit is a recipe, not evidence that a device followed it.",
        art: " time -->\n q0 --[H]---o----\n            |\n q1 -------[X]---",
        source: "rust/domain/circuit.rs",
    },
    Topic {
        title: "Qubit / superposition / phase",
        body: "A qubit is a two-level quantum system. Superposition assigns amplitudes to its possible outcomes. Measurement probabilities depend on squared amplitude magnitudes; relative phase affects interference and can be invisible in one measurement basis. A qubit is not simply an unknown classical bit.",
        art: " magnitude -> probability\n relative phase -> interference",
        source: "Learn: A qubit is not a tiny classical coin",
    },
    Topic {
        title: "Measurement basis / probe",
        body: "A basis specifies the measurement view, such as Z, X or Y. A probe combines an input preparation and a measurement basis. The catalogue has 12 uniform choices: inputs 0, 1, +, i and views Z, X, Y. A portfolio combines views to reduce specific blind spots.",
        art: " prepare -> recipe -> rotate view -> measure",
        source: "rust/research/probes.rs",
    },
    Topic {
        title: "Shot / count / probability",
        body: "A shot is one execution and measurement. Counts tally outcomes across shots. Dividing counts by the shot total estimates probabilities. Simulator probabilities and sampled frequencies need not agree exactly. More shots reduce sampling uncertainty; they do not fix a blind measurement view.",
        art: " 0 1 0 0 1  -> counts [3,2]\n           -> frequencies [0.6,0.4]",
        source: "QuantumExecutor; ProbabilityOracle",
    },
    Topic {
        title: "Bell / GHZ / entanglement",
        body: "A Bell circuit creates a two-qubit entangled state; GHZ extends this pattern to more qubits. The ideal H-then-CX example returns only 00 and 11 in Z. Correlated counts are useful to explain the fixture, but a single classical tally alone is not a general entanglement proof.",
        art: " 00 <---- equal ideal weights ----> 11",
        source: "rust/domain/workload.rs: Family::Ghz",
    },
    Topic {
        title: "TVD / threshold",
        body: "Total variation distance is half the sum of absolute differences between probability vectors. It ranges from 0 (same) to 1 (disjoint). A threshold is the approved tolerance. An alarm means score > threshold + numeric tolerance; it does not establish malicious cause.",
        art: " [1,0] vs [0,1] -> TVD 1\n [.5,.5] vs [.6,.4] -> TVD .1",
        source: "rust/research/verification.rs",
    },
    Topic {
        title: "Calibration / alpha / context",
        body: "Calibration estimates a threshold from independent clean observations. Alpha is a target upper-tail fraction. A context includes raw circuit, noise/backend state, probe order and shot allocation. A threshold learned in one context need not transfer to another.",
        art: " clean data -> fit tolerance -> freeze -> evaluate",
        source: "H42 / H44; CalibratedVerifier; JointVerifier",
    },
    Topic {
        title: "TPR / FPR / Wilson interval",
        body: "TPR is the fraction of attacked repetitions that alarm. FPR is the fraction of clean repetitions that alarm. Wilson 95% intervals show finite-sample uncertainty in these rates. A high aggregate detection rate can hide a completely missed family. A small aggregate FPR does not guarantee each context meets the same bound.",
        art: " attack + alarm -> detection\n clean  + alarm -> false positive\n attack + quiet -> missed intervention",
        source: "rust/research/statistics.rs: Rate",
    },
    Topic {
        title: "Noise / drift / false cause",
        body: "Noise is modeled imperfection in gates or readout. Drift changes that noise over time. Both can shift output distributions without an attack. RASQP's simple depolarizing model is a simulation model; it does not capture every device effect.",
        art: " same recipe + changed context -> changed tally",
        source: "NoiseModel; H10 / H27 / H44",
    },
    Topic {
        title: "Normalization / equivalence",
        body: "Normalization rewrites known local identities. Equality after normalization is useful evidence. Inequality is not proof of different behavior: the rewrite rules are incomplete. Full-operator equivalence asks a stronger bounded question up to global phase, with exponential cost.",
        art: " H H -> identity\n different normalized form -> may remain inconclusive",
        source: "rust/research/normalization.rs; Statevector",
    },
    Topic {
        title: "Digest / HMAC / nonce / ledger",
        body: "A digest identifies bytes. HMAC authenticates a message for holders of a secret key. A nonce is a unique request token. A ledger records consumed nonces to reject replay. These checks bind declared data; a dishonest key holder can still sign a lie. Durable replay protection needs persistent trusted storage.",
        art: " message + key -> tag\n nonce -> ledger: first use?",
        source: "rust/classical/transcript.rs",
    },
    Topic {
        title: "Accept / Reject / Inconclusive",
        body: "Accept releases counts after all enabled checks pass. Reject reports a specified violation. Inconclusive means insufficient evidence for acceptance, including unknown normalization or changed calibration context. Reject and Inconclusive both withhold counts. No behavioral alarm alone is weaker than an accepted guarded result.",
        art: " checks -> Accept       -> Some(counts)\n        -> Reject       -> None\n        -> Inconclusive -> None",
        source: "rust/application/guard.rs",
    },
    Topic {
        title: "Holdout / ablation / negative control",
        body: "A holdout evaluates examples withheld from training/selection. An ablation removes a component to test what it contributes. A negative control intentionally has no malicious intervention, for example benign drift or an equivalent rewrite. Preserve failures and blind spots when comparing methods.",
        art: " train -> freeze -> holdout\n full method <-> remove component\n malicious case <-> benign control",
        source: "docs/RESEARCH_V03.txt; docs/NATIVE_PROTOCOL_V1.json",
    },
    Topic {
        title: "Oracle / QPU / quantum advantage",
        body: "An oracle is a trusted idealized computation used by these local experiments. A QPU is physical quantum hardware; the native simulator is ordinary classical software. Quantum advantage would require a competitive end-to-end demonstration, which this project does not provide.",
        art: " local classical simulator != physical QPU",
        source: "H37 / H38; docs/RESEARCH_V03.txt",
    },
];
