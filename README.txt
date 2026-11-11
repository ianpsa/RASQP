RASQP — Rust SDK and quantum assurance terminal notebook
=======================================================

RASQP studies checks for hybrid classical/quantum workflows: circuit inspection,
classical artifact fingerprints, signed request/result binding, replay controls,
and calibrated comparisons of measurement distributions. It is a research
prototype with explicit trust boundaries and negative findings.

Source: https://github.com/ianpsa/RASQP
Requirements and installation details: REQUIREMENTS.txt
Terminal guide: docs/TUI_GUIDE.txt
SDK reference: docs/RUST_SDK.txt
Research interpretation: docs/RESEARCH_V03.txt
Migration map: docs/MIGRATION_MATRIX.txt

Start the interactive notebook
------------------------------
    cargo run --locked --release --bin rasqp-tui

The repository pins Rust 1.98.1. Source builds also need a native C compiler
and linker for bundled SQLite. Cargo installs the Rust dependencies using
Cargo.toml and Cargo.lock. The native SDK/TUI does not require Python.

Use a Unicode-capable monospace terminal font. Minimum size: 60 x 18;
recommended size: 120 x 42. Braille graphics give 2 x 4 drawing dots per cell;
box-drawn circuits and eighth-block bars show finer detail in the terminal.

Click tabs, topics and buttons. The wheel scrolls the panel under the pointer;
click or drag the notebook scrollbar. Keyboard controls remain available:
Tab or 1–5 switches pages, arrows choose topics, PageUp/PageDown scrolls,
Enter runs, ? opens help, and q quits. Quitting stops an active laboratory child.

Pages:
- Learn: an introduction to qubits, measurement views and the project questions.
- Workbench: nine live SDK experiments with actual results and uncertainty.
- Studies: the frozen native research runner, criteria, logs and loaded evidence.
- Code map: Rust modules and every JSON command-line operation.
- Glossary: definitions, diagrams and a labeled Bloch-sphere illustration.

Experiment with clean controls, bit/phase flips, dormant controlled gates,
small rotations, benign rewrites, drift, signed-response replay and actual
GuardedRuntime execution. Change the probe, noise or shot budget with the
buttons, inspect JSON, and save the current educational evidence.

Studies run in a separate child and save unique directories under
results/runs/tui/. Linux seccomp is the default; other operating systems require
an explicit unconfined selection. A standalone TUI can run studies without
Cargo or another installed RASQP executable. It never submits physical IBM jobs.

Headless preview and existing evidence
-------------------------------------
    cargo run --locked --bin rasqp-tui -- --snapshot workbench --demo 2
    cargo run --locked --bin rasqp-tui -- --summary results/native-v1/native_summary.json

Snapshots use the real renderer without terminal escape sequences. Educational
Workbench examples are separate from frozen research runs. The loaded native
summary records H41's failed criterion and H42/H44's bounded screening outcomes;
the interface does not present all registry IDs as validated security claims.

Other native tools
------------------
    cargo build --locked --release
    cargo run --locked --example runtime
    cargo run --locked --example protect
    target/release/rasqp-lab all results/runs/my-run

rasqp accepts JSON commands for digests, normalization/traces, inspection,
probabilities, equivalence, QASM, alignment, offline probe selection,
individual/joint scoring and calibration, signed-result protection, and
statistical comparisons. Run rasqp --help for the operation list.

rasqp-external evaluates separately versioned external datasets. The optional
Python validation harnesses under research/ have their own pinned requirements.
They are not dependencies of the terminal interface or native simulator.

Check and package
-----------------
For the complete local Linux CI equivalent, including native experiments and
offline Python harnesses, see docs/LOCAL_VALIDATION.txt:

    bash scripts/check-local.sh

    cargo fmt --all --check
    cargo clippy --locked --all-targets -- -D warnings
    cargo test --locked --all-targets
    cargo doc --locked --no-deps
    cargo package --locked

The GitHub release workflow calls CI before building Linux x86_64, Windows
x86_64 and macOS aarch64 archives. A tag matching Cargo.toml, such as v0.3.1,
triggers a GitHub prerelease. Archives contain native executables, SDK source,
requirements, documentation, checksums and build metadata. Cargo registry
publication remains disabled by publish=false.

When Actions is unavailable, docs/LOCAL_VALIDATION.txt describes building and
uploading a release directly from the local host. A local Linux release does
not establish validation or provide binaries for Windows or macOS.

Scientific scope
----------------
All native quantum results are local classical simulation. A behavioral alarm
measures deviation, not malicious intent; benign drift can alarm. Public probes
can have blind spots and do not prove honest workload execution. Approved
policies, keys, baselines and replay storage require an independently trusted
host. No real-QPU efficacy, universal protection or quantum advantage is claimed.

See the frozen protocol JSON files under docs/, archived observations under
results/, and the SDK/research guides for methods and limits. The terminal
illustrations are explanations; measured plots come from the actual SDK output.
