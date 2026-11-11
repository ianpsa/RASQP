# RASQP

**Rust SDK and quantum assurance terminal notebook.**

RASQP studies checks for hybrid classical/quantum workflows: circuit inspection,
classical artifact fingerprints, signed request/result binding, replay controls,
and calibrated comparisons of measurement distributions. It is a research
prototype with explicit trust boundaries and negative findings.

<a id="contents"></a>

## Contents

- [Quick start](#quick-start)
- [Repository structure](#repository-structure)
- [Build and runtime requirements](#requirements)
- [Terminal notebook guide](#terminal-guide)
- [SDK and command-line reference](#sdk-reference)
- [Experiment results](#experiment-results)
- [Research findings and limitations](#research-findings)
- [External dataset evaluation](#external-dataset)
- [Literature-derived variants](#literature-variants)
- [False-positive controls and author artifacts](#false-positive-controls)
- [IBM calibration simulation and hardware pilot](#ibm-studies)
- [Local validation and release delivery](#local-validation)
- [Implementation migration and backend differences](#migration)
- [Local Python reference archive](#legacy-python-reference)
- [Historical delivery manifests](#historical-manifests)
- [Frozen protocols](#frozen-protocols)

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

See [requirements and installation](#requirements) for platform prerequisites.

## Repository structure

| Path | Contents |
| --- | --- |
| `src/` | Main Rust implementation: domain types, runtime guards, classical controls, simulation adapters, research engines, laboratory, and TUI |
| `examples/` | SDK usage examples |
| `tests/` | Rust integration tests and shared fixtures |
| `research/` | Optional Python study and validation harnesses |
| `docs/` | Frozen experiment protocols and historical checksum manifests |
| `results/` | Archived observations and evaluation results |
| `audit/` | Independent evidence verification |
| `scripts/` | Local validation tooling |

<a id="requirements"></a>

## Build and runtime requirements

### Native SDK, command-line tools and terminal interface

- Rust 1.98 or newer, as declared by Cargo.toml. The repository pins Rust 1.98.1
  in rust-toolchain.toml, including the rustfmt and Clippy components.
- Cargo and rustup for source builds. Install the pinned toolchain with:

```sh
rustup toolchain install 1.98.1 --profile minimal --component rustfmt --component clippy
```

- A working native C compiler and linker. rusqlite builds its bundled SQLite:
  a separate SQLite installation is not required.
  Linux: GCC or Clang and the platform development/linker tools.
  macOS: Xcode Command Line Tools (xcode-select --install).
  Windows: Visual Studio Build Tools with the C++ workload and Windows SDK
  when using the MSVC Rust toolchain.
- Network access for the first Cargo dependency download; cached builds can
  use --offline. Cargo.lock records the dependency versions used by --locked.
- A writable build directory. Experiments and saved demonstration evidence
  need a writable working directory for results/runs/tui/.

### Rust libraries

Cargo installs all dependencies listed in Cargo.toml and resolves them using
Cargo.lock. Do not install similarly named Python packages for the native TUI.

The terminal interface uses Ratatui 0.30.2 and Crossterm 0.29. The native SDK
also uses Serde/serde_json, SHA-256/HMAC/hex, bundled SQLite, num-complex,
rand/rand_chacha, thiserror, tar and flate2. Linux laboratory filtering uses
seccompiler and libc. tempfile is used by development tests.

### Interactive terminal

- An interactive terminal attached to standard input and output.
- At least 60 columns by 18 rows; 120 columns by 42 rows is recommended.
- A monospace font covering Unicode box drawing, block elements and Braille
  patterns. The renderer uses 2 x 4 Braille dots per character cell and
  eighth-cell block fills. It does not require a separate ASCII-image tool,
  an image viewer, Sixel, Kitty graphics, Python, a GPU or a quantum computer.
- A color-capable terminal. True color displays the intended palette.
- Mouse reporting support for clicks, wheel scrolling and scrollbar dragging.
  Keyboard navigation remains available when a terminal reserves mouse input.
- Noninteractive environments can use --snapshot instead of an interactive UI:

```sh
cargo run --locked --bin rasqp-tui -- --snapshot workbench --demo 2
```

### Build and launch

From the repository root:

```sh
cargo build --locked --release
cargo run --locked --release --bin rasqp-tui
```

A downloaded native release executable does not need a Rust toolchain or C
compiler. Extract the whole platform archive, then run rasqp-tui (or
rasqp-tui.exe on Windows). Use --summary PATH to load recorded native evidence;
a standalone TUI can also create a new study without Cargo or a sibling lab
binary. Without a summary file, it shows an empty evidence state.

Use a native executable matching the operating system and processor. Official
release workflow targets are Linux x86_64, Windows x86_64 and macOS aarch64.
The Linux build uses the runner's normal system ABI; no portable static-musl
compatibility guarantee is made.

### Laboratory execution

Linux seccomp is enabled by default for native study children. Environments
must permit installing that filter. On macOS/Windows, explicitly choose an
unconfined laboratory run: press u in Studies before Run, or pass
--unconfined to rasqp-lab. The interactive parent remains outside that filter.

The native studies use classical simulation. Memory grows exponentially with
circuit width; the small educational fixtures do not establish scalability.
No IBM account, QPU access or cloud credentials are needed for native runs.

### Optional independent Python research harnesses

These are separate from the Rust SDK/TUI:

- Python 3.12 and pip for the external-dataset and IBM offline validation jobs.
- External-dataset harness dependencies:

```sh
python -m pip install -r research/external_dataset/requirements.lock.txt
```

- IBM calibration/hardware harness dependencies:

```sh
python -m pip install -r research/ibm/requirements.lock.txt
```

Use separate virtual environments for the two pinned harnesses. Physical IBM
execution additionally requires an independently configured provider account
and the hardware-pilot protocol; the TUI does not submit physical jobs.
The ignored legacy/python archive is optional and is not in release packages.

### Validation and source packaging

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo doc --locked --no-deps
cargo package --locked
```

For a local packaging check with uncommitted files only, add --allow-dirty.
GitHub release publication additionally requires Git push access to the
repository. A version-matching v* tag invokes .github/workflows/release.yml;
GitHub Actions runs CI, builds the platform archives and publishes a prerelease
using the repository-scoped GITHUB_TOKEN. crates.io publication is disabled.
For local Linux validation and direct release uploads without GitHub Actions,
see [local validation and release delivery](#local-validation). Direct uploads require GitHub authentication.

<a id="terminal-guide"></a>

## Terminal notebook guide

Launch the native Ratatui interface from the repository root:

```sh
cargo run --locked --bin rasqp-tui
# Faster full research runs after compilation:
cargo run --locked --release --bin rasqp-tui
```

Use a terminal with at least 60 columns and 18 rows; 120 × 42 is comfortable.
The interface uses Unicode circuit artwork and colored probability bars. It does
not need a graphics protocol, image viewer, Python runtime, or quantum hardware.

### Start with the guided tour

The five pages answer different questions:

| Page | What it explains or runs |
| --- | --- |
| Learn | Recipes, qubits, measurement views, calibration, the five manuscript questions and trust boundaries |
| Workbench | Nine small experiments calling the actual Rust simulator, verifier, signed-response gate and guarded runtime |
| Studies | All six native lab modes, their questions, frozen parameters, recorded outcomes, logs and summary JSON |
| Code map | Domain, runtime, classical controls, research, adapters, lab/release tools, and every JSON CLI operation |
| Glossary | Plain-language definitions, examples, and links to the relevant source paths |

The interface distinguishes educational demonstrations from frozen research runs.
It preserves failed criteria, blind spots and benign controls. The five manuscript
questions are retrospective organizing questions, not new preregistered hypotheses.

### High-resolution terminal illustrations

The graphics use Ratatui's native Braille canvas: each character cell holds a
2 × 4 grid of dots. This draws smoother curves and finer plots than one ASCII
symbol per cell, while keeping everything scrollable in the notebook.

- Learn and Glossary show a labeled, illustrative single-qubit Bloch sphere.
- Workbench draws a Braille stem plot from the actual approved probabilities
  and the first candidate trial's observed frequencies. The outcomes remain
  discrete; the renderer does not smooth or interpolate a probability curve.
- Probability bars use `█▉▊▋▌▍▎▏` for eighth-cell fill precision.
- Circuit diagrams use gate boxes, `●` controls, `⊕` targets, and continuous
  box-drawing wires. Longer recipes split into labeled panels so connections
  do not wrap across lines. `M` labels measurement.

Use a monospace terminal font with Unicode box-drawing, block and Braille
coverage. No external image renderer or terminal graphics protocol is needed.
The Bloch sphere is explicitly an illustration, separate from measured evidence.

### Mouse controls

Click a tab to change pages, a topic to open it, or a button at the bottom to
run an experiment, change its settings, inspect JSON, or save evidence. Disabled
buttons are dimmed. Click Help for the controls and Close to dismiss it.

The wheel scrolls the panel under the pointer: the notebook scrolls its text,
while the topic list scrolls independently without changing the open topic.
Click a visible topic after scrolling to open it. Click or drag the notebook
scrollbar to move through long explanations or raw JSON.

Mouse reporting is enabled only while the interactive interface is running and
is released on exit. Keyboard controls remain available; in terminals that
reserve mouse events, use the keys below.

### Keys

| Key | Action |
| --- | --- |
| Tab / Shift-Tab, Left / Right, 1–5 | Switch page |
| Up / Down, j / k | Select a topic or experiment |
| PageUp / PageDown, a / d | Scroll the notebook |
| Home / End | Jump to the beginning or end |
| / | Search the current page's titles, explanations and source references |
| Enter while searching | Finish editing the filter |
| Esc | Clear search or leave the raw JSON view |
| ? | Open help |
| q / Ctrl-C | Quit; an active lab child is stopped and reaped |

Workbench keys:

| Key | Action |
| --- | --- |
| Enter | Run clean calibration, then clean and candidate evaluations |
| b | Cycle input / basis: 0/Z, +/X, 1/Z, i/Y |
| n | Cycle depolarizing noise: 0, 0.005, 0.02 |
| s | Cycle shots per repetition: 256, 1024, 4096 |
| r | Rerun using a new seed |
| v | View raw JSON, including sampled counts, calibration, rewrites, QASM and gate outcomes |
| e | Save the current educational evidence to a unique JSON file under `results/runs/tui/demos/` |

Settings changes clear old results. Each example uses 79 clean calibration
repetitions, then 64 independent clean and 64 candidate repetitions. References
are exact density-matrix probabilities in the selected approved noise context.
The drift control uses candidate noise 0.03 above the approved value. Alpha is
0.05. These parameters are educational, separate from the frozen studies.

Cyan bars show approved probabilities; amber bars show the first candidate
trial's sampled frequencies. The TVD and alarm shown above the bars belong to
that first trial. The rates below use all 64 repetitions, with Wilson 95%
intervals. A benign candidate's alarms are false positives.

The drawings show the raw approved and candidate recipes. The selected probe
wraps both recipes with preparation and measurement gates. Bit strings put q0
on the right. The full-operator comparison is ideal; it does not assert equality
under gate noise. A behavioral alarm and the structural inspection decision
are separate evidence, and a quiet behavioral check alone does not release
counts through the protection gate.

Try these experiments:

1. **Clean control:** observe Bell outcomes 00/11 and finite sampling variation.
2. **Bit flip:** compare changed joint counts with a visible recipe edit.
3. **Phase flip:** run the default 0/Z view; press b, then Enter. The operator
   change invisible to the first view becomes observable in +/X.
4. **Dormant controlled flip:** compare 0/Z with 1/Z to activate the control.
5. **Small rotation:** vary shots, noise and basis to explore limited sensitivity.
6. **Benign rewrite:** H,H cancels ideally but adds physical noisy gates.
7. **Benign drift:** the recipe stays the same while the context changes.
8. **Signed response:** inspect actual first-use, replay and altered-count
   outcomes. The public demonstration key and memory ledger are local fixtures.
9. **Guarded execution:** inspect actual `GuardedRuntime` outcomes and submitted
   shots for an approved job and a visible edit. The artifact is a self-approved
   fingerprint of the running executable; optional separate probe jobs are off.

### Run the frozen native studies

Select Studies, choose `all`, `batch`, `hypotheses`, `trojans`, `extensions` or
`context`, then press Enter. The TUI launches an isolated lab-only instance of
itself using the existing `ResearchLab`, so a standalone executable can run
studies without Cargo or a sibling `rasqp-lab` binary.

On Linux the child opens outputs before applying the existing seccomp filter.
On other platforms, press u to explicitly choose unconfined execution before
running. The interactive parent remains outside the child filter. The UI never
launches physical IBM jobs.

Press x to cancel. Quitting also stops an active study. Partial files are
retained and previous loaded evidence stays visible if a run is canceled or
fails. A completed valid summary replaces the loaded evidence.

Each run gets a unique directory under `results/runs/tui/` containing:

- `native_protocol.json`: the exact native protocol;
- `native_trials.jsonl`: raw seeded observations;
- `native_summary.json`: study results and measured simulation cost;
- `native_report.md`: generated report;
- `runner.log`: child progress and errors.

These runs are ignored by Git and do not overwrite published evidence. H41,
H42 and H44 badges come from the loaded summary's actual boolean fields; a
missing study is reported as missing. They are scoped screening outcomes,
not universal security conclusions. Press v for the selected study's full
summary JSON and r to reload the same summary file.

By default the UI loads `results/native-v1/native_summary.json` relative to the
current working directory. Select another native summary when launching:

```sh
cargo run --locked --bin rasqp-tui -- --summary results/runs/my-run/native_summary.json
```

Missing or malformed summaries produce an actionable empty state. The study
runner remains available. Separate external-dataset and IBM summaries have
different schemas; use their own tools and reports rather than treating them
as native summaries.

### Preview without taking over a terminal

```sh
cargo run --locked --bin rasqp-tui -- --snapshot learn
cargo run --locked --bin rasqp-tui -- --snapshot workbench --demo 2 --width 120 --height 42
cargo run --locked --bin rasqp-tui -- --snapshot studies --summary results/native-v1/native_summary.json
```

Supported snapshot pages: `learn`, `workbench`, `studies`, `sdk`, `glossary`.
Workbench snapshots execute the selected demonstration (0–8). Snapshots use
the same renderer with Ratatui's headless backend and print plain text without
terminal escape sequences. They also support noninteractive CI and documentation.

### Checks

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
```

The tests cover phase and trigger blind spots, benign equivalence, clean-only
calibration, signed replay/tamper withholding, actual guarded execution,
navigation and stale-result clearing, mouse hit targets, independent wheel scrolling,
scrollbar dragging, compact/large layouts, malformed evidence,
CLI previews and the standalone sandboxed study-to-summary path.

<a id="sdk-reference"></a>

## SDK and command-line reference

The library, native simulator, research runners, reports, Linux sandbox and release
packager run in Rust. Python/Qiskit/Cirq remain independent reference implementations
and parity tests; no Python process is required by a Rust SDK or laboratory execution.
See the [migration map](#migration) for explicit language/backend differences.

### Build and use

```sh
cargo build --locked --release
cargo test --locked --all-targets
cargo run --locked --example runtime
cargo run --locked --example protect
cargo doc --locked --no-deps --open
target/release/rasqp-lab all results/runs/my-run
```

The laboratory enables Linux seccomp by default. On other operating systems, opt into
`--unconfined` to run trusted fixtures without that filter. There is no silent fallback.
The SDK itself runs on Linux, Windows and macOS; only Linux was executed locally.

`rasqp-lab` supports `all`, `batch`, `hypotheses`, `trojans`, `extensions`, and `context`.
It writes raw JSONL counts and decisions, JSON summaries, a protocol snapshot and a
Markdown report. Files open before sandbox installation. Existing output files in the
chosen directory are replaced: use a fresh directory to preserve an earlier experiment.

Rust modules are grouped by responsibility:

| Module | Main types |
|---|---|
| `domain` | `Circuit`, `Gate`, `Family`, `MirrorTrap`; executor/oracle traits |
| `application` | `GuardedRuntime<B,S>`, `Supervisor<S>`, `Inspector`, policies and result gates |
| `classical` | `TranscriptVerifier<S>`, `SqliteLedger`, `ArtifactFingerprint`, `FunctionGuard` |
| `research` | `Normalizer`, `Alignment`, `VariantCatalog`, `Lineage`, `ProbeSelector`, verifiers, statistics |
| `adapters` | `NativeBackend`, `Statevector`, `DensityMatrix`, `NativeExecutor`, `PassManager`, `OpenQasm`, `PauliSandwich` |
| `lab` | `ResearchLab`, `ProtocolRegistry`, `ResearchReport`, sandbox |

The SDK can be consumed as a local path dependency or extracted `.crate` source package.
The configured source repository is https://github.com/ianpsa/RASQP;
[Requirements](#requirements) documents build and terminal prerequisites. `publish=false` prevents accidental public crates.io publication. No project
redistribution license was invented. Rust ABI stability, a C ABI and a Python wheel are
not promised. The optional Python bridge is retained locally in the ignored
`legacy/python/src/rasqp/rust_sdk.py` archive.

#### Guarded execution API

The `Probabilities` CLI operation computes an offline simulator distribution for a
specified circuit. It does not guard a running application. `GuardedRuntime::execute`
is the application entry point: it checks the submitted circuit and artifact, creates
fresh local request IDs, executes the workload through a `QuantumBackend`, validates
the measurement shape and calibrated distribution, optionally runs a fixed portfolio
of challenge circuits, rechecks the artifact, and releases counts only on acceptance.
The application must consume `outcome.accepted_counts`, never a backend result directly.

The [working runtime example](examples/runtime.rs) builds circuits with Rust types;
no CLI JSON is needed. It uses `NativeBackend`, a local simulator. To use a quantum
provider, implement `QuantumBackend` so it submits the exact circuit, verifies the
provider connection and job identity, and converts results to canonical basis counts.
There is no real-QPU adapter in this repository yet. The adapter must own transport
authentication; local request nonces do not authenticate a remote provider's counts.

| Detection | SDK method | When it runs | What it establishes |
|---|---|---|---|
| Circuit/resource change | `inspect`, called by `GuardedRuntime::execute` | Before submission | Rejects width/gate-budget violations; unproven semantic differences are inconclusive. |
| Classical file change | `ArtifactFingerprint::measure`, called before and after execution | Every guarded run | Detects changed on-disk bytes relative to a trusted fingerprint, within file check/use limits. |
| Workload measurement anomaly | `CalibratedVerifier::score` and `rejects` | Every guarded run | Flags total-variation distance beyond a calibrated reference threshold. |
| Challenge measurement anomaly | `ProbePlan` plus `JointVerifier::score` and `rejects` | Only when probes are configured | Flags the largest deviation in an ordered portfolio of separate challenge jobs. |
| Local request reuse | `NonceStore::claim`, normally `SqliteLedger` | Every guarded workload and challenge | Prevents reuse of locally generated request IDs in the same trusted ledger. |
| Signed transcript replay | `transcript::verify`, called by `Supervisor::execute` or `protect` | Separate signed-response integration | Verifies HMAC binding and consumes a nonce, only when a trusted signer actually produced the HMAC. |
| Function pointer/snapshot change | `FunctionGuard` | Only when explicitly called | Compares a pointer and caller-supplied byte snapshot; it is not loaded-code attestation. |
| Mutation/probe/Pauli studies | `Mutation`, `ProbeSelector`, `PauliSandwich`, `ResearchLab` | Offline research unless integrated by an application | Develops and evaluates candidate checks; these do not run automatically in `GuardedRuntime`. |

An anomaly alarm is statistical evidence, not a proof of attack. Separate challenge
jobs cannot prove that an adaptive provider executed the workload honestly, and a
provider that fabricates plausible counts may pass these checks. This runtime protects
the application's measurement consumption within its configured, trusted scope.

#### Signed-response API

```rust
use rasqp::{Circuit, Gate, SqliteLedger};
use rasqp::application::guard::Supervisor;

// In an application, supply a trusted Policy and a key from a secret store.
// let mut supervisor = Supervisor::new(policy, key, SqliteLedger::open("nonces.db")?)?;
// let outcome = supervisor.execute(&approved, &submitted, &request, &response)?;
// Only outcome.accepted_counts may be passed to the consumer.
let circuit = Circuit::new(2, vec![Gate::h(0), Gate::cx(0, 1)])?;
# Ok::<(), rasqp::Error>(())
```

`examples/protect.rs` is a complete working example of signed-response verification. It uses a test key and demonstration
nonce and writes `demo.sqlite`; applications need fresh unpredictable nonces and random
keys with at least 32 bytes. `guard::protect_batch` checks the entire ordered batch before
releasing any counts. A rejected batch can consume a prefix of nonces; retries need fresh
requests. Batch checking does not prove a hidden relation to an unverified workload.

#### Native CLI

```sh
echo '{"operation":"digest","circuit":{"width":1,"gates":[]}}' | target/release/rasqp
```

One JSON command goes to stdin and one result to stdout. Invalid input exits 2 and writes
JSON with `accepted_counts:null` to stderr. Reject/inconclusive are valid outcomes and
exit 0: inspect `decision`. Input is limited to 8 MiB.

Operations: digest, normalize, normalize_trace, inspect, score, joint_score, protect,
protect_batch, probabilities, equivalent, qasm, align, select_probes, paired, holm, evaluate_joint.
Schemas are the tagged `Command` enum in `src/main.rs`; complete integration examples
are in the local `legacy/python/integration/test_sdk.py` archive. Protection operations take an explicit ledger pathname
as the positional argument and read `RASQP_KEY_HEX` from the trusted caller's environment.
The native service API avoids subprocess environment transport for secrets.

### Security and numerical contracts

* Gates: x,h,z,rx,ry,rz,cx; width 1–16; at most 4096 gates. Angles are finite;
  nonrotation angles must be zero. Normalization aborts after a million scanned positions.
* Normalization uses conservative rewrites, with trace offsets and original resource
  counts. Different normal forms mean inconclusive unless a trusted oracle resolves them.
* Counts use q0 as the least significant bit, length 2^width, nonnegative u64 values,
  checked sum equal to shots, and a maximum total of 2^53.
* HMAC covers every request field and count. SQLite WAL/FULL and a unique nonce key
  provide atomic claims across local processes and persistence across restart. Invalid
  authentication does not burn a nonce; authentic rejected results do. SQLite URI
  interpretation is disabled. Deletion or rollback of the database still permits replay.
* Context mismatch is inconclusive; artifact mismatch rejects. No rejected/inconclusive
  outcome exposes counts. Policies, approved circuits and expected requests come from
  the trusted supervisor, never copied from provider claims.
* In the signed-response API, the application must commit ordered probes, shots,
  thresholds, references, approved raw variants, backend epoch and artifact manifest
  into `context_digest` with its fixed encoding; that API compares supplied identifiers.
  `GuardedRuntime` instead computes a local digest of its full `RuntimePolicy` and
  remeasures the configured file. Neither API infers a provider's actual calibration.
* `ArtifactFingerprint::measure` hashes actual file bytes. It does not attest loaded
  machine code or remove check/use races. `FunctionGuard` checks a function identity and
  a supplied code snapshot, not arbitrary mutable state or Python closures.
* Individual/joint TVD calibration requires exchangeable clean batches and fixed probe,
  shot and backend context. A statistical alarm does not identify malicious intent.

Statevectors allow up to 16 qubits; full operators and density matrices are capped at 8.
Their exponential memory cost is explicit. Native sampling uses ChaCha8 with recorded
seeds for reproducibility; never derive production keys/nonces from experiment seeds.
The exact noisy engine implements the declared local depolarizing/readout channel, not
an IBM hardware calibration. Native gate optimization is not Qiskit's transpiler.

### Wire format v1

Unsigned integers are little-endian. Text is u64 UTF-8 byte length then bytes. Digests
are lowercase SHA-256 hex. Angles use little-endian IEEE binary64, canonical positive
zero for either signed zero, and no NaN/infinity.

Circuit: ASCII `RASQP-CIRCUIT`, bytes 0 and 1, u8 width, u64 gate count, then each gate:
u8 opcode, u8 arity, arity u8 qubits, f64 angle. Opcodes 0–6 are x,h,z,rx,ry,rz,cx.

Response: ASCII `RASQP-RESPONSE`, bytes 0 and 1, length-prefixed job_id, nonce,
circuit_digest, backend, artifact_digest, context_digest, then u8 width, u64 shots,
u64 count length, each u64 count. HMAC-SHA256 covers all bytes.

Frozen vectors: `tests/fixtures/wire_vectors.json` and
`tests/fixtures/response_vector.json`. Legacy Python
JSON digests remain unchanged for archival reproduction and are incompatible with v1.
No implicit downgrade or dual acceptance exists.

### CI and GitHub releases

CI checks formatting, Clippy, tests, native release builds, docs and source packaging on
Linux/Windows/macOS. Linux runs all native experiments under seccomp. Comparisons with
Qiskit/Cirq and the original Python tests are local checks using the ignored archive;
see [legacy instructions](#legacy-python-reference).
The native runtime, lab and package generation do not require Python.

The tag workflow calls CI, checks version/platform agreement, creates native `.tar.gz`
archives with SDK source and SHA-256 files, then publishes a GitHub prerelease. It uses
the repository's `GITHUB_TOKEN`; only the publish job has contents write. There
is no public registry upload or paid artifact-attestation dependency. Checksums and build
metadata are not independent cryptographic signing.

For a local archive:

```sh
cargo build --locked --release
cargo package --locked --allow-dirty
target/release/rasqp-release
```

CI uses a clean checkout without `--allow-dirty`. Push a tag matching Cargo.toml
(`v0.3.1`) to the configured GitHub repository to invoke the prerelease workflow.
The terminal interface is documented in [terminal guide](#terminal-guide); build prerequisites
are in the root [requirements](#requirements). See GitHub Actions for remote job status.

A compromised supervisor/key store/consumer remains outside the guarantee. Rust memory
safety does not prevent a privileged attacker from replacing binaries or bypassing checks.
No QPU execution, quantum authentication protocol, blind computation, machine-code
attestation or quantum advantage is claimed.

<a id="experiment-results"></a>

## Experiment results

The reports below describe bounded simulation studies. Their failed criteria,
negative controls, and limitations remain part of the evidence. They do not
establish real-QPU efficacy, universal protection, or quantum advantage.

| Study | Readable report | Structured results |
| --- | --- | --- |
| Native Rust suite | [Research interpretation](#research-findings), [full generated report](results/native-v1/native_report.md) | [Summary](results/native-v1/native_summary.json) |
| External dataset | [Report](results/external-dataset-v1/REPORT.md) | [Evaluation](results/external-dataset-v1/evaluation.json) |
| Literature-derived variants | [Report](results/web-variants-v1/REPORT.md) | [Evaluation](results/web-variants-v1/evaluation.json) |
| False-positive controls and QDoor artifacts | [Report](results/false-positives-v1/REPORT.md) | [Evaluation](results/false-positives-v1/evaluation.json) |
| IBM calibration simulation | [Report](results/ibm-calibration-v1/REPORT.md) | [Evaluation](results/ibm-calibration-v1/evaluation.json) |
| IBM hardware-pilot preflight | [Protocol and reproduction guide](#ibm-studies) | [Plan](results/ibm-hardware-pilot-preflight-v1/plan.json) |

The preflight plan does not record physical QPU execution. Study directories also
contain the relevant protocol snapshots, raw observations, provenance, and validation
manifests. Older result snapshots remain in their original directories.

Documentation uses Markdown. Machine-readable JSON/JSONL, CSV, QASM, binary artifacts,
and raw validation logs retain their original contents and formats. Historical
manifests may refer to earlier `.txt` report names or the former `rust/` source layout;
they describe the files at evaluation time and are preserved verbatim.

New native runs write `native_report.md`; Python study analyzers write `REPORT.md`.
Use a fresh directory under `results/runs/` to preserve archived evidence.

<a id="research-findings"></a>

## Research findings and limitations

This work follows the draft RASQP paper's bounded hypothesis: layered controls may
mitigate specified hybrid-workflow threats; quantum properties do not eliminate classical
tampering. Earlier H01–H38 claims and data were retained.

### Implementation outcome

The project now has a Rust SDK, native scientific engines, experiment runners, reporting,
Linux filtering and release packaging. A persistent ledger, versioned encoding, circuit
and artifact/context binding, resource limits and individual/joint result gates are
implemented. The migration is functional, with explicit differences in RNG, compiler
optimization and language-specific hook inspection: see the [migration map](#migration).

The original Python regression suite passed, along with new portfolio tests. Rust
integration tests compare wire vectors, HMAC, process races, normalization/trace/alignment,
policy decisions, statistics, OpenQASM and native simulators against Python/Qiskit/Cirq.
Sixty random mixed-gate circuits (1–4 qubits) were checked against both independent ideal
engines and Aer's noisy quantum probabilities with an independent readout matrix.
Tolerances are 1e-12 ideal and 1e-11 noisy. These finite tests are not a simulator proof.

### Versioned studies and negative results

`EXTENSION_PROTOCOL_V03.json` was fixed before its first run. The failed noise conditions
motivated `EXTENSION_PROTOCOL_V04.json`, fixed before that follow-up. The follow-up is
exploratory and data-informed, not an externally preregistered independent confirmation.
No threshold in v03 was changed after seeing its results. Native execution has its own
`NATIVE_PROTOCOL_V1.json`, different random streams and explicitly named backend changes.

| Hypothesis | Tested extension | Outcome and boundary |
|---|---|---|
| H39 | Atomic durable replay claims | One accepted submission in a 16-process race, none after restart; database deletion/rollback excluded. |
| H40 | Bind classical artifacts, circuit, context and response | Declared substitutions withhold counts; stale calibration is inconclusive. A signer falsely claiming the old executable hash remains a counterexample. |
| H41 | Training-selected complementary probes at equal evaluation shots | Strong coverage improved, but the first protocol failed the noisy equivalent-variant false-alarm criterion. Retained failure. |
| H42 | Joint maximum-score calibration versus per-probe OR | Reduces false alarms in observed noisy runs. Python v03 failed its overall criterion; the separate native stream passed. A single calibration draw is not a conditional FPR guarantee. |
| H43 | Compose orthogonal controls without partial result release | Circuit/resource, transcript, artifact and behavioral fixtures are withheld; batch gate tests forbid partial release. Integration evidence, not universal superiority or a new cryptographic protocol. |
| H44 | Context-conditioned portfolio contract | Follow-up candidate met its fixed strong-attack and clean-FPR criteria in all six noise/approved-variant conditions. Ablations do not show that noisy references are necessary. |

### Native measured results

The full run is recorded in `results/native-v1/`: **129,200 circuit instances and
77,388,288 sampled shots**, including references and calibrations. It contains **24,192
main-batch decisions**, **6,480 Trojan decisions**, 360 normalization variants with
360 preserved operators/recognized normal forms, and 390/390 restored mutation reversals.
The Python atlas had 396 interventions: native benchmark RNG changes the lengths of
basis circuits and hence the number of distinct insertion positions. This difference
is documented, not silently described as identical data.

The original Python batches were rerun separately in `results/v03-rerun/`, including
29,419 circuit instances and 20,747,776 shots for the original hypothesis suite. Original
files under `results/` and the earlier reports were not overwritten.

For native H41, the selected portfolio was 0/Z, 1/Z and +/X, trained only on visible
two-qubit X, Z and CX mutations. Three-qubit holdouts include X, Z, CX, XZ and CXZ:

| Condition | Strong attacks detected | Clean alarms | Equivalent X-pair alarms |
|---|---:|---:|---:|
| Ideal, joint portfolio | 128/128 for each family | 0/128 | 0/128 |
| Noise 0.005, joint portfolio | 128/128 for each family | 4/128 | 14/128 |
| Noise 0.005, naïve OR | 128/128 for each family | 11/128 | 34/128 |

The selected single i/Y probe missed the combined XZ mutation in the ideal condition
(0/128); the portfolio caught 128/128. This is a fixture-specific benefit of complementary
observability, not evidence that portfolios universally dominate single probes. The
equivalent-variant result still fails H41's FPR rule.

H44 uses four-qubit holdouts, two independently approved raw variants (identity and
X3-X3), 511 calibration batches, 8192 reference shots per probe and 768 evaluation shots
per decision, with a candidate alpha of 0.01. References and variant approval are trusted
inputs; the runtime does not choose a calibration based on a hidden attack label.

| Noise | Approved variant | Clean alarms / 256 | Strong detections | Weak RZ(0.12) / 256 |
|---|---|---:|---:|---:|
| 0 | Identity | 0 | 256/256 each | 147 |
| 0 | X3-X3 | 0 | 256/256 each | 143 |
| 0.005 | Identity | 2 | 256/256 each | 1 |
| 0.005 | X3-X3 | 1 | 256/256 each | 3 |
| 0.01 | Identity | 2 | 256/256 each | 14 |
| 0.01 | X3-X3 | 3 | 256/256 each | 5 |

Each condition meets Wilson95 lower TPR ≥90% for every strong family and upper FPR ≤10%.
The ideal-reference alpha=0.01 ablation also meets the criteria and sometimes has fewer
clean alarms. Therefore the data support feasibility of the configured contract, **not
superiority of noisy-reference acquisition**. The larger calibration budget and stricter
alpha prevent attributing the cross-study improvement to one component.

At noise 0.005/0.01, weak rotations are mostly missed. Benign drift causes 256/256 alarms
in all H44 conditions; it must trigger quarantine/recalibration rather than a malicious
label. A provider returning the known reference distribution is accepted in almost all
cases: the observed alarms are at roughly clean false-alarm rates, not evidence of
successful detection of fabrication.

### Candidate contribution to the paper

The defensible RASQP-specific concept is an **execution-context assurance contract**:
an independently approved raw circuit and classical artifact manifest, bounded resource
cost, versioned probe/calibration policy, authenticated request/result association and
durable nonce history jointly determine whether a trusted supervisor releases results.
The complementary-probe experiment illustrates why a composition can expose a combined
mutation that a single training-optimal probe misses; the failures show why the contract
must retain the raw implementation and calibration context.

This is an implemented architectural and experimental contribution candidate. It is not
yet an established new scientific algorithm. Pauli checks and quantum metamorphic tests
already have substantial prior art: [Gonzales et al., Pauli Check Sandwiching](https://arxiv.org/abs/2206.00215)
and [Paltenghi and Pradel, MorphQ](https://arxiv.org/abs/2206.01111). A recent directly related
work is [Testing and Verification of Quantum Compilers through Assurance Contracts and Evidence](https://arxiv.org/abs/2610.00255).
These sources preclude claiming that checks, metamorphic relations or assurance contracts
were invented here. The novelty assessment needs a systematic comparison with such work.

Suggested paper wording: “In a bounded simulation study, a context-conditioned portfolio
combined with classical binding and replay controls met predefined detection and clean
false-alarm criteria for specified mutations. Weak perturbations, benign drift and
adaptive fabricated responses reveal remaining limitations. The results demonstrate
prototype feasibility rather than general security, hardware efficacy or quantum advantage.”

### Reproduction and interpretation

```sh
cargo build --locked --release
target/release/rasqp-lab all results/reproduction-native
```

Use `all` to reproduce the archived native stream: standalone modes start from the base
seed rather than consuming earlier studies' samples. Every sample's seed and circuit is
logged. Wilson intervals are per condition, not simultaneous population guarantees.
McNemar comparisons use the declared pairing and Holm families. Runtime property tests
do not use p-values. Matrix oracles and exact probabilities are classical computation,
not free quantum resources. Cache reuse affects local simulator timing; no QPU latency,
queue, financial cost or speedup can be inferred.

The recorded local validation was on Linux. CI additionally targets Windows and macOS.
Current delivery workflows are configured at https://github.com/ianpsa/RASQP; consult
GitHub Actions for remote validation status. Delivery changes do not alter frozen criteria
or the archived observations described above.

<a id="external-dataset"></a>

## External dataset evaluation

The integration uses `external_dataset` in Python and `external-dataset` in result paths. The frozen protocol retains its original version identifier and bytes so archived protocol hashes remain valid. Historical manifests and overlap-field names keep their recorded identifiers.

Source: Wildisley José de Souza Filho, Victor Takashi Hayashi, and Bryan Kano
Ferreira, *Dataset e Análise de Sensibilidade de Trojans em Circuitos Quânticos*,
WTICG/SBSeg 2026, [doi:10.5753/sbseg_estendido.2026.29446](https://doi.org/10.5753/sbseg_estendido.2026.29446).
[Author repository](https://github.com/apenas-will/IC-QT-2026), pinned commit
`48ec00d02776e602e3336c8db66ea9fda8c59576`.

The [protocol](docs/EXTERNAL_DATASET_PROTOCOL_V1.json) inventories the complete
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
uv pip install --python .venv/bin/python -r research/external_dataset/requirements.lock.txt
mkdir -p results/runs
curl -fL https://codeload.github.com/apenas-will/IC-QT-2026/tar.gz/48ec00d02776e602e3336c8db66ea9fda8c59576 -o results/runs/external-dataset-source.tar.gz
.venv/bin/python research/external_dataset/prepare.py --archive results/runs/external-dataset-source.tar.gz
cargo build --locked --release
target/release/rasqp-external results/runs/external-dataset-input results/runs/external-dataset-reproduction
.venv/bin/python research/external_dataset/analyze.py --output results/runs/external-dataset-reproduction
.venv/bin/python research/external_dataset/validate_oracles.py --output results/runs/external-dataset-reproduction
cargo build --locked
.venv/bin/python -m pytest research/external_dataset/test_dataset.py -q
cargo test --locked --test external_dataset
```

Choose a fresh output directory for every run. The full evaluator applies Linux
seccomp and logs compressed raw counts, seeds, contexts, references and thresholds.
The analyzer independently recomputes all decisions, checks shot accounting, and
reports per-base macro averages and paired base-cluster bootstrap intervals instead
of treating every shot or related variant as an independent application.

The first full attempt stopped at a normalization work-budget error after 89 groups.
Its partial output remains in the local interrupted-run archive. The runner was corrected to record an
`analysis_error_withheld` structural outcome and continue behavioral evaluation.
No eligibility rule, probe, reference size, threshold, shot allocation or statistical
criterion was changed. Native-v1 results remain unchanged.

<a id="literature-variants"></a>

## Literature-derived variants

The new suite tests a frozen RASQP detector on 32 newly generated four-qubit
workloads (16 angle-encoding/variational and 16 QAOA MaxCut) and 320 circuit
interventions. It does not import infected Wildisley circuits or train a classifier.
The three probe choices, reference/calibration budgets, alpha, shot allocation and
detector source hashes remain those used in the preceding external study.

[Protocol](docs/WEB_VARIANTS_PROTOCOL_V1.json) and
[results](results/web-variants-v1/REPORT.md) identify the mechanisms:

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
.venv/bin/python -m pytest research/web_variants/test_variants.py research/external_dataset/test_dataset.py -q
cargo test --locked --test external_dataset
```

Use a fresh output directory. All raw samples, conditions, seeds, thresholds and
per-variant failures are retained. Qiskit independently checks ideal probe distances;
the analysis recomputes the Rust detector decisions from raw counts. No QNN datasets,
downloaded notebook cells, pulse configurations or remote quantum hardware are run.

<a id="false-positive-controls"></a>

## False-positive controls and author artifacts

This separate exploratory study tests 98 eligible author-supplied unitary benchmark
circuits: 43 QASMBench and 55 VeriQBench, plus 92 operator-checked compiler rewrites.
It also replays the QDoor authors' clean, compiled-initialization and trained-backdoor
parameter artifacts on three explicitly specified feature vectors.

The [protocol](docs/FALSE_POSITIVE_PROTOCOL_V1.json) precedes its evaluation.
The [report](results/false-positives-v1/REPORT.md), raw count log, every circuit,
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
[the external-dataset harness](#external-dataset):

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

<a id="ibm-studies"></a>

## IBM calibration simulation and hardware pilot

The [calibration protocol](docs/IBM_CALIBRATION_PROTOCOL_V1.json) and
[results](results/ibm-calibration-v1/REPORT.md) describe **local simulation**
using the IBM Runtime package's archived Torino and Sherbrooke snapshots, dated
February 2025. They do not describe physical execution or current device conditions.
The separate [hardware protocol](docs/IBM_HARDWARE_PILOT_V1.json) defines a
prospective Open Plan pilot within the user's ten-minute allowance.

The detector stays fixed: original 0/Z, 1/Z and +/X probes, 768 evaluation shots,
alpha 0.01 and unchanged Rust `JointVerifier`. The new CLI `evaluate_joint` operation
calibrates and scores external counts using that existing verifier; Python independently
reconstructs each threshold and score. This does not add a quantum security guarantee
or an authenticated `GuardedRuntime` provider adapter.

### Completed calibration-based simulation

Four previously inspected QASMBench unitary blocks are tested on each of two disjoint
topology-selected layouts per backend, with three independent reference/calibration
sampling acquisitions: 16 physical-layout/workload contexts and 48 sampling acquisitions.
These are repeated draws from fixed saved models, not independently measured hardware
recalibrations. References have 8192 shots/probe, calibration has 511 batches and each
case has 256 evaluation decisions. Each circuit has separate gate-duration scheduling,
idle delays, backend-derived thermal/depolarizing channels and asymmetric assignment
probabilities taken directly from per-qubit calibration. Virtual RZ gates are not
assigned an invented depolarization probability.
Idle relaxation uses `min(T2, 2*T1)` to keep the channel physically admissible when
estimated calibration values exceed the Markovian bound; raw T1/T2 values are retained.

All logical output-wire mappings are preserved through compilation and compact
physical-subgraph remapping. Tests compare remapped channels with a full five-qubit
backend calculation and independently check asymmetric readout. Finite Kraus
serialization errors below 1e-7 are recorded before normalization; larger errors stop
the run. Crosstalk, leakage and arbitrary non-Markovian effects are not modeled.

The run records 102,592,512 simulated shots and independently reconstructs 107,520
decisions. Clean alarms average 0.96% for Torino and 1.04% for Sherbrooke. Not every
strong mutation satisfies the per-context sensitivity criterion. Post hoc ideal probe
TVDs distinguish intrinsically invisible interventions from noisy loss of sensitivity;
they do not change the criteria. Different workload cohorts prevent attributing a
change from earlier detection averages solely to the noise model.
The additional post hoc interpretation is reproduced with
`.venv-ibm/bin/python research/ibm/interpret.py --output <run-directory>`.
It flags the unchanged parameter-scaling cases in sources without explicit RX/RY/RZ
instructions, while retaining every original row and prespecified failure.

The initial numerical-validation run is preserved locally in
`results/runs/ibm-calibration-initial-validation/`; the subsequent run retains identical
measurement counts and adds explicitly labeled ideal-probe diagnostics. Every older
published protocol and result remains separate.

### Reproduce locally

IBM dependencies live in `.venv-ibm`, separate from the older research environment.
`python-dateutil==4.0.0` in the old lock conflicts with IBM Cloud SDK constraints, so
the IBM environment uses its own pinned dependency closure.

```sh
uv venv .venv-ibm --python python3.12
uv pip install --python .venv-ibm/bin/python -r research/ibm/requirements.lock.txt
cargo build --locked --release --bin rasqp
.venv-ibm/bin/python -m pytest research/ibm/test_ibm.py -q
.venv-ibm/bin/python research/ibm/calibrated.py --output results/runs/ibm-calibration-replay
```

The pre-existing pinned QASMBench artifacts in `results/false-positives-v1/sources/`
are required for the full run, but the offline unit tests need no credentials or
external data download. Reanalysis uses `--analyze-only`. Source hashes, full device
properties/configuration, restricted quantum channels, asymmetric readout matrices,
compact scheduled ISA QPY, distributions and all sampled counts are retained.

### Connect to a physical IBM computer

1. Obtain an API key from the [IBM Quantum dashboard](https://quantum.cloud.ibm.com/)
   and ensure the account has an Open Plan instance. This version refuses paid plans.
2. In the local IDE terminal, run:

   ```sh
   .venv-ibm/bin/python research/ibm/hardware.py configure
   .venv-ibm/bin/python research/ibm/hardware.py check
   ```

   Enter the key at the hidden prompt. It is saved only in `.ibm/accounts.json`,
   ignored by Git and restricted to the local user. Do not put the key in chat,
   command arguments, source code, result artifacts or CI. `QISKIT_IBM_TOKEN` can
   alternatively supply an existing environment secret. Configuration and connection
   checks submit no quantum job.
3. Prepare the live backend plan, optionally selecting a backend by name:

   ```sh
   .venv-ibm/bin/python research/ibm/hardware.py prepare --output results/runs/ibm-live
   ```

   Review `plan.json`, which contains the actual target compilation, current calibration,
   physical layout, shot total and conservative usage estimate. The stored fake-backend
   preflight in `results/ibm-hardware-pilot-preflight-v1/` estimates about 270 seconds,
   **not a live usage measurement**. Live estimates will differ. Preparing a plan
   submits no QPU job. Mock plans are technically prevented from submitting hardware.
4. Submit and collect the three stages in order:

   ```sh
   .venv-ibm/bin/python research/ibm/hardware.py submit --output results/runs/ibm-live --stage reference
   .venv-ibm/bin/python research/ibm/hardware.py collect --output results/runs/ibm-live --stage reference
   .venv-ibm/bin/python research/ibm/hardware.py submit --output results/runs/ibm-live --stage calibration
   .venv-ibm/bin/python research/ibm/hardware.py collect --output results/runs/ibm-live --stage calibration
   .venv-ibm/bin/python research/ibm/hardware.py submit --output results/runs/ibm-live --stage evaluation
   .venv-ibm/bin/python research/ibm/hardware.py collect --output results/runs/ibm-live --stage evaluation
   .venv-ibm/bin/python research/ibm/hardware.py analyze --output results/runs/ibm-live
   ```

   `collect` checks status and returns promptly while queued/running; repeat only
   **collection**, never submission. A following stage is unavailable until the previous
   results are retained. No retry, session reservation or duplicate submission occurs
   automatically. An ambiguous submission leaves a durable intent; inspect the IBM
   dashboard and reconcile its job ID before any further action. Never delete its
   submission journal just to retry.

### The ten-minute limit

The fixed pilot uses two workloads, five cases each, three probes, 127 calibration
batches and 64 evaluation decisions, totaling **735,744 shots**. Runtime caps are
60 seconds for reference, 120 for calibration and 240 for evaluation: **420 seconds
total**, reserving three of the user's ten minutes. Only three submission attempts
are permitted. A conservative estimate also reserves two seconds per PUB and uses
the backend's default repetition delay plus the scheduled circuit duration. A plan
exceeding any stage cap or the total ceiling is refused.

Shot count alone cannot guarantee QPU usage or billing. The provider enforces the
per-job limits; actual usage/metrics are retrieved, and failed/canceled jobs may consume
allowance. Queue waiting is different from execution time. This pilot neither checks
nor replenishes the user's remaining quota. IBM's live remaining allowance must still
cover the run. Do not replace a failed job automatically within this protocol.

Hardware PUBs are randomized in order within each stage. To reduce overhead, repeated
observations use the actual ordered bitstrings from a single PUB per distinct circuit,
partitioned into 256-shot batches. Probes are consequently sequential: the test assumes
stationarity across their acquisition and does not claim independently interleaved
physical decisions. Acquisition-time calibration snapshots, job IDs, exact logical
measurement width, all raw bitstrings and usage are retained. Dry-run/mock output is
never analyzed as genuine quantum measurements. No physical result is claimed until
authenticated IBM jobs have actually completed.
Job-associated calibration properties and snapshots immediately before submission and
after retrieval are retained separately. Pending usage accounting is recorded as
unknown, rather than interpreted as zero QPU consumption.

Primary documentation: [noise models](https://quantum.cloud.ibm.com/docs/en/guides/build-noise-models),
[local testing](https://quantum.cloud.ibm.com/docs/en/guides/local-testing-mode),
[credentials](https://quantum.cloud.ibm.com/docs/en/guides/save-credentials),
[SamplerV2](https://quantum.cloud.ibm.com/docs/en/api/qiskit-ibm-runtime/sampler-v2),
[usage](https://quantum.cloud.ibm.com/docs/en/guides/estimate-job-run-time).

<a id="local-validation"></a>

## Local validation and release delivery

### Run the Linux checks without GitHub Actions

From the repository root, prepare separate Python 3.12 environments once:

```sh
python3.12 -m venv .venv
.venv/bin/python -m pip install -r research/external_dataset/requirements.lock.txt
python3.12 -m venv .venv-ibm
.venv-ibm/bin/python -m pip install -r research/ibm/requirements.lock.txt
```

Then, with a clean tracked working tree:

```sh
bash scripts/check-local.sh
```

This runs formatting, Clippy, all Rust test targets, debug/release builds,
API documentation, source-package verification, the full native study under
Linux seccomp, the external-dataset harness, and the offline IBM tests. It
independently audits the archived native statistics. Fresh study output goes
to a unique ignored results/ci/local.* directory; frozen results are preserved.
It does not submit QPU jobs or require IBM credentials.

Set RASQP_OFFLINE=1 to use cached Cargo dependencies. Override interpreter
paths with RASQP_EXTERNAL_PYTHON and RASQP_IBM_PYTHON if needed. Source-package
verification requires committed source changes. The host must allow seccomp.
These are Linux checks; Windows and macOS require separate validation.

### Build and publish the current platform package

After validation, package the committed source and record its commit:

```sh
GITHUB_SHA="$(git rev-parse HEAD)" target/release/rasqp-release
cd dist
sha256sum -c rasqp-v0.3.1-linux-x86_64.tar.gz.sha256
sha256sum -c rasqp-tui-v0.3.1-linux-x86_64.sha256
cd ..
```

The version-matching annotated tag must point to that same commit:

```sh
git tag -a v0.3.1 -m 'RASQP v0.3.1: local validation and release delivery'
git push origin trunk refs/tags/v0.3.1
gh auth login
gh release create v0.3.1 \
    dist/rasqp*-v0.3.1-linux-x86_64* \
    --repo ianpsa/RASQP --verify-tag --prerelease \
    --title 'RASQP v0.3.1 (research SDK)' --generate-notes
```

Use the current Cargo version for subsequent releases. Never move an existing
release tag. The packager emits the full archive plus standalone rasqp,
rasqp-tui, rasqp-lab and rasqp-external executables, each with a .sha256 file.
Standalone asset names include the version and platform, plus .exe on Windows.
On Linux/macOS, mark a directly downloaded file executable with chmod +x
before running it; the archive already preserves executable permissions.
A local x86_64 Linux build produces assets for Linux x86_64. They are uploaded
directly with gh and do not depend on an Actions run. Existing hosted workflows remain available once
the account's billing lock is resolved; local delivery does not clear it.

### Repository scope

Version code, build configuration, implementation/reproduction guides,
experiment protocols, audit tools, and archived observations. Manuscripts,
reference papers and presentations remain local and are ignored by Git.
Removing a tracked file with git rm --cached preserves its working copy.
Earlier commits and release tags retain their historical trees.

<a id="migration"></a>

## Implementation migration and backend differences

Rust is the default implementation. Python files are retained locally under the ignored
`legacy/python/` directory as independent regression oracles and the original study
archive. Python paths below are relative to that directory. The native CLI, lab and release packager do not
spawn Python. This is a functional port with explicit backend/language differences, not
a claim that Python, Qiskit or Cirq themselves were rewritten.

| Original Python responsibility | Rust replacement |
|---|---|
| domain.py | domain/circuit.rs, domain traits; validated constructors and versioned encoding |
| application.py | application/guard.rs, Supervisor, single/batch gates |
| classical.py | classical/transcript.rs, MemoryLedger baseline, SqliteLedger, TranscriptVerifier; classical/artifact.rs |
| verification.py | research/verification.rs and statistics.rs |
| circuits.py | domain/workload.rs, Family and MirrorTrap |
| attacks.py | research/attacks.rs, Mutation |
| obfuscation.py | research/obfuscation.rs, WorkloadTransforms |
| hooks.py | classical/hooks.rs, FunctionGuard; supplied code snapshot/address instead of Python introspection |
| research/normalization.py | research/normalization.rs, Normalizer, Rewrite, Normalized, Resources |
| research/genome.py | research/genome.rs, Alignment, VariantCatalog, Stage, Lineage |
| research/probes.py, portfolio.py | research/probes.rs, ProbeSelector; verification/JointVerifier |
| research/policy.py, service.py | research/policy.rs, application/inspection.rs and guard.rs |
| research/statistics.py | research/statistics.rs; Wilson, exact binomial/McNemar, Holm |
| adapters/ibm.py | adapters/simulator.rs, native statevector/density sampling; qasm.rs exports provider-compatible IR |
| adapters/cirq_check.py | independent statevector vs density tests, plus retained Cirq integration oracle |
| adapters/malicious_pass.py | adapters/compiler.rs, CircuitPass, InjectGatePass, PassManager |
| adapters/research_checks.py | adapters/checks.rs, restricted PauliSandwich and conditional terminal reset |
| scripts/run_experiments.py | lab/experiment.rs batch and lab/supplemental.rs |
| scripts/run_trojans.py | ResearchLab::trojans |
| scripts/run_hypotheses.py | ResearchLab::hypotheses and hypothesis_supplements |
| scripts/run_extensions.py, run_context_extension.py | ResearchLab::extension/context_extension |
| scripts/define_hypotheses.py | lab/protocol.rs, immutable embedded ProtocolRegistry |
| scripts/build_report.py, build_hypothesis_report.py | lab/report.rs, ResearchReport; native schema and Markdown renderer |
| scripts/sandbox.py | lab/sandbox.rs and rasqp-lab entrypoint |
| scripts/package_release.py | rasqp-release, native archive/checksum generation |
| rust_sdk.py | optional compatibility client only; native callers use structs/traits directly |

Rust paths in the table are relative to `src/`. `tests/native_research.rs`,
`tests/rust_runtime.rs`, and the local `legacy/python/integration/test_sdk.py` cover the
migrated behavior. Shared wire vectors remain versioned in `tests/fixtures/`.

The active implementation moved from `rust/` to `src/`. Archived results and
checksum manifests retain the paths recorded at evaluation time. Detector
identity checks read the corresponding files under `src/` while preserving
the archived keys, so existing preparation manifests and hardware plans remain
comparable without rewriting historical evidence.

### Intentional differences

1. MT19937/NumPy/Aer RNG streams are replaced by a versioned ChaCha8 stream. Generated
   variational benchmark parameters and sampled counts differ. Matching seeds do not
   imply matching circuit or shot data. Parity tests serialize the same circuits.
2. Native noisy simulation was checked against Aer on the supported gate alphabet.
   Y/T/CZ fixtures are lowered to XZ/RZ/H-CX-H. Ideal operators agree up to global phase;
   the number of noisy primitive operations can differ from native backend gates.
3. The local native compiler uses declared normalization rules. It does not reproduce
   Qiskit synthesis/layout/optimization level 3, IBM routing or backend scheduling.
4. Dynamic checking supports the original fixture's terminal flag measurements and
   conditional data reset. It is not a general dynamic-circuit interpreter. Flags remain
   observable and are never erased to manufacture acceptance.
5. Safe Rust has no Python bytecode or monkey-patching mechanism. The port tests function
   replacement, a supplied byte snapshot, passive copying and explicit supervisor bypass.
   It makes no claim to detect all runtime instrumentation.
6. Report formatting and storage use a unified native JSONL/JSON/Markdown schema. Original
   Python reports and raw evidence remain untouched. A protocol registry reader replaces
   mutable regeneration of an already evaluated hypothesis registry.
7. The native Linux sandbox denies network, exec and new writable file opens and common
   mutations. It is not a filesystem read jail or VM and does not port Python's rlimits;
   native circuit/alignment/work budgets provide separate bounds. macOS/Windows require
   an explicit unconfined laboratory flag; runtime verification remains cross-platform.

The original Python tests remain available as local external validation; see
[legacy instructions](#legacy-python-reference). CI uses native Rust checks because the Python
archive is ignored and absent from fresh checkouts. GitHub is the configured source and prerelease destination. crates.io publication
is disabled; the native TUI does not substitute a remote IBM execution adapter.

<a id="legacy-python-reference"></a>

## Local Python reference archive

`legacy/python/` contains the former Python SDK, simulation scripts, tests, integration suite,
dependency files, and original README. It is preserved locally and ignored by Git;
fresh checkouts and release packages do not contain it. Keep a separate backup if you
need historical Python reproduction. The active SDK and automated CI use Rust.

When the archive is available, run these commands from the repository root:

```sh
uv venv --python 3.12 .venv
uv pip install --python .venv/bin/python -r legacy/python/requirements.lock.txt
uv pip install --python .venv/bin/python --no-deps -e legacy/python
cargo build --locked
.venv/bin/python -m pytest -c legacy/python/pyproject.toml legacy/python/integration -q
.venv/bin/python legacy/python/scripts/sandbox.py test
```

The sandbox switches to `legacy/python/`, resolves its source and tests there, and
stores default rerun outputs locally. Shared evaluated protocols are read from `docs/`.
An explicit sandbox output directory is resolved relative to `legacy/python/`; use an
absolute path when selecting a different output location. The integration suite uses
the root `target/debug/rasqp` binary and versioned vectors in `tests/fixtures/`.

Historical reports are in `docs/reports/`, checksums in `docs/manifests/`, and the loose
original execution logs in ignored `results/logs/`. Regenerated Python reports stay in
the local archive. `legacy/python/README.md` preserves historical commands and path names;
use the commands above for the current layout. Python hypothesis generation writes a
local `legacy/python/docs/` copy rather than replacing the evaluated protocol registry.

<a id="historical-manifests"></a>

## Historical delivery manifests

These checksum files are preserved verbatim as records of earlier deliveries. Their
paths refer to the layout at that time; they are not manifests of the current tree.

The active Rust implementation lives in `src/`. Documentation and generated reports
now use Markdown; manifests can still record the original `.txt` names and `rust/`
paths. The optional Python reference archive, manuscripts, and presentation material
remain local. These historical manifests are not rewritten during documentation
or source-layout changes.

See the [current experiment reports](#experiment-results) and their per-study
validation manifests for archived observations. The native release packager generates
checksums for each new archive and standalone executable.

<a id="frozen-protocols"></a>

## Frozen protocols

| Study | Protocol |
| --- | --- |
| Original hypothesis registry | [HYPOTHESIS_PROTOCOL.json](docs/HYPOTHESIS_PROTOCOL.json) |
| Native Rust experiments | [NATIVE_PROTOCOL_V1.json](docs/NATIVE_PROTOCOL_V1.json) |
| Complementary-probe extension | [EXTENSION_PROTOCOL_V03.json](docs/EXTENSION_PROTOCOL_V03.json) |
| Context-conditioned follow-up | [EXTENSION_PROTOCOL_V04.json](docs/EXTENSION_PROTOCOL_V04.json) |
| External dataset | [EXTERNAL_DATASET_PROTOCOL_V1.json](docs/EXTERNAL_DATASET_PROTOCOL_V1.json) |
| Literature-derived variants | [WEB_VARIANTS_PROTOCOL_V1.json](docs/WEB_VARIANTS_PROTOCOL_V1.json) |
| False-positive controls | [FALSE_POSITIVE_PROTOCOL_V1.json](docs/FALSE_POSITIVE_PROTOCOL_V1.json) |
| IBM calibration simulation | [IBM_CALIBRATION_PROTOCOL_V1.json](docs/IBM_CALIBRATION_PROTOCOL_V1.json) |
| IBM hardware-pilot plan | [IBM_HARDWARE_PILOT_V1.json](docs/IBM_HARDWARE_PILOT_V1.json) |

Protocols remain JSON inputs. Raw observations, dependency manifests, validation
logs, and [historical checksums](#historical-manifests) retain their original formats.
