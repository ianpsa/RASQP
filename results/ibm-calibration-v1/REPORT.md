# IBM calibration snapshot evaluation

**This run is local simulation, not IBM QPU execution.** The stored IBM snapshots date from February 2025. They model device-calibrated gate noise, thermal relaxation, scheduled idling and asymmetric readout, but not general crosstalk, leakage or temporal hardware variation. Three separate reference/calibration sampling acquisitions share each fixed model; they are not three physical recalibrations.

| Backend snapshot | Case | Alarms / decisions | Mean alarm rate | Failing clean / strong context-acquisitions |
|---|---|---:|---:|---:|
| fake_sherbrooke | clean | 64/6144 | 1.04% | 0 |
| fake_sherbrooke | equivalent_optimized | 42/4608 | 0.91% | diagnostic |
| fake_sherbrooke | x0 | 4619/6144 | 75.18% | 6 |
| fake_sherbrooke | z0 | 3100/6144 | 50.46% | 12 |
| fake_sherbrooke | cx01 | 5381/6144 | 87.58% | 3 |
| fake_sherbrooke | rz0_0.12 | 101/6144 | 1.64% | diagnostic |
| fake_sherbrooke | swap_end | 4148/6144 | 67.51% | diagnostic |
| fake_sherbrooke | parameter_scale_1.1 | 1943/6144 | 31.62% | diagnostic |
| fake_sherbrooke | benign_readout_drift_x2 | 1075/6144 | 17.50% | diagnostic |
| fake_torino | clean | 59/6144 | 0.96% | 0 |
| fake_torino | equivalent_optimized | 52/4608 | 1.13% | diagnostic |
| fake_torino | x0 | 4623/6144 | 75.24% | 6 |
| fake_torino | z0 | 3102/6144 | 50.49% | 12 |
| fake_torino | cx01 | 4641/6144 | 75.54% | 6 |
| fake_torino | rz0_0.12 | 96/6144 | 1.56% | diagnostic |
| fake_torino | swap_end | 3113/6144 | 50.67% | diagnostic |
| fake_torino | parameter_scale_1.1 | 2474/6144 | 40.27% | diagnostic |
| fake_torino | benign_readout_drift_x2 | 2548/6144 | 41.47% | diagnostic |

107,520 decisions independently reconstructed against the unchanged Rust JointVerifier. 102,592,512 sampled simulator shots; zero hardware shots. Every source, compact physical mapping, scheduled ISA circuit and quantum/readout model is retained.

Logical classical-bit mappings are preserved after routing. Clean references alone determine thresholds. No failed case is retuned. Topology-only path selection deliberately includes qubits of varying quality; the manifest shows per-qubit calibration and snapshot dates. The optimized equivalent control is evaluated against original raw calibration. Extra readout drift is benign by construction.

Post hoc ideal probe TVDs are retained to separate input/basis blind spots from sensitivity lost to noise. These additional diagnostics do not change the prespecified pass criteria. Different workloads and mutations prevent attributing differences from earlier dataset averages solely to hardware noise.

Separate probe outcomes do not prove that an untrusted provider executed the workload. Visible mutations may already be withheld by conservative structural inspection; this study measures measurement sensitivity, not end-to-end SDK release or detection of clandestine provider faults. A hardware pilot is separately specified in docs/IBM_HARDWARE_PILOT_V1.json.

## Post hoc interpretation

The topology-selected physical qubits have mean assignment errors from 0.63% to 25.73%. These include poor qubits; the study does not estimate the average quality of a modern processor or select layouts to obtain favorable outcomes.

Some strong mutations are intrinsically invisible to the three fixed preparations/bases. The post hoc ideal distributions show this before considering noise. Noise from added physical gates can itself raise alarms even when the logical ideal distributions agree. Consequently, these aggregate results must not be interpreted as evidence that noise alone caused all misses, or that an alarm establishes harmful application impact.

The parameter-scale diagnostic changes explicit RX/RY/RZ instructions only. The QFT and quantum-walk source programs have no such instructions and their parameter cases are unchanged controls. All original rows remain, while interpretation.json separately identifies operator-changing and ideally visible cases. Conditional descriptive rates do not replace the original criteria.
