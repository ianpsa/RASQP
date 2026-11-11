#!/usr/bin/env bash
# Run the Linux CI checks without GitHub Actions. No cloud jobs are submitted.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

if [[ "$(uname -s)" != Linux ]]; then
    echo 'Full local validation requires Linux for the seccomp experiment suite.' >&2
    exit 2
fi

external_python="${RASQP_EXTERNAL_PYTHON:-.venv/bin/python}"
ibm_python="${RASQP_IBM_PYTHON:-.venv-ibm/bin/python}"
for interpreter in "$external_python" "$ibm_python"; do
    "$interpreter" -c 'import pytest, qiskit, qiskit_aer'
done
"$ibm_python" -c 'import qiskit_ibm_runtime'

cargo_flags=(--locked)
if [[ "${RASQP_OFFLINE:-0}" == 1 ]]; then
    cargo_flags+=(--offline)
fi

cargo fmt --all --check
cargo clippy "${cargo_flags[@]}" --all-targets -- -D warnings
cargo test "${cargo_flags[@]}" --all-targets
cargo build "${cargo_flags[@]}"
cargo build "${cargo_flags[@]}" --release
cargo doc "${cargo_flags[@]}" --no-deps
cargo package "${cargo_flags[@]}"

mkdir -p results/ci
run_dir="$(mktemp -d "$PWD/results/ci/local.XXXXXX")"
target/release/rasqp-lab all "$run_dir"
"$external_python" -m pytest \
    research/wildisley/test_dataset.py \
    research/web_variants/test_variants.py \
    research/false_positives/test_controls.py -q
"$ibm_python" -m pytest research/ibm/test_ibm.py -q
"$external_python" audit/verify_evidence.py --output "$run_dir/numeric_evidence.json"
printf '\nLocal Linux validation passed. Fresh experiment evidence: %s\n' "$run_dir"
