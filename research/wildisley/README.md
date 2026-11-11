# External evaluation using Wildisley Filho's dataset

Source: Wildisley José de Souza Filho, Victor Takashi Hayashi, and Bryan Kano
Ferreira, *Dataset e Análise de Sensibilidade de Trojans em Circuitos Quânticos*,
WTICG/SBSeg 2026, [doi:10.5753/sbseg_estendido.2026.29446](https://doi.org/10.5753/sbseg_estendido.2026.29446).
[Author repository](https://github.com/apenas-will/IC-QT-2026), pinned commit
`48ec00d02776e602e3336c8db66ea9fda8c59576`.

The [protocol](../../docs/WILDISLEY_PROTOCOL_V1.json) inventories the complete
535-golden/5,350-infected corpus. It evaluates every complete base group supported by
the bounded unitary native simulator, records all excluded groups, and preserves
author group/fold identifiers. It never regenerates replacement Trojans and calls
them the authors' dataset. Original QASM files are fetched on demand; their relative
paths and SHA-256 digests are recorded in the inventory.

The behavioral study compares fixed and complementary probes, jointly calibrated
and independent OR rules, and a Bonferroni-corrected OR. Every base/raw golden/noise
context receives its own reference and calibration. All evaluation cases use that
same approved context. The appended-X-pair control deliberately stresses a raw
context change, rather than claiming it was separately calibrated.

Authors' archived GroupKFold text predictions are reused and labeled as archived;
the harness does not retrain their classifiers. Complete base groups cannot cross
folds. Full-corpus and identical eligible-cohort classification metrics are reported
separately. RASQP's access to a golden circuit and new execution samples is a different
information setting from single-QASM classification.

The importer rejects mid-circuit measurements, reset, classical control and
over-budget circuits rather than erasing their effects. Terminal measurement maps
are retained, but the RASQP experiment measures the full quantum register in declared
bases. The declared noisy primitive decomposition differs from the authors' original
simulation configuration. This is a new external evaluation, not exact reproduction
of their sensitivity table.

Standalone dependencies and converter/native/Aer parity tests live here, outside the
ignored Python archive. To reproduce from the repository root:

```sh
uv pip install --python .venv/bin/python -r research/wildisley/requirements.lock.txt
mkdir -p results/runs
curl -fL https://codeload.github.com/apenas-will/IC-QT-2026/tar.gz/48ec00d02776e602e3336c8db66ea9fda8c59576 -o results/runs/wildisley-source.tar.gz
.venv/bin/python research/wildisley/prepare.py --archive results/runs/wildisley-source.tar.gz
cargo build --locked --release
target/release/rasqp-external results/runs/wildisley-input results/runs/wildisley-reproduction
.venv/bin/python research/wildisley/analyze.py --output results/runs/wildisley-reproduction
.venv/bin/python research/wildisley/validate_oracles.py --output results/runs/wildisley-reproduction
cargo build --locked
.venv/bin/python -m pytest research/wildisley/test_dataset.py -q
cargo test --locked --test external_dataset
```

Choose a fresh output directory for every run. The full evaluator applies Linux
seccomp and logs compressed raw counts, seeds, contexts, references and thresholds.
The analyzer independently recomputes all decisions, checks shot accounting, and
reports per-base macro averages and paired base-cluster bootstrap intervals instead
of treating every shot or related variant as an independent application.

The first full attempt stopped at a normalization work-budget error after 89 groups.
Its partial output was retained locally under
`results/runs/wildisley-interrupted-v1/`. The runner was corrected to record an
`analysis_error_withheld` structural outcome and continue behavioral evaluation.
No eligibility rule, probe, reference size, threshold, shot allocation or statistical
criterion was changed. Native-v1 results remain unchanged.
