# Experiment results

The reports below describe bounded simulation studies. Their failed criteria,
negative controls, and limitations remain part of the evidence. They do not
establish real-QPU efficacy, universal protection, or quantum advantage.

| Study | Readable report | Structured results |
| --- | --- | --- |
| Native Rust suite | [Research interpretation](../docs/RESEARCH_V03.md), [full generated report](native-v1/native_report.md) | [Summary](native-v1/native_summary.json) |
| External dataset | [Report](wildisley-v1/REPORT.md) | [Evaluation](wildisley-v1/evaluation.json) |
| Literature-derived variants | [Report](web-variants-v1/REPORT.md) | [Evaluation](web-variants-v1/evaluation.json) |
| False-positive controls and QDoor artifacts | [Report](false-positives-v1/REPORT.md) | [Evaluation](false-positives-v1/evaluation.json) |
| IBM calibration simulation | [Report](ibm-calibration-v1/REPORT.md) | [Evaluation](ibm-calibration-v1/evaluation.json) |
| IBM hardware-pilot preflight | [Protocol and reproduction guide](../research/ibm/README.md) | [Plan](ibm-hardware-pilot-preflight-v1/plan.json) |

The preflight plan does not record physical QPU execution. Study directories also
contain the relevant protocol snapshots, raw observations, provenance, and validation
manifests. Older result snapshots remain in their original directories.

Documentation uses Markdown. Machine-readable JSON/JSONL, CSV, QASM, binary artifacts,
and raw validation logs retain their original contents and formats. Historical
manifests may refer to earlier `.txt` report names or the former `rust/` source layout;
they describe the files at evaluation time and are preserved verbatim.

New native runs write `native_report.md`; Python study analyzers write `REPORT.md`.
Use a fresh directory under `results/runs/` to preserve archived evidence.
