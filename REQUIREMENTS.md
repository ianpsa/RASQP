# RASQP build and runtime requirements

## Native SDK, command-line tools and terminal interface

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

## Rust libraries

Cargo installs all dependencies listed in Cargo.toml and resolves them using
Cargo.lock. Do not install similarly named Python packages for the native TUI.

The terminal interface uses Ratatui 0.30.2 and Crossterm 0.29. The native SDK
also uses Serde/serde_json, SHA-256/HMAC/hex, bundled SQLite, num-complex,
rand/rand_chacha, thiserror, tar and flate2. Linux laboratory filtering uses
seccompiler and libc. tempfile is used by development tests.

## Interactive terminal

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

## Build and launch

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

## Laboratory execution

Linux seccomp is enabled by default for native study children. Environments
must permit installing that filter. On macOS/Windows, explicitly choose an
unconfined laboratory run: press u in Studies before Run, or pass
--unconfined to rasqp-lab. The interactive parent remains outside that filter.

The native studies use classical simulation. Memory grows exponentially with
circuit width; the small educational fixtures do not establish scalability.
No IBM account, QPU access or cloud credentials are needed for native runs.

## Optional independent Python research harnesses

These are separate from the Rust SDK/TUI:

- Python 3.12 and pip for the external-dataset and IBM offline validation jobs.
- External-dataset harness dependencies:

```sh
python -m pip install -r research/wildisley/requirements.lock.txt
```

- IBM calibration/hardware harness dependencies:

```sh
python -m pip install -r research/ibm/requirements.lock.txt
```

Use separate virtual environments for the two pinned harnesses. Physical IBM
execution additionally requires an independently configured provider account
and the hardware-pilot protocol; the TUI does not submit physical jobs.
The ignored legacy/python archive is optional and is not in release packages.

## Validation and source packaging

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
see docs/LOCAL_VALIDATION.md. Direct uploads require GitHub authentication.
