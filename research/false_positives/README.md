# False positives and external paper artifacts

This separate exploratory study tests 98 eligible author-supplied unitary benchmark
circuits: 43 QASMBench and 55 VeriQBench, plus 92 operator-checked compiler rewrites.
It also replays the QDoor authors' clean, compiled-initialization and trained-backdoor
parameter artifacts on three explicitly specified feature vectors.

The [protocol](../../docs/FALSE_POSITIVE_PROTOCOL_V1.json) precedes its evaluation.
The [report](../../results/false-positives-v1/REPORT.md), raw count log, every circuit,
source hashes and original QASM are retained separately from older experiments.
No detector or threshold selection was changed. The Rust recorder gained an explicit
mixed-control schema; the original 1-clean/10-infected schema remains strict.

False positives require known clean controls. Previously published Trojan datasets
contain research-injected mutations, not artifacts recovered from an intrusion.
Published author artifacts provide external provenance, not hardware validation or
proof that every stored backdoor reproduces a paper's full attack success.

The noisy joint verifier alarms on 0.79% of unchanged original QASMBench decisions
and 0.92% of unchanged original VeriQBench decisions. Exact compiler rewrites scored
against original calibration cause 12.64% and 4.30% alarms, respectively; some individual
circuits cause 100%. Separately approved optimized contexts return to 0.88% and 0.92%.
This exposes the need to bind calibration to an approved raw implementation and its
noise context. It does not implement automatic approval or recalibration.

QDoor is an instructive failure: against original calibration, the intended benign
compiled initialization causes 613/768 alarms (79.82%), compared with 745/768 (97.01%)
for the author-labeled trained backdoor. Against separately approved compiled
initialization, backdoor alarms fall to 74/768 (9.64%). These fractions describe three
diagnostic feature contexts, each with 256 decisions; they are not MNIST attack-success
rates. The inspector returns inconclusive for both artifacts against the original,
and for the backdoor against compiled initialization. An exposed candidate can therefore
be withheld by the structural component even when its behavioral probes rarely alarm.

All 190 matched benchmark contexts satisfy the per-context Wilson95 upper-FPR <=10%
criterion in both noise settings. This loose criterion does not prove FPR <=1%, simultaneous
population bounds, calibration-to-calibration robustness or production suitability.
Macro rates describe related benchmark circuits; six original files are byte-identical
to earlier Wildisley inputs. Optional optimizer outputs that fail the strict 1e-9 operator
check are inventoried and excluded as rewrites while their original circuits remain.
Wire permutations emitted by Qiskit are explicitly materialized before comparison.

Replay on Linux using the standalone environment documented in
[the external-dataset harness](../wildisley/README.md):

```sh
curl -L --fail https://codeload.github.com/pnnl/QASMBench/tar.gz/357b942396d5c2b7cbc1c229c585a6ef5ccaebac -o /tmp/qasmbench.tar.gz
curl -L --fail https://codeload.github.com/Veri-Q/Benchmark/tar.gz/9be108313844e30c5df5780989a68cac4d966cfe -o /tmp/veriqbench.tar.gz
curl -L --fail https://codeload.github.com/chuchengc/QDoor/tar.gz/2ab751fc2c8dd858a3d2a82ede600c0b45ac4a67 -o /tmp/qdoor.tar.gz
cargo build --locked --release --bin rasqp --bin rasqp-external
.venv/bin/python research/false_positives/prepare.py --qasmbench-archive /tmp/qasmbench.tar.gz --veriqbench-archive /tmp/veriqbench.tar.gz --qdoor-archive /tmp/qdoor.tar.gz --output results/runs/fp-replay-input
target/release/rasqp-external results/runs/fp-replay-input results/runs/fp-replay docs/FALSE_POSITIVE_PROTOCOL_V1.json
.venv/bin/python research/false_positives/analyze.py --prepared results/runs/fp-replay-input --output results/runs/fp-replay
.venv/bin/python research/false_positives/validate_oracles.py --output results/runs/fp-replay
.venv/bin/python -m pytest research/false_positives/test_controls.py -q
```

Downloads must match pinned archive hashes before extraction. The QDoor tensor reader
extracts fixed-size little-endian float32 ZIP storage only after checking both the
checkpoint and static tensor-metadata hashes. It never calls `torch.load`, unpickles
data, executes upstream code or retrains models. Its 48-parameter U2/CNOT topology is
translated with a restricted AST reader; the 28-parameter original RX/RY/RZ/CRX block
is checked against the pinned source. QDoor's PCA inputs, model training, approximate
synthesis search, pulse execution and classification experiment are not reproduced.
The additional `oracle_validation.json` checks every distinct eight-qubit artifact
and all three probes against independently constructed Aer depolarization/readout
channels, in addition to ideal Qiskit checks on every recorded case.

These are **behavioral and structural SDK component tests**. Rejected and inconclusive
structural outcomes are separate from false-positive attack labels. No end-to-end
`GuardedRuntime` release rate, cloud provider job or real QPU was measured here.
