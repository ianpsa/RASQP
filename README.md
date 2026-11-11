# RASQP

**Rust SDK and quantum assurance terminal notebook.**

RASQP studies checks for hybrid classical/quantum workflows: circuit inspection,
classical artifact fingerprints, signed request/result binding, replay controls,
and calibrated comparisons of measurement distributions. It is a research
prototype with explicit trust boundaries and negative findings.

Browse the [documentation](docs/README.md) and [experiment results](results/README.md)
for implementation guides, reproduction commands, protocols, and readable reports.

## Quick start

### Download the Linux executable

Download the [standalone Linux x86_64 notebook](https://github.com/ianpsa/RASQP/releases/download/v0.3.1/rasqp-tui-v0.3.1-linux-x86_64),
then make it executable and launch it:

```sh
chmod +x rasqp-tui-v0.3.1-linux-x86_64
./rasqp-tui-v0.3.1-linux-x86_64
```

The downloaded executables do not require Rust or Python. This release provides
Linux x86_64 binaries; Windows and macOS builds require separate validation.

The [release page](https://github.com/ianpsa/RASQP/releases/tag/v0.3.1) also offers
the other standalone command-line tools, SHA-256 checksums, a validation log,
and a full archive with executables, SDK source, and documentation.

### Build from source

The repository pins Rust **1.98.1**. Source builds also require a native C
compiler and linker for bundled SQLite. Cargo installs dependencies using
`Cargo.toml` and `Cargo.lock`; the native SDK and notebook do not require Python.

```sh
cargo build --locked --release
cargo run --locked --release --bin rasqp-tui
```

See [requirements and installation](REQUIREMENTS.md) for platform prerequisites.

## Interactive notebook

Explore circuit drawings, measurement distributions, live SDK experiments,
frozen studies, and a searchable glossary.

| Page | What it contains |
| --- | --- |
| Learn | Qubits, measurement views, and the project questions |
| Workbench | Nine live SDK experiments with measured results and uncertainty |
| Studies | Frozen native experiments, criteria, logs, and loaded evidence |
| Code map | Rust modules and JSON command-line operations |
| Glossary | Definitions, diagrams, and a labeled Bloch-sphere illustration |

Use a Unicode-capable monospace terminal. The minimum size is **60 × 18**;
**120 × 42** is recommended. Braille graphics, box-drawn circuits, and block bars
render directly in the terminal.

- **Navigate:** click tabs and topics, or use `Tab` / `1`–`5` and the arrow keys.
- **Scroll:** use the mouse wheel, drag the scrollbar, or press `PageUp` / `PageDown`.
- **Run:** click a button or press `Enter`.
- **Help / quit:** press `?` / `q`. Quitting stops an active laboratory child.

Workbench covers clean controls, bit/phase flips, dormant controlled gates,
small rotations, benign rewrites, drift, signed-response replay, and actual
`GuardedRuntime` execution. Change probes, noise, or shot budgets; inspect JSON;
and save educational evidence.

Studies save new runs under `results/runs/tui/`. Linux seccomp is the default;
other operating systems require an explicit unconfined selection. A standalone
notebook can run studies without Cargo or another RASQP executable. It never
submits physical IBM jobs.

### Preview without an interactive terminal

```sh
cargo run --locked --bin rasqp-tui -- --snapshot workbench --demo 2
cargo run --locked --bin rasqp-tui -- --summary results/native-v1/native_summary.json
```

Snapshots use the real renderer without terminal escape sequences. Workbench
demonstrations are separate from frozen research runs. The native summary retains
H41's failed criterion and H42/H44's bounded screening outcomes; registry IDs are
not presented as independently validated security claims.

See the [terminal guide](docs/TUI_GUIDE.md) for more detail.

## SDK and command-line tools

```sh
cargo run --locked --example runtime
cargo run --locked --example protect
target/release/rasqp --help
target/release/rasqp-lab all results/runs/my-run
```

| Executable | Purpose |
| --- | --- |
| `rasqp-tui` | Interactive terminal notebook |
| `rasqp` | JSON commands for inspection, simulation, scoring, calibration, and signed-result protection |
| `rasqp-lab` | Native experiment runner |
| `rasqp-external` | Evaluation of separately versioned external datasets |
| `rasqp-release` | Native release archive and checksum generation |

The optional Python validation harnesses under `research/` have their own pinned
requirements. They are separate from the terminal interface and native simulator.

## Repository structure

| Path | Contents |
| --- | --- |
| `src/` | Main Rust implementation: domain types, runtime guards, classical controls, simulation adapters, research engines, laboratory, and TUI |
| `examples/` | SDK usage examples |
| `tests/` | Rust integration tests and shared fixtures |
| `research/` | Optional Python study and validation harnesses |
| `docs/` | Implementation guides and frozen experiment protocols |
| `results/` | Archived observations and evaluation results |
| `audit/` | Independent evidence verification |
| `scripts/` | Local validation tooling |

## Validation and packaging

Run the native development checks:

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo doc --locked --no-deps
cargo package --locked
```

For the full local Linux CI equivalent, including native experiments and offline
Python harnesses, prepare the environments described in the
[local validation guide](docs/LOCAL_VALIDATION.md), then run:

```sh
bash scripts/check-local.sh
```

The GitHub release workflow calls CI before building Linux x86_64, Windows x86_64,
and macOS aarch64 archives. A version-matching tag triggers a prerelease. Archives
contain native executables, SDK source, documentation, checksums, and build metadata.
Each executable is also published as a separate download with its own checksum;
asset names identify the version, operating system, and architecture.
Cargo registry publication is disabled with `publish = false`.

When Actions is unavailable, the local validation guide explains direct release
uploads. A local Linux build provides validation and binaries for that platform.

## Scientific scope and evidence

All native quantum results are **local classical simulation**. A behavioral alarm
measures deviation, not malicious intent; benign drift can alarm. Public probes
can have blind spots and do not prove honest workload execution. Approved policies,
keys, baselines, and replay storage require an independently trusted host.
No real-QPU efficacy, universal protection, or quantum advantage is claimed.

The native full study records **129,200 circuit instances** and **77,388,288 sampled
shots**. Negative findings and frozen criteria remain part of the evidence.
Terminal illustrations explain mechanisms; measured plots use actual SDK output.

- [SDK reference](docs/RUST_SDK.md)
- [Research methods, results, and limits](docs/RESEARCH_V03.md)
- [Python-to-Rust migration and implementation differences](docs/MIGRATION_MATRIX.md)
- [Frozen protocols](docs/)
- [Archived results](results/)
