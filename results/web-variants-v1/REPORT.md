# Web-derived attack mechanisms: fixed-detector transfer test

The Wildisley infected rows did not fit RASQP references or thresholds. The detector is a calibrated distribution comparison, not a classifier trained on those infected labels. Earlier probe selection did use toy X/Z/CX mutations, so related-family familiarity remains real. The new CLI regression replaces infected candidates with clean circuits and verifies identical calibration data and thresholds, while evaluation outcomes change.

This challenge uses 32 fresh four-qubit workloads and 320 locally implemented variants. **These are literature-derived mechanism tests, not a newly downloaded independent dataset or a reproduction of full paper classification/hardware attacks.** The same three probes, alpha, reference size, calibration size and evaluation budget are retained; detector file hashes match the Wildisley run.

| Noise | Workload | Variant | Joint detection | Fixed detection | Ideally invisible bases |
|---|---|---|---:|---:|---:|
| 0 | encoding_variational | swap_start | 0.0088 | 0.0078 | 16/16 |
| 0 | encoding_variational | swap_middle | 1.0000 | 1.0000 | 0/16 |
| 0 | encoding_variational | swap_end | 1.0000 | 1.0000 | 0/16 |
| 0 | encoding_variational | parameter_scale_plus10 | 0.7354 | 0.9209 | 0/16 |
| 0 | encoding_variational | parameter_scale_plus50 | 1.0000 | 1.0000 | 0/16 |
| 0 | encoding_variational | parameter_scale_minus50 | 1.0000 | 1.0000 | 0/16 |
| 0 | encoding_variational | mixed_trigger_mcx_001 | 0.0088 | 0.0078 | 16/16 |
| 0 | encoding_variational | mixed_trigger_phase_010 | 0.0205 | 0.0117 | 0/16 |
| 0 | encoding_variational | qtrojan_encoding_sandwich | 1.0000 | 1.0000 | 0/16 |
| 0 | encoding_variational | encoding_layer_erasure | 1.0000 | 1.0000 | 0/16 |
| 0 | qaoa_maxcut | swap_start | 0.0078 | 0.0137 | 16/16 |
| 0 | qaoa_maxcut | swap_middle | 0.7520 | 0.7559 | 4/16 |
| 0 | qaoa_maxcut | swap_end | 0.7529 | 0.7539 | 4/16 |
| 0 | qaoa_maxcut | parameter_scale_plus10 | 0.4375 | 0.7021 | 0/16 |
| 0 | qaoa_maxcut | parameter_scale_plus50 | 1.0000 | 1.0000 | 0/16 |
| 0 | qaoa_maxcut | parameter_scale_minus50 | 0.9990 | 1.0000 | 0/16 |
| 0 | qaoa_maxcut | mixed_trigger_mcx_001 | 0.0137 | 0.0137 | 16/16 |
| 0 | qaoa_maxcut | mixed_trigger_phase_010 | 0.0195 | 0.0117 | 0/16 |
| 0 | qaoa_maxcut | cost_layer_erasure | 0.8887 | 0.9590 | 0/16 |
| 0 | qaoa_maxcut | mixer_sign_flip | 1.0000 | 1.0000 | 0/16 |
| 0.005 | encoding_variational | swap_start | 0.0039 | 0.0107 | 16/16 |
| 0.005 | encoding_variational | swap_middle | 1.0000 | 1.0000 | 0/16 |
| 0.005 | encoding_variational | swap_end | 1.0000 | 0.9912 | 0/16 |
| 0.005 | encoding_variational | parameter_scale_plus10 | 0.5059 | 0.8320 | 0/16 |
| 0.005 | encoding_variational | parameter_scale_plus50 | 1.0000 | 1.0000 | 0/16 |
| 0.005 | encoding_variational | parameter_scale_minus50 | 1.0000 | 1.0000 | 0/16 |
| 0.005 | encoding_variational | mixed_trigger_mcx_001 | 0.0684 | 0.1865 | 16/16 |
| 0.005 | encoding_variational | mixed_trigger_phase_010 | 0.2568 | 0.5713 | 0/16 |
| 0.005 | encoding_variational | qtrojan_encoding_sandwich | 1.0000 | 0.9912 | 0/16 |
| 0.005 | encoding_variational | encoding_layer_erasure | 1.0000 | 1.0000 | 0/16 |
| 0.005 | qaoa_maxcut | swap_start | 0.0098 | 0.0127 | 16/16 |
| 0.005 | qaoa_maxcut | swap_middle | 0.7539 | 0.7490 | 4/16 |
| 0.005 | qaoa_maxcut | swap_end | 0.7217 | 0.7568 | 4/16 |
| 0.005 | qaoa_maxcut | parameter_scale_plus10 | 0.3057 | 0.5967 | 0/16 |
| 0.005 | qaoa_maxcut | parameter_scale_plus50 | 0.9600 | 1.0000 | 0/16 |
| 0.005 | qaoa_maxcut | parameter_scale_minus50 | 0.9863 | 1.0000 | 0/16 |
| 0.005 | qaoa_maxcut | mixed_trigger_mcx_001 | 0.0605 | 0.1377 | 16/16 |
| 0.005 | qaoa_maxcut | mixed_trigger_phase_010 | 0.1553 | 0.2998 | 0/16 |
| 0.005 | qaoa_maxcut | cost_layer_erasure | 0.8301 | 0.9131 | 0/16 |
| 0.005 | qaoa_maxcut | mixer_sign_flip | 0.9980 | 0.9971 | 0/16 |

Per-variant >=90% detection and every-context clean confidence requirements are evaluated separately in evaluation.json. Passing a mean never substitutes for a failed condition. Clean calibration is conditional on trusted raw contexts; semantic-equivalence cases are reported separately. The field named fold is an execution block, not a newly trained cross-validation fold.

Sources and implementation boundaries:
- [SQUASH](https://arxiv.org/html/2506.24081v1): untargeted SWAP insertion only. The full HQNN classification experiment is not reproduced. A scalar global phase cannot change fidelity; the targeted exp(i delta) expression is not implemented as a physical attack.
- [QTrojan](https://arxiv.org/html/2302.08090v1): active RX/RY encoding sandwich, at circuit level. The pinned author notebook was inspected as JSON and never executed. No pulse configuration, trained QNN or classification attack-success rate is claimed.
- [Upadhyay and Ghosh](https://arxiv.org/html/2209.11872v1): multiplicative variational-parameter tampering. No full iterative optimizer, cloud or approximation-ratio experiment is reproduced.
- [Conditional Trojans](https://arxiv.org/html/2502.08880v1): mixed multi-control diagnostic variants extend the activation idea; these exact fixtures are our constructions, not author dataset files.

Known SWAP symmetry and prior rotation/CX studies make these diagnostic transfer tests, not proof that every attack concept was previously unknown. The initial-swap and mixed-trigger cases deliberately retain physically distinct computations invisible to the fixed input preparations. They are dormant on those inputs, including the ordinary zero-state workload execution, so their low alarm rates do not establish a bypass that changes the current workload output. They diagnose limits of general circuit coverage; an activating input is separately demonstrated by the physical regression test. Any noisy detection of such cases can arise from additional gate noise rather than logical activation.

Independent raw verification: 196,608 decisions; native versus Qiskit ideal probe-TVD error at most 2.28e-15. Cost: 127,303,680 simulated shots, zero QPU shots.

[CutBackdoor](https://arxiv.org/abs/2607.18126) requires a cut-execution and reconstruction pipeline absent here. [QAD](https://netslab.ucd.ie/quantum-attack-dataset-qad/) is a separately identified public dataset, not imported or scored in this suite. Neither is counted as successfully tested protection.
