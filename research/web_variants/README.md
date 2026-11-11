# Literature-derived transfer tests and calibration leakage check

The new suite tests a frozen RASQP detector on 32 newly generated four-qubit
workloads (16 angle-encoding/variational and 16 QAOA MaxCut) and 320 circuit
interventions. It does not import infected Wildisley circuits or train a classifier.
The three probe choices, reference/calibration budgets, alpha, shot allocation and
detector source hashes remain those used in the preceding external study.

[Protocol](../../docs/WEB_VARIANTS_PROTOCOL_V1.json) and
[results](../../results/web-variants-v1/REPORT.md) identify the mechanisms:

- [SQUASH](https://arxiv.org/html/2506.24081v1): untargeted SWAP insertions.
- [QTrojan](https://arxiv.org/html/2302.08090v1): circuit-level input encoding
  sandwich; the [author notebook](https://github.com/chuchengc/Qtrojan/tree/bc8ae00fcf87ed30ed13ef04af4fdf1b8bdea8ea)
  was inspected as data, not executed.
- [Upadhyay and Ghosh](https://arxiv.org/html/2209.11872v1): multiplicative
  parameter tampering; the optimizer/cloud experiment is not reproduced.
- [Conditional Trojans](https://arxiv.org/html/2502.08880v1): activation motivates
  our mixed multi-control diagnostics, rather than supplying these exact fixtures.

These are **mechanism-level reimplementations**, not a newly published independent
infected dataset or complete reproductions of the papers' application experiments.
SWAP was already a known audit blind spot, and rotations/CX were used in prior
development. The study tests new compositions/parameterizations on fresh workloads;
it does not establish that the detector had never encountered the attack concepts.
Start-of-circuit SWAP and mixed-control stress cases are dormant on the tested
initial states. Their low alarm rates show a coverage boundary, not demonstrated
harm to the current zero-input application output. Active-input behavior is checked
separately; no application-level attack success is inferred from operator change alone.

The regression `infected_candidates_cannot_change_clean_calibration` in
`tests/external_dataset.rs` runs the actual CLI twice, retaining the same approved
golden circuit but replacing every infected candidate by a clean one. Calibration
counts, references and thresholds must be identical; evaluation flags must change.
This directly tests the separation between fitting and attacked observations.
It does not promise resilience if the supposedly trusted clean baseline is poisoned.

The prior 94%/80% rates belong to the RASQP distribution detector. Authors' archived
supervised-classifier predictions are a separate baseline; those classifiers were
trained on their training folds as expected. RASQP's fixed probes were originally
selected against toy X/Z/CX mutations, which is development exposure, even though
the Wildisley infected rows were not used to fit the external detector.

Reproduce from the repository root with the standalone dependencies already installed:

```sh
.venv/bin/python research/web_variants/prepare.py
cargo build --locked --release --bin rasqp-external
target/release/rasqp-external results/runs/web-variants-input results/runs/web-variants-reproduction docs/WEB_VARIANTS_PROTOCOL_V1.json
.venv/bin/python research/web_variants/analyze.py --output results/runs/web-variants-reproduction
.venv/bin/python -m pytest research/web_variants/test_variants.py research/wildisley/test_dataset.py -q
cargo test --locked --test external_dataset
```

Use a fresh output directory. All raw samples, conditions, seeds, thresholds and
per-variant failures are retained. Qiskit independently checks ideal probe distances;
the analysis recomputes the Rust detector decisions from raw counts. No QNN datasets,
downloaded notebook cells, pulse configurations or remote quantum hardware are run.
