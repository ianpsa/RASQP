"""Reconstruct all decisions and distinguish false alarms from withholding."""
import argparse
import collections
import json
from pathlib import Path
import shutil
import sys

import numpy as np

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from research.false_positives.prepare import detector_hashes, sha
from research.external_dataset.analyze import audit_raw


def category(group, row):
    role = row['gate'] if row['case'] != 'equivalent_x_pair' else 'equivalent_x_pair'
    return f"{group['family']}:{group['approved']}:{role}"


def describe(rows, method):
    values = [r['methods'][method] for r in rows]
    rejected, n = sum(v['rejected'] for v in values), sum(v['n'] for v in values)
    return {'contexts': len(rows), 'alarms': rejected, 'decisions': n,
            'macro_alarm_rate': float(np.mean([v['rate'] for v in values])),
            'pooled_descriptive_alarm_fraction': rejected/n,
            'worst_context_rate': max(v['rate'] for v in values),
            'worst_context_wilson95_upper': max(v['wilson95'][1] for v in values),
            'contexts_exceeding_clean_upper_bound_0_10': sum(v['wilson95'][1] > .10 for v in values),
            'contexts_below_detection_lower_bound_0_90': sum(v['wilson95'][0] < .90 for v in values)}


def analyze(prepared, output):
    protocol = json.loads((output/'protocol.json').read_text())
    preparation = json.loads((prepared/'preparation.json').read_text())
    assert sha(prepared/'input.jsonl') == preparation['input_sha256']
    assert sha(output/'protocol.json') == preparation['protocol_sha256']
    assert detector_hashes() == preparation['detector_sha256']
    for relative, expected in preparation['source_files_sha256'].items():
        assert sha(prepared/relative) == expected
    groups = [json.loads(line) for line in (prepared/'input.jsonl').open()]
    by_group = {g['base_circuit_id']: g for g in groups}
    samples = {(g['base_circuit_id'], s['sample_id']): s for g in groups for s in g['samples']}
    summary = json.loads((output/'native_summary.json').read_text())
    counters, outcomes = audit_raw(output/'native_trials.jsonl.gz', protocol)
    assert outcomes == summary['outcomes']
    assert all(counters[k] == summary['cost'][k] for k in ['shots', 'circuits', 'executor_calls'])
    max_error, comparisons = 0.0, 0
    per_category = collections.defaultdict(list)
    for row in outcomes:
        gid = row['context']['base_circuit_id']
        group = by_group[gid]
        if row['case'] == 'equivalent_x_pair':
            expected = [0.0, 0.0, 0.0]
        else:
            expected = samples[gid, row['sample_id']]['ideal_probe_tvd_qiskit']
        error = max(abs(a-b) for a, b in zip(expected, row['ideal_probe_tvd']))
        max_error = max(max_error, error)
        comparisons += 3
        assert error < 1e-9, (gid, row['sample_id'], error)
        per_category[row['context']['noise'], category(group, row)].append(row)
    rows = []
    for (noise, name), selected in sorted(per_category.items()):
        rows.append({'noise': noise, 'category': name,
                     'methods': {m: describe(selected, m) for m in protocol['methods']},
                     'structural_inspection': dict(collections.Counter(r['inspection']['decision'] for r in selected)),
                     'ideal_probe_tvd_min': min(max(r['ideal_probe_tvd']) for r in selected),
                     'ideal_probe_tvd_max': max(max(r['ideal_probe_tvd']) for r in selected)})
    # Keep every small author-artifact outcome visible, with explicit features.
    qdoor = []
    for row in outcomes:
        g = by_group[row['context']['base_circuit_id']]
        if g['family'] != 'QDoor': continue
        qdoor.append({**row, 'approved': g['approved'], 'features': g['features']})
    matched = []
    for noise in protocol['noise']:
        selected = [r for r in outcomes if r['context']['noise'] == noise and r['gate'] == 'approved'
                    and r['case'] == 'clean' and r['mode'] != 'QDoor']
        matched.append({'noise': noise, 'methods': {m: describe(selected, m) for m in protocol['methods']}})
    evidence = {'version': protocol['version'], 'preparation': preparation,
                'protocol_sha256': sha(output/'protocol.json'), 'raw_sha256': sha(output/'native_trials.jsonl.gz'),
                'source_code_sha256': {p: sha(ROOT/p) for p in ['src/external_main.rs',
                    'research/false_positives/prepare.py', 'research/false_positives/analyze.py']},
                'cost': summary['cost'], 'raw_verified': dict(counters),
                'ideal_probe_tvd_comparisons': comparisons, 'maximum_ideal_tvd_parity_error': max_error,
                'matched_benchmark_contexts': matched, 'categories': rows, 'qdoor_contexts': qdoor,
                'inference_boundary': 'Conditional simulation alarm probabilities; one reference/calibration per context. Per-context Wilson intervals, not a pooled or simultaneous population guarantee. Structural inspection is a separate SDK component, not a full guarded-runtime execution.'}
    (output/'evaluation.json').write_text(json.dumps(evidence, indent=2)+'\n')
    for origin, target in [('input.jsonl', 'circuits.jsonl'), ('preparation.json', 'preparation_manifest.json'),
                           ('inventory.json', 'inventory.json')]:
        shutil.copyfile(prepared/origin, output/target)
    shutil.copytree(prepared/'sources', output/'sources', dirs_exist_ok=True)
    report = ['# External clean controls and QDoor artifacts', '',
        f"Evaluated {preparation['eligible_benchmark_circuits']} author-provided clean benchmark circuits, their supported exact compiler rewrites, and six QDoor approval/feature contexts. The three probes, alpha=0.01, 511 calibration batches, 8192 reference shots per probe and 768 evaluation shots remain frozen. This study uses 256 evaluation repetitions for more precise per-context intervals. No infected artifact enters calibration.", '',
        '**These are published research artifacts, not circuits recovered from a real intrusion.** QASMBench and VeriQBench are clean unitary-block benchmarks. QDoor supplies trained numeric parameter artifacts and source topology; this replay does not reproduce MNIST/PCA classification, approximate synthesis training, or the article\'s attack-success rate.', '',
        '## Clean-control and compilation results', '',
        'An alarm on a known benign rewrite is a false alarm if treated as an attack, even when noisy gate behavior really changed. An inconclusive structural inspection is an availability/approval outcome, not an attack diagnosis. Reapproving an independently validated rewrite is a new calibration context, not automatic recalibration.', '',
        '| Noise | Source : approved circuit : evaluated case | Joint alarms | Worst context | Structural accept / inconclusive / reject |',
        '|---|---|---:|---:|---|']
    for row in rows:
        rate = row['methods']['portfolio_joint']
        structural = row['structural_inspection']
        report.append(f"| {row['noise']:g} | {row['category']} | {rate['alarms']}/{rate['decisions']} ({rate['macro_alarm_rate']:.2%}) | {rate['worst_context_rate']:.2%} | {structural.get('accept',0)} / {structural.get('inconclusive',0)} / {structural.get('reject',0)} |")
    report.extend(['', '## QDoor author-artifact cases', '',
        'Compiled initialization is an author-intended benign approximation, not an exactly equivalent rewrite. Its operator overlap distance from the original is about 0.0201; the trained backdoor distance is about 0.0341. The supplied QASM matches initialization parameters within float32 rounding (maximum difference about 2.37e-7). Those distances describe artifacts, not maliciousness or a security bound.', '',
        '| Noise | Approved | Feature vector | Evaluated artifact | Joint alarms | Wilson95 | Inspector |',
        '|---|---|---|---|---:|---|---|'])
    for row in qdoor:
        if row['case'] == 'equivalent_x_pair': continue
        rate = row['methods']['portfolio_joint']
        report.append(f"| {row['context']['noise']:g} | {row['approved']} | {row['features']} | {row['gate']} | {rate['rejected']}/{rate['n']} | {rate['wilson95'][0]:.3f}–{rate['wilson95'][1]:.3f} | {row['inspection']['decision']} |")
    report.extend(['', '## Scope and reproducibility', '',
        f"Scanned {sum(preparation['inventory_counts'].values())} source QASM files. Exclusions: {preparation['exclusions']}. The inventory also retains optional compiler rewrites that failed the strict operator check; an excluded rewrite never excludes its eligible original. Compiler output wire permutations are explicitly restored before comparing operators and submitting native gates.", '',
        f"{preparation['byte_identical_wildisley_eligible']} eligible source files are byte-identical to previously inspected Wildisley artifacts. Repositories contain related programs and pretranspiled counterparts; treat counts as a descriptive benchmark, not independent new programs or an unseen-population guarantee. All per-context results are retained; macro averages do not rescue a failing context.", '',
        f"Independently reconstructed {counters['decisions_checked']:,} method decisions and all thresholds from raw samples. Compared {comparisons:,} ideal probe distances with Qiskit; maximum absolute difference {max_error:.3g}. Cost: {summary['cost']['shots']:,} simulated shots, zero QPU shots. These are SDK component tests; no full-runtime release rate or physical device security guarantee is inferred.", '',
        'Sources: [QASMBench](https://github.com/pnnl/QASMBench), [VeriQBench](https://github.com/Veri-Q/Benchmark), [QDoor author toy implementation](https://github.com/chuchengc/QDoor), [QDoor article](https://arxiv.org/abs/2307.09529). Pinned commits, archive hashes and source file hashes are in preparation_manifest.json. See [reproduction commands](https://github.com/ianpsa/RASQP#false-positive-controls).'])
    (output/'REPORT.md').write_text('\n'.join(report)+'\n')
    print(json.dumps({'matched_clean': matched, 'ideal_tvd_parity_error': max_error,
                     'verified': dict(counters), 'cost': summary['cost']}, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--prepared', type=Path, default=ROOT/'results/runs/false-positives-input-v1')
    parser.add_argument('--output', type=Path, default=ROOT/'results/false-positives-v1')
    args = parser.parse_args()
    analyze(args.prepared, args.output)
