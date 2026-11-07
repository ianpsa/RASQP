"""Verify frozen-detector challenge counts; report every variant, including misses."""
import argparse
import collections
import hashlib
import json
from pathlib import Path
import sys

import numpy as np

ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT))
from research.wildisley.analyze import audit_raw
from research.web_variants.prepare import detector_hashes


def analyze(prepared,output):
    protocol=json.loads((output/'protocol.json').read_text())
    preparation=json.loads((prepared/'preparation.json').read_text())
    assert detector_hashes()==preparation['detector_sha256'], 'detector retuning during evaluation'
    summary=json.loads((output/'native_summary.json').read_text())
    counters,outcomes=audit_raw(output/'native_trials.jsonl.gz',protocol)
    assert outcomes==summary['outcomes']
    for key in ('shots','circuits','executor_calls'):
        assert counters[key]==summary['cost'][key]
    groups=[json.loads(line) for line in (prepared/'input.jsonl').open()]
    index={s['sample_id']:s for g in groups for s in g['samples']}
    parity=0.0
    for row in outcomes:
        if row['case']=='equivalent_x_pair':continue
        expected=index[row['sample_id']]['ideal_probe_tvd_qiskit']
        error=max(abs(a-b) for a,b in zip(expected,row['ideal_probe_tvd']))
        parity=max(parity,error)
        assert error<1e-10,(row['sample_id'],error)
    rows=[]
    for noise in protocol['noise']:
        for family in ('encoding_variational','qaoa_maxcut'):
            selected=[r for r in outcomes if r['context']['noise']==noise and r['mode']==family]
            clean={r['context']['base_circuit_id']:r for r in selected if r['case']=='clean'}
            assert len(clean)==16
            for variant in protocol['variants']['both_workloads']+protocol['variants'][family]:
                members=[r for r in selected if r['case']=='infected' and r['gate']==variant]
                assert len(members)==16
                effective=[r for r in members if index[r['sample_id']]['operator_changed']]
                methods={}
                for method in protocol['methods']:
                    mean=float(np.mean([r['methods'][method]['rate'] for r in members]))
                    alarms=float(np.mean([r['methods'][method]['rate'] for r in clean.values()]))
                    bad_clean=sum(r['methods'][method]['wilson95'][1]>.10 for r in clean.values())
                    methods[method]={'macro_detection':mean,'macro_clean_fpr':alarms,
                        'contexts_failing_clean_bound':bad_clean,'detection_requirement_pass':mean>=.90,
                        'combined_requirement_pass':mean>=.90 and bad_clean==0,
                        'operator_changed_detection':float(np.mean([r['methods'][method]['rate'] for r in effective])) if effective else None}
                rows.append({'noise':noise,'workload':family,'variant':variant,'bases':len(members),
                    'operator_changed':len(effective),'ideal_invisible':sum(max(r['ideal_probe_tvd'])<1e-10 for r in members),
                    'methods':methods})
    aggregates=[]
    for noise in protocol['noise']:
        for variant in sorted({row['variant'] for row in rows}):
            values=[row for row in rows if row['noise']==noise and row['variant']==variant]
            total=sum(row['bases'] for row in values)
            aggregates.append({'noise':noise,'variant':variant,'base_contexts':total,
                'joint_detection':sum(row['bases']*row['methods']['portfolio_joint']['macro_detection'] for row in values)/total,
                'fixed_detection':sum(row['bases']*row['methods']['fixed_0Z']['macro_detection'] for row in values)/total})
    (output/'aggregate_rates.json').write_text(json.dumps(aggregates,indent=2)+'\n')
    evidence={'version':protocol['version'],'source':protocol['source'],
        'scope':protocol['source']['kind'],'bases':len(groups),'variants':len(index)-len(groups),
        'detector_unchanged':True,'detector_sha256':preparation['detector_sha256'],
        'protocol_sha256':hashlib.sha256((output/'protocol.json').read_bytes()).hexdigest(),
        'input_sha256':preparation['input_sha256'],
        'raw_sha256':hashlib.sha256((output/'native_trials.jsonl.gz').read_bytes()).hexdigest(),
        'native_qiskit_max_tvd_error':parity,'raw_verified':dict(counters),'cost':summary['cost'],'conditions':rows,
        'unimplemented':protocol['deferred']}
    (output/'evaluation.json').write_text(json.dumps(evidence,indent=2)+'\n')
    (output/'preparation_manifest.json').write_text(json.dumps(preparation,indent=2)+'\n')
    # Small, fully inspectable circuit definitions for every challenge.
    (output/'circuits.jsonl').write_bytes((prepared/'input.jsonl').read_bytes())
    report=['# Web-derived attack mechanisms: fixed-detector transfer test','',
        'The Wildisley infected rows did not fit RASQP references or thresholds. The detector is a calibrated distribution comparison, not a classifier trained on those infected labels. Earlier probe selection did use toy X/Z/CX mutations, so related-family familiarity remains real. The new CLI regression replaces infected candidates with clean circuits and verifies identical calibration data and thresholds, while evaluation outcomes change.','',
        f"This challenge uses {len(groups)} fresh four-qubit workloads and {len(index)-len(groups)} locally implemented variants. **These are literature-derived mechanism tests, not a newly downloaded independent dataset or a reproduction of full article classification/hardware attacks.** The same three probes, alpha, reference size, calibration size and evaluation budget are retained; detector file hashes match the Wildisley run.",'',
        '| Noise | Workload | Variant | Joint detection | Fixed detection | Ideally invisible bases |',
        '|---|---|---|---:|---:|---:|']
    for row in rows:
        report.append(f"| {row['noise']:g} | {row['workload']} | {row['variant']} | {row['methods']['portfolio_joint']['macro_detection']:.4f} | {row['methods']['fixed_0Z']['macro_detection']:.4f} | {row['ideal_invisible']}/{row['bases']} |")
    report.extend(['','Per-variant >=90% detection and every-context clean confidence requirements are evaluated separately in evaluation.json. Passing a mean never substitutes for a failed condition. Clean calibration is conditional on trusted raw contexts; semantic-equivalence cases are reported separately. The field named fold is an execution block, not a newly trained cross-validation fold.','',
        'Sources and implementation boundaries:',
        '- [SQUASH](https://arxiv.org/html/2506.24081v1): untargeted SWAP insertion only. The full HQNN classification experiment is not reproduced. A scalar global phase cannot change fidelity; the targeted exp(i delta) expression is not implemented as a physical attack.',
        '- [QTrojan](https://arxiv.org/html/2302.08090v1): active RX/RY encoding sandwich, at circuit level. The pinned author notebook was inspected as JSON and never executed. No pulse configuration, trained QNN or classification attack-success rate is claimed.',
        '- [Upadhyay and Ghosh](https://arxiv.org/html/2209.11872v1): multiplicative variational-parameter tampering. No full iterative optimizer, cloud or approximation-ratio experiment is reproduced.',
        '- [Conditional Trojans](https://arxiv.org/html/2502.08880v1): mixed multi-control diagnostic variants extend the activation idea; these exact fixtures are our constructions, not author dataset files.','',
        'Known SWAP symmetry and prior rotation/CX studies make these diagnostic transfer tests, not proof that every attack concept was previously unknown. The initial-swap and mixed-trigger cases deliberately retain physically distinct computations invisible to the fixed input preparations. They are dormant on those inputs, including the ordinary zero-state workload execution, so their low alarm rates do not establish a bypass that changes the current workload output. They diagnose limits of general circuit coverage; an activating input is separately demonstrated by the physical regression test. Any noisy detection of such cases can arise from additional gate noise rather than logical activation.','',
        f"Independent raw verification: {counters['decisions_checked']:,} decisions; native versus Qiskit ideal probe-TVD error at most {parity:.3g}. Cost: {summary['cost']['shots']:,} simulated shots, zero QPU shots.",'',
        '[CutBackdoor](https://arxiv.org/abs/2607.18126) requires a cut-execution and reconstruction pipeline absent here. [QAD](https://netslab.ucd.ie/quantum-attack-dataset-qad/) is a separately identified public dataset, not imported or scored in this suite. Neither is counted as successfully tested protection.'])
    (output/'REPORT.txt').write_text('\n'.join(report)+'\n')
    print(json.dumps({'detector_unchanged':True,'bases':len(groups),'variants':len(index)-len(groups),
                      'verified_decisions':counters['decisions_checked'],'max_parity_error':parity},indent=2))


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--prepared',type=Path,default=ROOT/'results/runs/web-variants-input')
    parser.add_argument('--output',type=Path,default=ROOT/'results/web-variants-v1')
    args=parser.parse_args()
    analyze(args.prepared,args.output)
