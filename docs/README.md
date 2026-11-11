# Documentation

## Build and implementation

- [Build and runtime requirements](../REQUIREMENTS.md)
- [Terminal notebook guide](TUI_GUIDE.md)
- [Rust SDK and integration reference](RUST_SDK.md)
- [Implementation migration and backend differences](MIGRATION_MATRIX.md)
- [Local validation and release delivery](LOCAL_VALIDATION.md)
- [Optional local Python reference archive](../legacy/README.md)

## Experiments and evidence

- [Research methods, results, and limits](RESEARCH_V03.md)
- [Archived experiment reports and observations](../results/README.md)
- [External-dataset harness](../research/wildisley/README.md)
- [Literature-derived circuit variants](../research/web_variants/README.md)
- [False-positive controls and author artifacts](../research/false_positives/README.md)
- [IBM calibration simulation and hardware-pilot protocol](../research/ibm/README.md)

## Frozen protocols

| Study | Protocol |
| --- | --- |
| Original hypothesis registry | [HYPOTHESIS_PROTOCOL.json](HYPOTHESIS_PROTOCOL.json) |
| Native Rust experiments | [NATIVE_PROTOCOL_V1.json](NATIVE_PROTOCOL_V1.json) |
| Complementary-probe extension | [EXTENSION_PROTOCOL_V03.json](EXTENSION_PROTOCOL_V03.json) |
| Context-conditioned follow-up | [EXTENSION_PROTOCOL_V04.json](EXTENSION_PROTOCOL_V04.json) |
| External dataset | [WILDISLEY_PROTOCOL_V1.json](WILDISLEY_PROTOCOL_V1.json) |
| Literature-derived variants | [WEB_VARIANTS_PROTOCOL_V1.json](WEB_VARIANTS_PROTOCOL_V1.json) |
| False-positive controls | [FALSE_POSITIVE_PROTOCOL_V1.json](FALSE_POSITIVE_PROTOCOL_V1.json) |
| IBM calibration simulation | [IBM_CALIBRATION_PROTOCOL_V1.json](IBM_CALIBRATION_PROTOCOL_V1.json) |
| IBM hardware-pilot plan | [IBM_HARDWARE_PILOT_V1.json](IBM_HARDWARE_PILOT_V1.json) |

Protocols remain JSON inputs. Raw observations, dependency manifests, validation
logs, and [historical checksums](manifests/README.md) retain their original formats.
