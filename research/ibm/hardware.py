"""Bounded IBM Open Plan pilot. Preparation never submits a quantum job."""
import argparse
import datetime
import getpass
import gzip
import json
import os
from pathlib import Path
import random
import sys

import numpy as np
from qiskit import ClassicalRegister, QuantumCircuit

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT))
from research.ibm.common import *

PROTOCOL = ROOT/'docs/IBM_HARDWARE_PILOT_V1.json'
CREDENTIALS = ROOT/'.ibm/accounts.json'
STAGES = ['reference','calibration','evaluation']


class PilotError(ValueError):
    """An intentionally public local error message, never an SDK exception."""



def configure():
    from qiskit_ibm_runtime import QiskitRuntimeService
    token=getpass.getpass('IBM Quantum API key (hidden; stored locally, never in results): ').strip()
    if not token: raise PilotError('API key is empty')
    CREDENTIALS.parent.mkdir(mode=0o700,parents=True,exist_ok=True)
    QiskitRuntimeService.save_account(token=token,channel='ibm_quantum_platform',
        name='rasqp-open',filename=str(CREDENTIALS),plans_preference=['open'],overwrite=False)
    os.chmod(CREDENTIALS,0o600)
    print('Saved the Open Plan account locally. No quantum job submitted.')


def service():
    from qiskit_ibm_runtime import QiskitRuntimeService
    token=os.environ.get('QISKIT_IBM_TOKEN')
    if token:
        credentials={'channel':'ibm_quantum_platform','token':token}
    elif CREDENTIALS.is_file():
        credentials={'filename':str(CREDENTIALS),'name':'rasqp-open'}
    else:
        raise PilotError('No IBM account configured. Run: .venv-ibm/bin/python research/ibm/hardware.py configure')
    discovery=QiskitRuntimeService(**credentials,plans_preference=['open'])
    instances=sorted([x for x in discovery.instances() if x['plan']=='open'],key=lambda x:x['crn'])
    if not instances:
        raise PilotError('Only a verified Open Plan instance is allowed by this pilot')
    result=QiskitRuntimeService(**credentials,instance=instances[0]['crn'],plans_preference=['open'])
    if result.active_instance()!=instances[0]['crn']: raise PilotError('active Open Plan instance mismatch')
    return result


def check():
    s=service()
    backends=s.backends(min_num_qubits=4,simulator=False,operational=True)
    print(json.dumps({'connected':True,'instance_plan':'open',
                      'backends':[{'name':b.name,'pending_jobs':b.status().pending_jobs} for b in backends]},indent=2))


def physical_isa(compact, backend, physical):
    c=QuantumCircuit(backend.num_qubits)
    c.add_register(ClassicalRegister(len(physical),'meas'))
    c.global_phase=compact.global_phase
    for item in compact.data:
        qubits=[physical[compact.find_bit(q).index] for q in item.qubits]
        clbits=[compact.find_bit(q).index for q in item.clbits]
        c.append(item.operation,qubits,clbits)
    return c


def prepare(output,backend,local=False):
    protocol=json.loads(PROTOCOL.read_text())
    output=fresh(output)
    (output/'protocol.json').write_bytes(PROTOCOL.read_bytes())
    write_json(output/'properties_before.json',backend.properties().to_dict())
    write_json(output/'configuration.json',backend.configuration().to_dict())
    circuits,index=[],[]
    workloads=[]
    for wi,relative in enumerate(protocol['workloads']):
        clean,provenance=unitary_source(relative)
        physical=connected_paths(backend,clean.num_qubits,count=1)[0]
        target=compact_target(backend,physical)
        workloads.append({'index':wi,'source':provenance,'physical_qubits':physical})
        for case,c in variants(clean,hardware=True).items():
            for pi,(state,basis) in enumerate(protocol['probes']):
                compact,measurement=compile_probe(c,state,basis,target,protocol)
                circuits.append(physical_isa(compact,backend,physical))
                index.append({'circuit_index':len(circuits)-1,'workload':wi,'case':case,'probe':pi,
                              'logical_to_physical_measurement':[physical[q] for q in measurement],
                              'duration_seconds':compact.estimate_duration(target,unit='s'),
                              'operations':dict(compact.count_ops())})
    qpy_write(output/'isa.qpy',circuits)
    rep_delay=getattr(backend,'default_rep_delay',None)
    if rep_delay is None: rep_delay=getattr(backend.configuration(),'default_rep_delay',None)
    rep_delay=float(rep_delay if rep_delay is not None else 250e-6)
    stages={}
    rng=random.Random(protocol['seed'])
    for stage in STAGES:
        selected=[dict(r) for r in index if stage=='evaluation' or r['case']=='clean']
        shots=protocol['reference_shots_per_probe'] if stage=='reference' else (
            protocol['calibration_repetitions' if stage=='calibration' else 'evaluation_repetitions']*protocol['shots_per_probe'])
        rng.shuffle(selected)
        for row in selected: row['shots']=shots
        # Reserve 2s per PUB, deliberately above IBM's 2s per sub-job approximation.
        estimate=sum(2+(rep_delay+r['duration_seconds'])*shots for r in selected)
        stages[stage]={'pubs':selected,'shots':shots*len(selected),
                       'conservative_estimate_seconds':estimate}
    total_shots=sum(x['shots'] for x in stages.values())
    total_estimate=sum(x['conservative_estimate_seconds'] for x in stages.values())
    if total_shots!=protocol['maximum_total_shots']: raise PilotError('pilot shot budget mismatch')
    within=(total_estimate<=protocol['reserved_maximum_qpu_seconds'] and
            all(x['conservative_estimate_seconds']<=protocol['stage_execution_caps_seconds'][stage] for stage,x in stages.items()))
    plan={'version':protocol['version'],'backend':backend.name,'local_mock':local,
          'workloads':workloads,'stages':stages,'rep_delay_seconds':rep_delay,
          'estimated_seconds':total_estimate,'total_shots':total_shots,'within_budget':within,
          'protocol_sha256':sha(output/'protocol.json'),'isa_sha256':sha(output/'isa.qpy'),
          'properties_sha256':sha(output/'properties_before.json'),
          'detector_sha256':detector_hashes()}
    write_json(output/'plan.json',plan)
    print(json.dumps({'backend':backend.name,'local_mock':local,'pubs':{k:len(v['pubs']) for k,v in stages.items()},
                      'shots':total_shots,'estimated_seconds':total_estimate,'within_budget':within,
                      'job_caps_seconds':protocol['stage_execution_caps_seconds'],'submitted_jobs':0},indent=2))
    return plan


def load_plan(output):
    output=Path(output)
    plan=json.loads((output/'plan.json').read_text())
    if sha(output/'protocol.json')!=plan['protocol_sha256'] or sha(output/'isa.qpy')!=plan['isa_sha256']:
        raise PilotError('prepared hardware plan changed')
    if sha(output/'properties_before.json')!=plan['properties_sha256']:
        raise PilotError('prepared calibration snapshot changed')
    if detector_hashes()!=plan['detector_sha256']: raise PilotError('detector changed after preparation')
    protocol=json.loads((output/'protocol.json').read_text())
    if sha(output/'protocol.json')!=sha(PROTOCOL): raise PilotError('hardware protocol differs from frozen pilot')
    if sum(protocol['stage_execution_caps_seconds'].values())>600: raise PilotError('user ten-minute allowance exceeded')
    # Recalculate limits from the stored PUB data rather than trusting display fields.
    if set(plan['stages'])!=set(STAGES): raise PilotError('invalid hardware stages')
    if sum(s['shots'] for s in plan['stages'].values())!=protocol['maximum_total_shots']:
        raise PilotError('stored shot budget mismatch')
    if not np.isfinite(plan['rep_delay_seconds']) or plan['rep_delay_seconds']<=0:
        raise PilotError('invalid repetition delay')
    for stage,s in plan['stages'].items():
        if any(not np.isfinite(p['duration_seconds']) or p['duration_seconds']<0 for p in s['pubs']):
            raise PilotError('invalid circuit duration')
        cases=protocol['cases'] if stage=='evaluation' else ['clean']
        expected={(wi,case,pi) for wi in range(len(plan['workloads'])) for case in cases for pi in range(3)}
        observed=[(p['workload'],p['case'],p['probe']) for p in s['pubs']]
        if len(observed)!=len(expected) or set(observed)!=expected: raise PilotError('hardware PUB coverage mismatch')
        expected_shots=protocol['reference_shots_per_probe'] if stage=='reference' else protocol[
            'calibration_repetitions' if stage=='calibration' else 'evaluation_repetitions']*protocol['shots_per_probe']
        if any(p['shots']!=expected_shots for p in s['pubs']): raise PilotError('per-PUB budget mismatch')
        if s['shots']!=sum(p['shots'] for p in s['pubs']): raise PilotError('PUB shot mismatch')
        estimate=sum(2+(plan['rep_delay_seconds']+p['duration_seconds'])*p['shots'] for p in s['pubs'])
        if abs(estimate-s['conservative_estimate_seconds'])>1e-9: raise PilotError('usage estimate changed')
        if estimate>protocol['stage_execution_caps_seconds'][stage]: raise PilotError('stage exceeds QPU budget')
    if sum(s['conservative_estimate_seconds'] for s in plan['stages'].values())>protocol['reserved_maximum_qpu_seconds']:
        raise PilotError('total plan exceeds QPU budget')
    return plan,protocol


def submit(output,stage):
    output=Path(output); plan,protocol=load_plan(output)
    if plan['local_mock']: raise PilotError('a mock backend plan cannot submit hardware jobs')
    for previous in STAGES[:STAGES.index(stage)]:
        if not (output/(previous+'-result.json.gz')).is_file():
            raise PilotError('retrieve the preceding stage before submitting another job')
    journal=output/(stage+'-submission.json')
    if journal.exists(): raise PilotError('submission already attempted; collect or reconcile its job ID, never retry automatically')
    if len(list(output.glob('*-submission.json')))>=protocol['max_jobs']: raise PilotError('job limit reached')
    s=service(); b=s.backend(plan['backend'])
    if not b.status().operational: raise PilotError('prepared backend is not operational')
    write_json(output/(stage+'-properties-before.json'),b.properties(refresh=True).to_dict())
    circuits=qpy_read(output/'isa.qpy')
    from qiskit_ibm_runtime import SamplerV2
    options={'max_execution_time':protocol['stage_execution_caps_seconds'][stage],
             'execution':{'rep_delay':plan['rep_delay_seconds']},
             'dynamical_decoupling':{'enable':False},
             'twirling':{'enable_gates':False,'enable_measure':False},
             'environment':{'job_tags':['rasqp',protocol['version'],stage]}}
    pubs=[(circuits[p['circuit_index']],None,p['shots']) for p in plan['stages'][stage]['pubs']]
    with journal.open('x') as handle:
        json.dump({'stage':stage,'state':'intent_recorded','plan_sha256':sha(output/'plan.json'),
                   'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()},handle)
        handle.flush(); os.fsync(handle.fileno())
    # A network ambiguity after this point remains an intent; no auto-resubmission.
    job=SamplerV2(mode=b,options=options).run(pubs)
    record=json.loads(journal.read_text()); record.update({'state':'submitted','job_id':job.job_id()})
    write_json(journal,record)
    print(json.dumps({'stage':stage,'job_id':job.job_id(),'status':'submitted','automatic_retries':False}))


def collect(output,stage):
    output=Path(output); plan,protocol=load_plan(output)
    journal=json.loads((output/(stage+'-submission.json')).read_text())
    if journal['plan_sha256']!=sha(output/'plan.json'): raise PilotError('submission plan mismatch')
    if 'job_id' not in journal: raise PilotError('ambiguous submission; reconcile the job in IBM dashboard before proceeding')
    destination=output/(stage+'-result.json.gz')
    if destination.exists(): raise PilotError('stage results already retained')
    s=service(); job=s.job(journal['job_id'])
    if job.backend().name!=plan['backend']: raise PilotError('returned backend differs from prepared backend')
    status=str(job.status())
    if status not in ['DONE','JobStatus.DONE']:
        print(json.dumps({'stage':stage,'job_id':job.job_id(),'status':status,'collected':False})); return
    result=job.result()
    if len(result)!=len(plan['stages'][stage]['pubs']): raise PilotError('result PUB count mismatch')
    records=[]
    for planned,observed in zip(plan['stages'][stage]['pubs'],result,strict=True):
        bits=observed.data.meas.get_bitstrings()
        width=len(plan['workloads'][planned['workload']]['physical_qubits'])
        if len(bits)!=planned['shots'] or any(len(v)!=width or set(v)-{'0','1'} for v in bits):
            raise PilotError('returned shot count or logical result width mismatch')
        records.append({'pub':planned,'bitstrings':bits,'metadata':observed.metadata})
    metrics=job.metrics()
    usage_status=metrics.get('usage',{}).get('status','unknown')
    value={'stage':stage,'job_id':job.job_id(),'backend':plan['backend'],'hardware':True,
           'records':records,'usage_seconds':job.usage() if usage_status not in ['pending','unknown'] else None,
           'usage_status':usage_status,'metrics':metrics,
           'result_metadata':result.metadata,'retrieved_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
    properties=job.properties()
    if properties is not None:
        path=output/(stage+'-job-properties.json')
        write_json(path,properties.to_dict())
        value['job_properties_sha256']=sha(path)
    else:
        value['job_properties_sha256']=None
    # Snapshot the then-current calibration separately; it need not match submission.
    write_json(output/(stage+'-properties-after.json'),s.backend(plan['backend']).properties().to_dict())
    with gzip.open(destination,'xt') as handle: json.dump(value,handle,default=default_json)
    print(json.dumps({'stage':stage,'job_id':job.job_id(),'collected':True,'shots':sum(len(x['bitstrings']) for x in records)}))


def analyze(output):
    output=Path(output); plan,protocol=load_plan(output)
    stages={}
    for stage in STAGES:
        with gzip.open(output/(stage+'-result.json.gz'),'rt') as handle: stages[stage]=json.load(handle)
        if stages[stage].get('hardware') is not True: raise PilotError('mock counts cannot be labeled hardware')
    def counts(bits,width): return np.bincount([int(x,2) for x in bits],minlength=1<<width).tolist()
    def batch(stage,wi,case):
        pubs=sorted([r for r in stages[stage]['records'] if r['pub']['workload']==wi and r['pub']['case']==case],key=lambda r:r['pub']['probe'])
        if [x['pub']['probe'] for x in pubs]!=[0,1,2]: raise PilotError('missing or duplicate probe result')
        width=len(plan['workloads'][wi]['physical_qubits']); size=protocol['shots_per_probe']
        if stage=='reference': return [counts(p['bitstrings'],width) for p in pubs]
        n=protocol['calibration_repetitions' if stage=='calibration' else 'evaluation_repetitions']
        return [[counts(p['bitstrings'][i*size:(i+1)*size],width) for p in pubs] for i in range(n)]
    outcomes=[]
    for wi in range(len(plan['workloads'])):
        references=[(np.array(c)/sum(c)).tolist() for c in batch('reference',wi,'clean')]
        clean=batch('calibration',wi,'clean')
        observed={case:batch('evaluation',wi,case) for case in protocol['cases']}
        evaluated=rust_evaluate(references,clean,[b for values in observed.values() for b in values],protocol['alpha'])
        size=protocol['evaluation_repetitions']
        for index,case in enumerate(observed):
            outcomes.append({'workload':wi,'case':case,**rate(evaluated['rejected'][index*size:(index+1)*size]),
                             'threshold':evaluated['verifier']['threshold']})
    write_json(output/'hardware_evaluation.json',{'hardware':True,'backend':plan['backend'],'outcomes':outcomes,
        'job_ids':{stage:value['job_id'] for stage,value in stages.items()},
        'recorded_shots':sum(len(r['bitstrings']) for s in stages.values() for r in s['records']),
        'scope':'Budget-constrained component pilot. Ordered shot partitions from sequential probe PUBs assume stationarity; not independent physical recalibrations or proof of workload authenticity.'})
    print(json.dumps(outcomes,indent=2))


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('command',choices=['configure','check','prepare','submit','collect','analyze'])
    parser.add_argument('--output',type=Path,default=ROOT/'results/runs/ibm-hardware-pilot')
    parser.add_argument('--backend')
    parser.add_argument('--local-backend',choices=['FakeTorino','FakeSherbrooke'])
    parser.add_argument('--stage',choices=STAGES)
    args=parser.parse_args()
    try:
        if args.command=='configure': configure()
        elif args.command=='check': check()
        elif args.command=='prepare':
            if args.local_backend:
                from qiskit_ibm_runtime import fake_provider
                backend=getattr(fake_provider,args.local_backend)()
            else:
                s=service(); backend=s.backend(args.backend) if args.backend else s.least_busy(operational=True,simulator=False,min_num_qubits=4)
            prepare(args.output,backend,local=bool(args.local_backend))
        elif args.command in ['submit','collect']:
            if args.stage is None: raise PilotError('--stage is required')
            globals()[args.command](args.output,args.stage)
        else: analyze(args.output)
    except PilotError as error:
        print(str(error),file=sys.stderr); raise SystemExit(2)
    except Exception as error:
        # SDK/network exception text may contain account details; never echo it.
        print(f'IBM operation stopped ({type(error).__name__}). No automatic retry. Check local configuration or IBM job status.',file=sys.stderr)
        raise SystemExit(2)
