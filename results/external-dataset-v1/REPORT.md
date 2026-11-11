# Wildisley Filho dataset: RASQP external evaluation

Pinned source: `48ec00d02776e602e3336c8db66ea9fda8c59576`. [Article](https://sol.sbc.org.br/index.php/sbseg_estendido/article/view/44500); [dataset repository](https://github.com/apenas-will/IC-QT-2026).

Inventoried 5,885 rows from 535 bases. Evaluated 3,256 rows from 296 complete base groups; exclusions are in preparation_manifest.json. Native-v1 is unchanged.

This is an exploratory transfer study with trusted golden references, not a reproduction of the authors' textual classifiers or original measurement/noise configuration. Classifier values below reuse archived out-of-fold predictions. Behavioral repeats are conditional on one calibration per base/noise context.

| Noise | Method | Macro infected detection | Macro clean alarms | Equivalent raw-context alarms |
|---|---|---:|---:|---:|
| 0 | fixed_0Z | 0.4727 | 0.0033 | 0.0032 |
| 0 | portfolio_joint | 0.9420 | 0.0030 | 0.0030 |
| 0 | portfolio_naive_or | 0.9478 | 0.0074 | 0.0071 |
| 0 | portfolio_bonferroni_or | 0.9433 | 0.0016 | 0.0013 |
| 0.005 | fixed_0Z | 0.4706 | 0.0102 | 0.0121 |
| 0.005 | portfolio_joint | 0.8069 | 0.0099 | 0.0131 |
| 0.005 | portfolio_naive_or | 0.8263 | 0.0275 | 0.0367 |
| 0.005 | portfolio_bonferroni_or | 0.7969 | 0.0060 | 0.0081 |

The equivalent control deliberately reuses the original raw calibration after adding an X pair; its alarms measure context mismatch, not malicious intent.

Noise 0: joint-minus-fixed gain 0.4694; paired base-cluster bootstrap 95% interval [0.4440452122043919, 0.49418918918918925]; fixed exploratory conjunction passes: **False**. 10/296 base contexts fail to establish the clean Wilson upper-bound requirement; this does not prove their true FPR exceeds 10%. 146/2960 infected variants are ideally invisible to all three probes. SDK inspection outcomes: {'inconclusive': 2960} (inconclusive withholds results and is not proof of attack).
Noise 0.005: joint-minus-fixed gain 0.3363; paired base-cluster bootstrap 95% interval [0.3111836201435811, 0.36238980679898647]; fixed exploratory conjunction passes: **False**. 43/296 base contexts fail to establish the clean Wilson upper-bound requirement; this does not prove their true FPR exceeds 10%. 146/2960 infected variants are ideally invisible to all three probes. SDK inspection outcomes: {'inconclusive': 2960} (inconclusive withholds results and is not proof of attack).

All clean and equivalent candidates pass structural inspection. Two normalization-budget errors per noise condition are included among inconclusive infected outcomes. Withholding every visible infected candidate classically means behavioral sensitivity does not establish additional withholding coverage in that information setting.

| Noise | Inserted gate | Variants | Fixed detection | Joint detection |
|---|---|---:|---:|---:|
| 0 | CH | 180 | 0.2064 | 0.9942 |
| 0 | CT | 201 | 0.1193 | 0.8976 |
| 0 | CX | 179 | 0.2421 | 0.8442 |
| 0 | CY | 214 | 0.2264 | 0.9816 |
| 0 | CZ | 211 | 0.1903 | 0.9670 |
| 0 | H | 394 | 0.9975 | 1.0000 |
| 0 | T | 405 | 0.1365 | 0.9118 |
| 0 | X | 369 | 0.8616 | 0.9169 |
| 0 | Y | 429 | 0.9280 | 0.9499 |
| 0 | Z | 378 | 0.1101 | 0.9385 |
| 0.005 | CH | 180 | 0.2951 | 0.8819 |
| 0.005 | CT | 201 | 0.1599 | 0.4976 |
| 0.005 | CX | 179 | 0.3292 | 0.7962 |
| 0.005 | CY | 214 | 0.3284 | 0.8920 |
| 0.005 | CZ | 211 | 0.1730 | 0.8509 |
| 0.005 | H | 394 | 0.9617 | 0.9407 |
| 0.005 | T | 405 | 0.1044 | 0.5546 |
| 0.005 | X | 369 | 0.8303 | 0.8602 |
| 0.005 | Y | 429 | 0.8777 | 0.8903 |
| 0.005 | Z | 378 | 0.1006 | 0.8522 |

| Archived text baseline | Full-source mean fold balanced accuracy | Same eligible-cohort mean fold balanced accuracy |
|---|---:|---:|
| BoW + LogisticRegression | 0.5585 | 0.5823 |
| BoW + MultinomialNB | 0.5058 | 0.5090 |
| BoW + ComplementNB | 0.5058 | 0.5090 |
| BoW + LinearSVC | 0.7329 | 0.8029 |
| TF-IDF + LogisticRegression | 0.6078 | 0.6602 |
| TF-IDF + MultinomialNB | 0.6050 | 0.6570 |
| TF-IDF + ComplementNB | 0.6050 | 0.6570 |
| TF-IDF + LinearSVC | 0.6618 | 0.7327 |

These are different information settings: RASQP receives an independently approved golden circuit and new behavioral samples; the text classifier predicts from a single QASM representation. The metrics do not establish a quantum advantage or direct superiority.

Executor cost: 1,177,559,040 simulated shots, 3,030,448 circuit instances, 32,560 local calls; zero QPU shots. Shared portfolio ablations reuse observations. Independent analysis verified 1,818,624 decisions from raw counts.

Source family labels and dataset labels are retained verbatim; source_dataset='other' is not relabeled as VeriQBench. Related variants share author base folds, but base separation does not imply complete separation of higher-level algorithm families.
