"""Hardware-calibration-based local evaluation; never initializes a cloud service."""
import argparse
import collections
import gzip
import json
from pathlib import Path
import shutil
import sys

import numpy as np
from qiskit.quantum_info import Statevector

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT))
from research.ibm.common import *

PROTOCOL = ROOT/'docs/IBM_CALIBRATION_PROTOCOL_V1.json'


def run(output):
    from qiskit_ibm_runtime import fake_provider
    protocol = json.loads(PROTOCOL.read_text())
    frozen = detector_hashes()
    output = fresh(output)
    shutil.copyfile(PROTOCOL, output/'protocol.json')
    rng = np.random.default_rng(protocol['seed'])
    outcomes, contexts, shot_total = [], [], 0
    raw = gzip.open(output/'observations.jsonl.gz','wt')
    for backend_class in protocol['fake_backends']:
        backend = getattr(fake_provider, backend_class)()
        destination = output/backend.name
        destination.mkdir()
        write_json(destination/'properties.json', backend.properties().to_dict())
        write_json(destination/'configuration.json', backend.configuration().to_dict())
        for workload_index, relative in enumerate(protocol['workloads']):
            clean, source = unitary_source(relative)
            candidates = variants(clean)
            for layout_index, physical in enumerate(connected_paths(backend,clean.num_qubits)):
                context_id = f'{backend.name}-w{workload_index}-l{layout_index}'
                directory = destination/context_id; directory.mkdir()
                target = compact_target(backend,physical)
                model, model_dict, readout = restricted_noise(backend,physical)
                write_json(directory/'gate_noise.json',model_dict)
                write_json(directory/'readout.json',readout)
                distributions, mapping, circuits = {}, [], []
                ideal_clean=[Statevector(probe(clean,s,b).remove_final_measurements(inplace=False)).probabilities()
                             for s,b in protocol['probes']]
                ideal_distances={}
                for case,candidate in candidates.items():
                    probabilities = []
                    ideal_distances[case]=[]
                    for probe_index,(state,basis) in enumerate(protocol['probes']):
                        ideal=Statevector(probe(candidate,state,basis).remove_final_measurements(inplace=False)).probabilities()
                        ideal_distances[case].append(float(np.abs(ideal-ideal_clean[probe_index]).sum()/2))
                        isa, measurement_map = compile_probe(candidate,state,basis,target,protocol)
                        physical_probability, normalization_error = density_probabilities(isa,measurement_map,model,backend,physical)
                        p = logical_probabilities(physical_probability,readout,measurement_map)
                        probabilities.append(p)
                        if case == 'clean':
                            distributions.setdefault('benign_readout_drift_x2',[]).append(
                                logical_probabilities(physical_probability,readout,measurement_map,factor=2))
                        mapping.append({'circuit_index':len(circuits),'case':case,'probe':probe_index,
                                        'logical_to_compact_measurement':measurement_map,
                                        'operations':dict(isa.count_ops()),
                                        'density_normalization_error':normalization_error,
                                        'duration_seconds':isa.estimate_duration(target,unit='s')})
                        circuits.append(isa)
                    distributions[case] = probabilities
                qpy_write(directory/'isa.qpy',circuits)
                write_json(directory/'isa_index.json',mapping)
                write_json(directory/'distributions.json',distributions)
                context = {'id':context_id,'backend_snapshot':backend.name,'workload':relative,
                           'physical_qubits':physical,'snapshot_date':backend.properties().last_update_date.isoformat(),
                           'source':source,'omitted_cases':[v for v in protocol['cases'] if v not in distributions],
                           't1_seconds':[backend.qubit_properties(q).t1 for q in physical],
                           't2_seconds':[backend.qubit_properties(q).t2 for q in physical],
                           'readout_mean_assignment_error':[float((m[0,1]+m[1,0])/2) for m in readout],
                           'posthoc_ideal_probe_tvd':ideal_distances}
                contexts.append(context)
                for acquisition in range(protocol['calibration_acquisitions']):
                    seed = int(rng.integers(0,2**63-1)); draws = np.random.default_rng(seed)
                    reference_counts = [draws.multinomial(protocol['reference_shots_per_probe'],p).tolist()
                                        for p in distributions['clean']]
                    references = [(np.array(c)/sum(c)).tolist() for c in reference_counts]
                    cal = [[draws.multinomial(protocol['shots_per_probe'],p).tolist()
                            for p in distributions['clean']] for _ in range(protocol['calibration_repetitions'])]
                    observed = {case:[[draws.multinomial(protocol['shots_per_probe'],p).tolist() for p in ps]
                                      for _ in range(protocol['evaluation_repetitions'])]
                                for case,ps in distributions.items()}
                    cases = list(observed)
                    result = rust_evaluate(references,cal,[b for case in cases for b in observed[case]],protocol['alpha'])
                    recorded = {'context':context_id,'acquisition':acquisition,'seed':seed,
                                'reference_counts':reference_counts,'calibration':cal,'observed':observed,
                                'rust':result}
                    raw.write(json.dumps(recorded)+'\n'); raw.flush()
                    repetitions = protocol['evaluation_repetitions']
                    for index,case in enumerate(cases):
                        flags = result['rejected'][index*repetitions:(index+1)*repetitions]
                        row = {'context':context_id,'acquisition':acquisition,'case':case,**rate(flags),
                               'threshold':result['verifier']['threshold']}
                        row['clean_bound_pass'] = row['wilson95'][1] <= .10 if case == 'clean' else None
                        row['detection_bound_pass'] = row['wilson95'][0] >= .90 if case in ['x0','z0','cx01'] else None
                        outcomes.append(row)
                    shot_total += 3*protocol['reference_shots_per_probe'] + 3*protocol['shots_per_probe']*(
                        protocol['calibration_repetitions']+len(cases)*repetitions)
                print(f"Calibrated local evaluation: {context_id}, physical {physical}",flush=True)
    raw.close()
    assert frozen == detector_hashes()
    write_json(output/'summary.json',{'version':protocol['version'],'contexts':contexts,'outcomes':outcomes,
        'cost':{'sampled_shots':shot_total,'qpu_shots':0},'detector_sha256':frozen,
        'source_files_sha256':{p:sha(ROOT/p) for p in ['research/ibm/common.py','research/ibm/calibrated.py',
                                                     'docs/IBM_CALIBRATION_PROTOCOL_V1.json','src/main.rs']}})
    analyze(output)


def analyze(output):
    protocol = json.loads((output/'protocol.json').read_text())
    summary = json.loads((output/'summary.json').read_text())
    by_context = {c['id']:c for c in summary['contexts']}
    independently_checked = 0
    reproduced = []
    with gzip.open(output/'observations.jsonl.gz','rt') as handle:
        for line in handle:
            row = json.loads(line)
            references = [(np.array(c)/sum(c)).tolist() for c in row['reference_counts']]
            batches = [b for values in row['observed'].values() for b in values]
            assert all(len(b) == 3 and all(sum(c)==protocol['shots_per_probe'] for c in b)
                       for b in row['calibration']+batches)
            checked = rust_evaluate(references,row['calibration'],batches,protocol['alpha'])
            assert checked == row['rust']
            offset = 0
            for case,observed in row['observed'].items():
                n=len(observed); flags=checked['rejected'][offset:offset+n]; offset+=n
                reproduced.append((row['context'],row['acquisition'],case,rate(flags)))
                independently_checked += n
    for (context,acquisition,case,computed),expected in zip(reproduced,summary['outcomes'],strict=True):
        assert (context,acquisition,case)==(expected['context'],expected['acquisition'],expected['case'])
        assert all(expected[k]==v for k,v in computed.items())
    table = []
    for backend in sorted({c['backend_snapshot'] for c in summary['contexts']}):
        for case in protocol['cases']:
            selected = [r for r in summary['outcomes'] if by_context[r['context']]['backend_snapshot']==backend and r['case']==case]
            if not selected: continue
            table.append({'backend':backend,'case':case,'contexts_acquisitions':len(selected),
                          'alarms':sum(r['alarms'] for r in selected),'decisions':sum(r['decisions'] for r in selected),
                          'macro_alarm_rate':float(np.mean([r['rate'] for r in selected])),
                          'ideally_invisible_context_acquisitions':sum(max(by_context[r['context']][
                              'posthoc_ideal_probe_tvd'].get(case,[0.]))<1e-9 for r in selected) if case not in ['clean','benign_readout_drift_x2'] else None,
                          'worst_clean_wilson_upper':max(r['wilson95'][1] for r in selected) if case=='clean' else None,
                          'failed_clean_contexts':sum(not r['clean_bound_pass'] for r in selected) if case=='clean' else None,
                          'failed_strong_contexts':sum(not r['detection_bound_pass'] for r in selected) if case in ['x0','z0','cx01'] else None})
    write_json(output/'evaluation.json',{'independently_checked_decisions':independently_checked,'rates':table,
                                        'raw_sha256':sha(output/'observations.jsonl.gz'),
                                        'summary_sha256':sha(output/'summary.json')})
    report=['# IBM calibration snapshot evaluation','',
        '**This run is local simulation, not IBM QPU execution.** The stored IBM snapshots date from February 2025. They model device-calibrated gate noise, thermal relaxation, scheduled idling and asymmetric readout, but not general crosstalk, leakage or temporal hardware variation. Three separate reference/calibration sampling acquisitions share each fixed model; they are not three physical recalibrations.','',
        '| Backend snapshot | Case | Alarms / decisions | Mean alarm rate | Failing clean / strong context-acquisitions |',
        '|---|---|---:|---:|---:|']
    for r in table:
        failed=r['failed_clean_contexts'] if r['case']=='clean' else r['failed_strong_contexts']
        report.append(f"| {r['backend']} | {r['case']} | {r['alarms']}/{r['decisions']} | {r['macro_alarm_rate']:.2%} | {failed if failed is not None else 'diagnostic'} |")
    report.extend(['',f"{independently_checked:,} decisions independently reconstructed against the unchanged Rust JointVerifier. {summary['cost']['sampled_shots']:,} sampled simulator shots; zero hardware shots. Every source, compact physical mapping, scheduled ISA circuit and quantum/readout model is retained.",'',
        'Logical classical-bit mappings are preserved after routing. Clean references alone determine thresholds. No failed case is retuned. Topology-only path selection deliberately includes qubits of varying quality; the manifest shows per-qubit calibration and snapshot dates. The optimized equivalent control is evaluated against original raw calibration. Extra readout drift is benign by construction.','',
        'Post hoc ideal probe TVDs are retained to separate input/basis blind spots from sensitivity lost to noise. These additional diagnostics do not change the prespecified pass criteria. Different workloads and mutations prevent attributing differences from earlier dataset averages solely to hardware noise.','',
        'Separate probe outcomes do not prove that an untrusted provider executed the workload. Visible mutations may already be withheld by conservative structural inspection; this study measures measurement sensitivity, not end-to-end SDK release or detection of clandestine provider faults. A hardware pilot is separately specified in docs/IBM_HARDWARE_PILOT_V1.json.'])
    (output/'REPORT.txt').write_text('\n'.join(report)+'\n')
    print(json.dumps({'verified_decisions':independently_checked,'cost':summary['cost'],'rates':table},indent=2))


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--output',type=Path,default=ROOT/'results/ibm-calibration-v1')
    parser.add_argument('--analyze-only',action='store_true')
    args=parser.parse_args()
    if args.analyze_only: analyze(args.output)
    else: run(args.output)
