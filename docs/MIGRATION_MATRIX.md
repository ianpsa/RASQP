# Python → Rust migration map

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

## Intentional differences

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
[legacy instructions](../legacy/README.md). CI uses native Rust checks because the Python
archive is ignored and absent from fresh checkouts. GitHub is the configured source and prerelease destination. crates.io publication
is disabled; the native TUI does not substitute a remote IBM execution adapter.
