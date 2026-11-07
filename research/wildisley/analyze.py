"""Independently check raw native decisions and compare matching dataset cohorts."""
import argparse
import collections
import gzip
import hashlib
import json
import math
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parents[2]


def tvd(counts, reference):
    total = sum(counts)
    assert total > 0 and len(counts) == len(reference)
    return sum(abs(c / total - p) for c, p in zip(counts, reference)) / 2


def quantile(scores, alpha):
    rank = math.ceil((len(scores) + 1) * (1 - alpha))
    return sorted(scores)[rank - 1] if rank <= len(scores) else 1.0


def wilson(k, n):
    z = 1.959963984540054
    den = 1 + z*z/n
    center = (k/n + z*z/(2*n))/den
    radius = z * math.sqrt((k/n)*(1-k/n)/n + z*z/(4*n*n))/den
    return [center-radius, center+radius]


def audit_raw(path, protocol):
    references, calibration, observations = {}, {}, {}
    thresholds = None
    counters = collections.Counter()
    checked = []
    context = None
    with gzip.open(path, "rt") as handle:
        for line in handle:
            row = json.loads(line)
            if row["kind"] == "samples":
                current = row["context"]
                if current != context:
                    references, calibration, observations = {}, {}, {}
                    thresholds = None
                    context = current
                width = context["width"]
                counts = row["counts"]
                assert all(len(c) == 1 << width and sum(c) == row["shots"] for c in counts)
                counters["shots"] += row["shots"] * len(counts)
                counters["circuits"] += len(counts)
                counters["executor_calls"] += 1
                stage, probe = row["stage"], row["probe"]
                if stage == "reference":
                    references[probe] = [c / sum(counts[0]) for c in counts[0]]
                elif stage in {"portfolio_calibration", "fixed_calibration"}:
                    calibration[stage, probe] = counts
                else:
                    observations[stage, probe] = counts
            elif row["kind"] == "calibration":
                assert row["context"] == context
                assert all(max(abs(a-b) for a,b in zip(row["references"][i], references[i])) < 1e-12 for i in range(3))
                alpha = protocol["alpha"]
                scores = [[tvd(c, references[i]) for c in calibration["portfolio_calibration", i]] for i in range(3)]
                joint = quantile([max(v) for v in zip(*scores)], alpha)
                fixed = quantile([tvd(c, references[0]) for c in calibration["fixed_calibration", 0]], alpha)
                thresholds = {"joint": joint, "fixed": fixed,
                              "individual": [quantile(s, alpha) for s in scores],
                              "bonferroni": [quantile(s, alpha/3) for s in scores]}
                assert abs(joint-row["joint_threshold"]) < 1e-12 and abs(fixed-row["fixed_threshold"]) < 1e-12
                for key, source in [("individual", "individual_thresholds"), ("bonferroni", "bonferroni_thresholds")]:
                    assert max(abs(a-b) for a,b in zip(thresholds[key], row[source])) < 1e-12
            else:
                assert row["kind"] == "decision" and thresholds is not None
                outcome = row["outcome"]
                case = outcome["case"]
                scores = [[tvd(c, references[i]) for c in observations[case, i]] for i in range(3)]
                flags = {
                    "fixed_0Z": [tvd(c, references[0]) > thresholds["fixed"]+1e-12 for c in observations[case+"_fixed", 0]],
                    "portfolio_joint": [max(s) > thresholds["joint"]+1e-12 for s in zip(*scores)],
                    "portfolio_naive_or": [any(v > t+1e-12 for v,t in zip(s, thresholds["individual"])) for s in zip(*scores)],
                    "portfolio_bonferroni_or": [any(v > t+1e-12 for v,t in zip(s, thresholds["bonferroni"])) for s in zip(*scores)],
                }
                assert flags == row["flags"]
                for name, decisions in flags.items():
                    record = outcome["methods"][name]
                    n, k = len(decisions), sum(decisions)
                    assert record["n"] == n and record["rejected"] == k
                    assert abs(record["rate"] - k/n) < 1e-12
                    assert max(abs(a-b) for a,b in zip(record["wilson95"], wilson(k,n))) < 1e-12
                    counters["decisions_checked"] += n
                checked.append(outcome)
                observations = {}
    return counters, checked


def classification(rows):
    counts = collections.Counter((r["label"], r["prediction"]) for r in rows)
    tp, tn, fp, fn = counts[1,1], counts[0,0], counts[0,1], counts[1,0]
    tpr, tnr = tp/(tp+fn), tn/(tn+fp)
    return {"n":len(rows), "tp":tp,"tn":tn,"fp":fp,"fn":fn,
            "accuracy":(tp+tn)/len(rows), "balanced_accuracy":(tpr+tnr)/2,
            "recall":tpr,"false_positive_rate":1-tnr,
            "precision":tp/(tp+fp) if tp+fp else 0,
            "f1":2*tp/(2*tp+fp+fn) if 2*tp+fp+fn else 0}


def main(prepared, output):
    prep = json.loads((prepared/"preparation.json").read_text())
    protocol = json.loads((output/"protocol.json").read_text())
    summary = json.loads((output/"native_summary.json").read_text())
    counters, outcomes = audit_raw(output/"native_trials.jsonl.gz", protocol)
    assert outcomes == summary["outcomes"]
    for name in ("shots", "circuits", "executor_calls"):
        assert counters[name] == summary["cost"][name]
    inventory = [json.loads(line) for line in (prepared/"inventory.jsonl").open()]
    index = {r["sample_id"]:r for r in inventory}
    baselines = {}
    for name, predictions in prep["predictions"].items():
        baselines[name] = {}
        for cohort in ("full", "eligible"):
            values = [r for r in predictions.values() if cohort == "full" or r["eligible"]]
            folds = [classification([r for r in values if r["fold"] == fold]) for fold in range(1,6)]
            baselines[name][cohort] = {"pooled":classification(values), "folds":folds,
                "mean_fold_balanced_accuracy":float(np.mean([r["balanced_accuracy"] for r in folds])),
                "sd_fold_balanced_accuracy":float(np.std([r["balanced_accuracy"] for r in folds], ddof=1))}
    evaluations = []
    rng = np.random.default_rng(protocol["seed"]+900_000)
    for noise in protocol["noise"]:
        selected = [r for r in outcomes if r["context"]["noise"] == noise]
        grouped = collections.defaultdict(list)
        for row in selected:
            grouped[row["context"]["base_circuit_id"]].append(row)
        methods = {}
        for method in protocol["methods"]:
            per_base = []
            for base, rows in grouped.items():
                infected = [r for r in rows if r["case"] == "infected"]
                clean = next(r for r in rows if r["case"] == "clean")
                equivalent = next(r for r in rows if r["case"] == "equivalent_x_pair")
                per_base.append({"base":base,"fold":rows[0]["context"]["fold"],
                    "tpr":float(np.mean([r["methods"][method]["rate"] for r in infected])),
                    "fpr":clean["methods"][method]["rate"],
                    "equivalent_alarm_rate":equivalent["methods"][method]["rate"],
                    "clean_upper":clean["methods"][method]["wilson95"][1]})
            tpr, fpr = (float(np.mean([r[key] for r in per_base])) for key in ("tpr","fpr"))
            methods[method] = {"macro_tpr":tpr,"macro_clean_fpr":fpr,"macro_balanced_accuracy":(tpr+1-fpr)/2,
                "macro_equivalent_alarm_rate":float(np.mean([r["equivalent_alarm_rate"] for r in per_base])),
                "contexts_failing_clean_upper_0.10":sum(r["clean_upper"] > .10 for r in per_base),
                "per_base":per_base,
                "folds":[{"fold":fold,"macro_tpr":float(np.mean([r["tpr"] for r in per_base if r["fold"]==fold])),
                          "macro_clean_fpr":float(np.mean([r["fpr"] for r in per_base if r["fold"]==fold]))} for fold in range(1,6)]}
        joint = methods["portfolio_joint"]["per_base"]
        fixed = methods["fixed_0Z"]["per_base"]
        differences = np.array([a["tpr"]-b["tpr"] for a,b in zip(joint,fixed)])
        bootstrap = np.mean(rng.choice(differences, size=(2000,len(differences)), replace=True),axis=1)
        gain = float(np.mean(differences))
        per_gate, per_mode = {}, {}
        for attribute, target in [("gate",per_gate),("mode",per_mode)]:
            for value in sorted({r[attribute] for r in selected if r["case"]=="infected"}):
                members = [r for r in selected if r["case"]=="infected" and r[attribute]==value]
                target[value] = {"variants":len(members), "rates":{m:float(np.mean([r["methods"][m]["rate"] for r in members])) for m in protocol["methods"]}}
        infected = [r for r in selected if r["case"]=="infected"]
        invisible = [r for r in infected if max(r["ideal_probe_tvd"]) < 1e-10]
        inspections = collections.Counter(r["inspection"]["decision"] for r in infected)
        inspection_by_case = {case:dict(collections.Counter(r["inspection"]["decision"] for r in selected if r["case"]==case)) for case in ("clean","infected","equivalent_x_pair")}
        evaluations.append({"noise":noise,"methods":methods,"gain_joint_minus_fixed":gain,
            "base_cluster_bootstrap95_gain":np.quantile(bootstrap,[.025,.975]).tolist(),
            "exploratory_criterion_pass":gain>=.20 and methods["portfolio_joint"]["contexts_failing_clean_upper_0.10"]==0,
            "per_gate":per_gate,"per_mode":per_mode,"infected_variants":len(infected),
            "ideal_invisible_to_all_three":len(invisible),"classical_inspection":dict(inspections),
            "classical_inspection_by_case":inspection_by_case,
            "classical_analysis_errors":sum(r["inspection"].get("status")=="analysis_error_withheld" for r in selected),
            "raw_changed_infected":sum(r["raw_changed"] for r in infected)})
    provenance = {k:v for k,v in prep.items() if k != "predictions"}
    (output/"preparation_manifest.json").write_text(json.dumps(provenance,indent=2)+"\n")
    with gzip.open(output/"inventory.jsonl.gz","wt") as h:
        for row in inventory:
            h.write(json.dumps(row,separators=(",",":"))+"\n")
    result = {"source":prep["source"],"coverage":{k:prep[k] for k in ("source_rows","source_bases","eligible_bases","eligible_rows","conversion_state_max_error")},
        "raw_verified":dict(counters),"cost":summary["cost"],"baseline_status":"Authors' archived GroupKFold predictions; no classifier retraining",
        "text_baselines":baselines,"behavioral_evaluation":evaluations,
        "protocol_sha256":hashlib.sha256((output/"protocol.json").read_bytes()).hexdigest(),
        "raw_sha256":hashlib.sha256((output/"native_trials.jsonl.gz").read_bytes()).hexdigest()}
    (output/"evaluation.json").write_text(json.dumps(result,indent=2)+"\n")
    text = ["# Wildisley Filho dataset: RASQP external evaluation", "",
        f"Pinned source: `{prep['source']['commit']}`. [Article]({prep['source']['article']}); [dataset repository]({prep['source']['repository']}).", "",
        f"Inventoried {prep['source_rows']:,} rows from {prep['source_bases']} bases. Evaluated {prep['eligible_rows']:,} rows from {prep['eligible_bases']} complete base groups; exclusions are in preparation_manifest.json. Native-v1 is unchanged.","",
        "This is an exploratory transfer study with trusted golden references, not a reproduction of the authors' textual classifiers or original measurement/noise configuration. Classifier values below reuse archived out-of-fold predictions. Behavioral repeats are conditional on one calibration per base/noise context.","",
        "| Noise | Method | Macro infected detection | Macro clean alarms | Equivalent raw-context alarms |", "|---|---|---:|---:|---:|"]
    for evaluation in evaluations:
        for name, values in evaluation["methods"].items():
            text.append(f"| {evaluation['noise']:g} | {name} | {values['macro_tpr']:.4f} | {values['macro_clean_fpr']:.4f} | {values['macro_equivalent_alarm_rate']:.4f} |")
    text.extend(["", "The equivalent control deliberately reuses the original raw calibration after adding an X pair; its alarms measure context mismatch, not malicious intent.", ""])
    for evaluation in evaluations:
        failures=evaluation['methods']['portfolio_joint']['contexts_failing_clean_upper_0.10']
        text.append(f"Noise {evaluation['noise']:g}: joint-minus-fixed gain {evaluation['gain_joint_minus_fixed']:.4f}; paired base-cluster bootstrap 95% interval {evaluation['base_cluster_bootstrap95_gain']}; fixed exploratory conjunction passes: **{evaluation['exploratory_criterion_pass']}**. {failures}/{prep['eligible_bases']} base contexts fail to establish the clean Wilson upper-bound requirement; this does not prove their true FPR exceeds 10%. {evaluation['ideal_invisible_to_all_three']}/{evaluation['infected_variants']} infected variants are ideally invisible to all three probes. SDK inspection outcomes: {evaluation['classical_inspection']} (inconclusive withholds results and is not proof of attack).")
    text.extend(["", "All clean and equivalent candidates pass structural inspection. Two normalization-budget errors per noise condition are included among inconclusive infected outcomes. Withholding every visible infected candidate classically means behavioral sensitivity does not establish additional withholding coverage in that information setting.", "",
                 "| Noise | Inserted gate | Variants | Fixed detection | Joint detection |", "|---|---|---:|---:|---:|"])
    for evaluation in evaluations:
        for gate,values in evaluation['per_gate'].items():
            text.append(f"| {evaluation['noise']:g} | {gate} | {values['variants']} | {values['rates']['fixed_0Z']:.4f} | {values['rates']['portfolio_joint']:.4f} |")
    text.extend(["", "| Archived text baseline | Full-source mean fold balanced accuracy | Same eligible-cohort mean fold balanced accuracy |", "|---|---:|---:|"])
    for name, values in baselines.items():
        text.append(f"| {name} | {values['full']['mean_fold_balanced_accuracy']:.4f} | {values['eligible']['mean_fold_balanced_accuracy']:.4f} |")
    text.extend(["", "These are different information settings: RASQP receives an independently approved golden circuit and new behavioral samples; the text classifier predicts from a single QASM representation. The metrics do not establish a quantum advantage or direct superiority.", "",
        f"Executor cost: {summary['cost']['shots']:,} simulated shots, {summary['cost']['circuits']:,} circuit instances, {summary['cost']['executor_calls']:,} local calls; zero QPU shots. Shared portfolio ablations reuse observations. Independent analysis verified {counters['decisions_checked']:,} decisions from raw counts.", "",
        "Source family labels and dataset labels are retained verbatim; source_dataset='other' is not relabeled as VeriQBench. Related variants share author base folds, but base separation does not imply complete separation of higher-level algorithm families."])
    (output/"REPORT.txt").write_text("\n".join(text)+"\n")
    print("\n".join(text[:14]))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--prepared",type=Path,default=ROOT/"results/runs/wildisley-input")
    parser.add_argument("--output",type=Path,default=ROOT/"results/wildisley-v1")
    args = parser.parse_args()
    main(args.prepared,args.output)
