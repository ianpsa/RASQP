# External clean controls and QDoor artifacts

Evaluated 98 author-provided clean benchmark circuits, their supported exact compiler rewrites, and six QDoor approval/feature contexts. The three probes, alpha=0.01, 511 calibration batches, 8192 reference shots per probe and 768 evaluation shots remain frozen. This study uses 256 evaluation repetitions for more precise per-context intervals. No infected artifact enters calibration.

**These are published research artifacts, not circuits recovered from a real intrusion.** QASMBench and VeriQBench are clean unitary-block benchmarks. QDoor supplies trained numeric parameter artifacts and source topology; this replay does not reproduce MNIST/PCA classification, approximate synthesis training, or the paper's attack-success rate.

## Clean-control and compilation results

An alarm on a known benign rewrite is a false alarm if treated as an attack, even when noisy gate behavior really changed. An inconclusive structural inspection is an availability/approval outcome, not an attack diagnosis. Reapproving an independently validated rewrite is a new calibration context, not automatic recalibration.

| Noise | Source : approved circuit : evaluated case | Joint alarms | Worst context | Structural accept / inconclusive / reject |
|---|---|---:|---:|---|
| 0 | QASMBench:optimized:approved | 85/9472 (0.90%) | 3.12% | 37 / 0 / 0 |
| 0 | QASMBench:optimized:equivalent_x_pair | 88/9472 (0.93%) | 3.12% | 37 / 0 / 0 |
| 0 | QASMBench:original:approved | 81/11008 (0.74%) | 2.34% | 43 / 0 / 0 |
| 0 | QASMBench:original:equivalent_x_pair | 80/11008 (0.73%) | 1.95% | 43 / 0 / 0 |
| 0 | QASMBench:original:exact_optimized_rewrite | 63/9472 (0.67%) | 2.73% | 5 / 32 / 0 |
| 0 | QDoor:compiled_initialization:approved | 7/768 (0.91%) | 1.17% | 3 / 0 / 0 |
| 0 | QDoor:compiled_initialization:equivalent_x_pair | 2/768 (0.26%) | 0.78% | 3 / 0 / 0 |
| 0 | QDoor:compiled_initialization:trained_backdoor | 132/768 (17.19%) | 28.91% | 0 / 3 / 0 |
| 0 | QDoor:original:approved | 9/768 (1.17%) | 2.73% | 3 / 0 / 0 |
| 0 | QDoor:original:compiled_initialization | 430/768 (55.99%) | 65.62% | 0 / 3 / 0 |
| 0 | QDoor:original:equivalent_x_pair | 6/768 (0.78%) | 0.78% | 3 / 0 / 0 |
| 0 | QDoor:original:trained_backdoor | 747/768 (97.27%) | 99.61% | 0 / 3 / 0 |
| 0 | VeriQBench:optimized:approved | 27/14080 (0.19%) | 2.73% | 55 / 0 / 0 |
| 0 | VeriQBench:optimized:equivalent_x_pair | 38/14080 (0.27%) | 1.95% | 55 / 0 / 0 |
| 0 | VeriQBench:original:approved | 35/14080 (0.25%) | 2.34% | 55 / 0 / 0 |
| 0 | VeriQBench:original:equivalent_x_pair | 41/14080 (0.29%) | 2.34% | 55 / 0 / 0 |
| 0 | VeriQBench:original:exact_optimized_rewrite | 38/14080 (0.27%) | 1.95% | 8 / 47 / 0 |
| 0.005 | QASMBench:optimized:approved | 83/9472 (0.88%) | 2.73% | 37 / 0 / 0 |
| 0.005 | QASMBench:optimized:equivalent_x_pair | 119/9472 (1.26%) | 3.91% | 37 / 0 / 0 |
| 0.005 | QASMBench:original:approved | 87/11008 (0.79%) | 3.12% | 43 / 0 / 0 |
| 0.005 | QASMBench:original:equivalent_x_pair | 129/11008 (1.17%) | 3.52% | 43 / 0 / 0 |
| 0.005 | QASMBench:original:exact_optimized_rewrite | 1197/9472 (12.64%) | 100.00% | 5 / 32 / 0 |
| 0.005 | QDoor:compiled_initialization:approved | 5/768 (0.65%) | 1.17% | 3 / 0 / 0 |
| 0.005 | QDoor:compiled_initialization:equivalent_x_pair | 8/768 (1.04%) | 1.56% | 3 / 0 / 0 |
| 0.005 | QDoor:compiled_initialization:trained_backdoor | 74/768 (9.64%) | 20.70% | 0 / 3 / 0 |
| 0.005 | QDoor:original:approved | 6/768 (0.78%) | 1.95% | 3 / 0 / 0 |
| 0.005 | QDoor:original:compiled_initialization | 613/768 (79.82%) | 92.19% | 0 / 3 / 0 |
| 0.005 | QDoor:original:equivalent_x_pair | 8/768 (1.04%) | 2.34% | 3 / 0 / 0 |
| 0.005 | QDoor:original:trained_backdoor | 745/768 (97.01%) | 100.00% | 0 / 3 / 0 |
| 0.005 | VeriQBench:optimized:approved | 129/14080 (0.92%) | 3.12% | 55 / 0 / 0 |
| 0.005 | VeriQBench:optimized:equivalent_x_pair | 154/14080 (1.09%) | 5.47% | 55 / 0 / 0 |
| 0.005 | VeriQBench:original:approved | 130/14080 (0.92%) | 2.73% | 55 / 0 / 0 |
| 0.005 | VeriQBench:original:equivalent_x_pair | 181/14080 (1.29%) | 4.30% | 55 / 0 / 0 |
| 0.005 | VeriQBench:original:exact_optimized_rewrite | 606/14080 (4.30%) | 100.00% | 8 / 47 / 0 |

## QDoor author-artifact cases

Compiled initialization is an author-intended benign approximation, not an exactly equivalent rewrite. Its operator overlap distance from the original is about 0.0201; the trained backdoor distance is about 0.0341. The supplied QASM matches initialization parameters within float32 rounding (maximum difference about 2.37e-7). Those distances describe artifacts, not maliciousness or a security bound.

| Noise | Approved | Feature vector | Evaluated artifact | Joint alarms | Wilson95 | Inspector |
|---|---|---|---|---:|---|---|
| 0 | original | [0, 0, 0, 0, 0, 0, 0, 0] | approved | 7/256 | 0.013–0.055 | accept |
| 0 | original | [0, 0, 0, 0, 0, 0, 0, 0] | compiled_initialization | 141/256 | 0.490–0.611 | inconclusive |
| 0 | original | [0, 0, 0, 0, 0, 0, 0, 0] | trained_backdoor | 255/256 | 0.978–0.999 | inconclusive |
| 0.005 | original | [0, 0, 0, 0, 0, 0, 0, 0] | approved | 5/256 | 0.008–0.045 | accept |
| 0.005 | original | [0, 0, 0, 0, 0, 0, 0, 0] | compiled_initialization | 236/256 | 0.882–0.949 | inconclusive |
| 0.005 | original | [0, 0, 0, 0, 0, 0, 0, 0] | trained_backdoor | 256/256 | 0.985–1.000 | inconclusive |
| 0 | compiled_initialization | [0, 0, 0, 0, 0, 0, 0, 0] | approved | 2/256 | 0.002–0.028 | accept |
| 0 | compiled_initialization | [0, 0, 0, 0, 0, 0, 0, 0] | trained_backdoor | 74/256 | 0.237–0.347 | inconclusive |
| 0.005 | compiled_initialization | [0, 0, 0, 0, 0, 0, 0, 0] | approved | 0/256 | 0.000–0.015 | accept |
| 0.005 | compiled_initialization | [0, 0, 0, 0, 0, 0, 0, 0] | trained_backdoor | 53/256 | 0.162–0.261 | inconclusive |
| 0 | original | [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | approved | 2/256 | 0.002–0.028 | accept |
| 0 | original | [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | compiled_initialization | 168/256 | 0.596–0.712 | inconclusive |
| 0 | original | [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | trained_backdoor | 255/256 | 0.978–0.999 | inconclusive |
| 0.005 | original | [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | approved | 0/256 | 0.000–0.015 | accept |
| 0.005 | original | [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | compiled_initialization | 236/256 | 0.882–0.949 | inconclusive |
| 0.005 | original | [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | trained_backdoor | 256/256 | 0.985–1.000 | inconclusive |
| 0 | compiled_initialization | [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | approved | 2/256 | 0.002–0.028 | accept |
| 0 | compiled_initialization | [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | trained_backdoor | 30/256 | 0.083–0.162 | inconclusive |
| 0.005 | compiled_initialization | [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | approved | 3/256 | 0.004–0.034 | accept |
| 0.005 | compiled_initialization | [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | trained_backdoor | 17/256 | 0.042–0.104 | inconclusive |
| 0 | original | [1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | approved | 0/256 | 0.000–0.015 | accept |
| 0 | original | [1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | compiled_initialization | 121/256 | 0.412–0.534 | inconclusive |
| 0 | original | [1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | trained_backdoor | 237/256 | 0.887–0.952 | inconclusive |
| 0.005 | original | [1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | approved | 1/256 | 0.001–0.022 | accept |
| 0.005 | original | [1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | compiled_initialization | 141/256 | 0.490–0.611 | inconclusive |
| 0.005 | original | [1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | trained_backdoor | 233/256 | 0.869–0.939 | inconclusive |
| 0 | compiled_initialization | [1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | approved | 3/256 | 0.004–0.034 | accept |
| 0 | compiled_initialization | [1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | trained_backdoor | 28/256 | 0.077–0.154 | inconclusive |
| 0.005 | compiled_initialization | [1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | approved | 2/256 | 0.002–0.028 | accept |
| 0.005 | compiled_initialization | [1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] | trained_backdoor | 4/256 | 0.006–0.039 | inconclusive |

## Scope and reproducibility

Scanned 750 source QASM files. Exclusions: {'width_budget': 645, 'mid_circuit_measurement': 5, '"vqe_uccsd_n4.qasm:225,8: \'q\' is not defined in this scope"': 1, '"vqe_uccsd_n4_transpiled.qasm:242,8: \'q\' is not defined in this scope"': 1}. The inventory also retains optional compiler rewrites that failed the strict operator check; an excluded rewrite never excludes its eligible original. Compiler output wire permutations are explicitly restored before comparing operators and submitting native gates.

6 eligible source files are byte-identical to previously inspected Wildisley artifacts. Repositories contain related programs and pretranspiled counterparts; treat counts as a descriptive benchmark, not independent new programs or an unseen-population guarantee. All per-context results are retained; macro averages do not rescue a failing context.

Independently reconstructed 1,009,664 method decisions and all thresholds from raw samples. Compared 2,958 ideal probe distances with Qiskit; maximum absolute difference 1.4e-14. Cost: 705,024,000 simulated shots, zero QPU shots. These are SDK component tests; no full-runtime release rate or physical device security guarantee is inferred.

Sources: [QASMBench](https://github.com/pnnl/QASMBench), [VeriQBench](https://github.com/Veri-Q/Benchmark), [QDoor author toy implementation](https://github.com/chuchengc/QDoor), [QDoor paper](https://arxiv.org/abs/2307.09529). Pinned commits, archive hashes and source file hashes are in preparation_manifest.json. See [reproduction commands](../../README.md#false-positive-controls).
