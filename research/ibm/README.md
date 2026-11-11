# IBM calibration tests and a bounded hardware pilot

The [calibration protocol](../../docs/IBM_CALIBRATION_PROTOCOL_V1.json) and
[results](../../results/ibm-calibration-v1/REPORT.md) describe **local simulation**
using the IBM Runtime package's archived Torino and Sherbrooke snapshots, dated
February 2025. They do not describe physical execution or current device conditions.
The separate [hardware protocol](../../docs/IBM_HARDWARE_PILOT_V1.json) defines a
prospective Open Plan pilot within the user's ten-minute allowance.

The detector stays fixed: original 0/Z, 1/Z and +/X probes, 768 evaluation shots,
alpha 0.01 and unchanged Rust `JointVerifier`. The new CLI `evaluate_joint` operation
calibrates and scores external counts using that existing verifier; Python independently
reconstructs each threshold and score. This does not add a quantum security guarantee
or an authenticated `GuardedRuntime` provider adapter.

## Completed calibration-based simulation

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

## Reproduce locally

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

## Connect to a physical IBM computer

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

## The ten-minute limit

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
