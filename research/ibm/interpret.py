"""Labeled post hoc diagnostics; never change recorded thresholds or pass criteria."""
import argparse
import json
from pathlib import Path
import sys

import numpy as np
from qiskit.quantum_info import Operator

ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT))
from research.ibm.common import unitary_source, variants, write_json, sha


def interpret(output):
    summary=json.loads((output/'summary.json').read_text())
    contexts={c['id']:c for c in summary['contexts']}
    diagnostics={}
    for relative in sorted({c['workload'] for c in contexts.values()}):
        source,provenance=unitary_source(relative)
        candidates=variants(source)
        diagnostics[relative]={'source':provenance,
            'explicit_rotations_scaled':sum(item.operation.name in ['rx','ry','rz'] for item in source.data),
            'operator_changed':{case:not Operator(source).equiv(Operator(c)) for case,c in candidates.items()}}
    conditional=[]
    for backend in sorted({c['backend_snapshot'] for c in contexts.values()}):
        for case in ['x0','z0','cx01','rz0_0.12','swap_end','parameter_scale_1.1']:
            rows=[r for r in summary['outcomes'] if r['case']==case and contexts[r['context']]['backend_snapshot']==backend]
            changed=[r for r in rows if diagnostics[contexts[r['context']]['workload']]['operator_changed'][case]]
            visible=[r for r in rows if max(contexts[r['context']]['posthoc_ideal_probe_tvd'][case])>=1e-9]
            conditional.append({'backend':backend,'case':case,'all_context_acquisitions':len(rows),
                'operator_changed_context_acquisitions':len(changed),
                'ideally_visible_context_acquisitions':len(visible),
                'all_macro_alarm_rate':float(np.mean([r['rate'] for r in rows])),
                'operator_changed_macro_alarm_rate':float(np.mean([r['rate'] for r in changed])) if changed else None,
                'ideally_visible_macro_alarm_rate':float(np.mean([r['rate'] for r in visible])) if visible else None})
    assignments=[v for c in contexts.values() for v in c['readout_mean_assignment_error']]
    value={'status':'Post hoc explanatory stratification; original criteria and every failed context remain unchanged.',
           'summary_sha256':sha(output/'summary.json'),'source_diagnostics':diagnostics,
           'selected_qubits_assignment_error_range':[min(assignments),max(assignments)],
           'conditional_rates':conditional,
           'limits':'Quantum noise can expose extra-gate noise even when a logical mutation preserves the ideal measured distributions. Alarm rates are not harmful-application-impact rates. Parameter scaling changes only explicit RX/RY/RZ source instructions: QFT and quantum-walk sources lack those instructions, so those parameter cases are unchanged controls. These facts do not rescue any prespecified failed criterion.'}
    write_json(output/'interpretation.json',value)
    path=output/'REPORT.md'
    # Older run directories may still contain the original plain-text filename.
    original=path if path.exists() else output/'REPORT.txt'
    text=original.read_text().split('\n## Post hoc interpretation\n')[0]
    text+='\n## Post hoc interpretation\n\n'
    text+=f"The topology-selected physical qubits have mean assignment errors from {min(assignments):.2%} to {max(assignments):.2%}. These include poor qubits; the study does not estimate the average quality of a modern processor or select layouts to obtain favorable outcomes.\n\n"
    text+='Some strong mutations are intrinsically invisible to the three fixed preparations/bases. The post hoc ideal distributions show this before considering noise. Noise from added physical gates can itself raise alarms even when the logical ideal distributions agree. Consequently, these aggregate results must not be interpreted as evidence that noise alone caused all misses, or that an alarm establishes harmful application impact.\n\n'
    text+='The parameter-scale diagnostic changes explicit RX/RY/RZ instructions only. The QFT and quantum-walk source programs have no such instructions and their parameter cases are unchanged controls. All original rows remain, while interpretation.json separately identifies operator-changing and ideally visible cases. Conditional descriptive rates do not replace the original criteria.\n'
    path.write_text(text)
    print(json.dumps({'readout_range':value['selected_qubits_assignment_error_range'],
                      'posthoc_strong':[r for r in conditional if r['case'] in ['x0','z0','cx01']]},indent=2))


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--output',type=Path,default=ROOT/'results/ibm-calibration-v1')
    args=parser.parse_args()
    interpret(args.output)
