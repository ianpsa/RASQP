# RASQP native Rust research run

Protocol SHA256: `d771d0df8a6d08ffa3929a966c9164514edac09425d64eed98beb4a1c2bd9792`. All quantum results are local simulation.

Native RNG changes sampled counts and generated benchmarks; this is not an exact replay of Python random streams. No scientific novelty or quantum advantage is established.

## batch

```json
{
  "configurations": 72,
  "decisions": 24192,
  "groups": {
    "adaptive_distribution/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 0.0,
      "rejected": 0,
      "wilson95": [
        0.0,
        0.008813890333169308
      ]
    },
    "adaptive_distribution/noise=0.005/shots=256": {
      "n": 432,
      "rate": 0.0,
      "rejected": 0,
      "wilson95": [
        0.0,
        0.008813890333169308
      ]
    },
    "adaptive_distribution/noise=0/shots=1024": {
      "n": 432,
      "rate": 0.0,
      "rejected": 0,
      "wilson95": [
        0.0,
        0.008813890333169308
      ]
    },
    "adaptive_distribution/noise=0/shots=256": {
      "n": 432,
      "rate": 0.0,
      "rejected": 0,
      "wilson95": [
        0.0,
        0.008813890333169308
      ]
    },
    "append_x/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "append_x/noise=0.005/shots=256": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "append_x/noise=0/shots=1024": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "append_x/noise=0/shots=256": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "append_z/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 0.08796296296296297,
      "rejected": 38,
      "wilson95": [
        0.0647564405253622,
        0.11843278391586065
      ]
    },
    "append_z/noise=0.005/shots=256": {
      "n": 432,
      "rate": 0.06481481481481481,
      "rejected": 28,
      "wilson95": [
        0.04522067356505566,
        0.09208030505825834
      ]
    },
    "append_z/noise=0/shots=1024": {
      "n": 432,
      "rate": 0.03009259259259259,
      "rejected": 13,
      "wilson95": [
        0.017669129148781142,
        0.05079948074766963
      ]
    },
    "append_z/noise=0/shots=256": {
      "n": 432,
      "rate": 0.032407407407407406,
      "rejected": 14,
      "wilson95": [
        0.01940085021099727,
        0.05365658426724439
      ]
    },
    "biased_counts/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 0.8425925925925926,
      "rejected": 364,
      "wilson95": [
        0.8052494574651758,
        0.8738965806398746
      ]
    },
    "biased_counts/noise=0.005/shots=256": {
      "n": 432,
      "rate": 0.7685185185185185,
      "rejected": 332,
      "wilson95": [
        0.7264835389098634,
        0.805820112577879
      ]
    },
    "biased_counts/noise=0/shots=1024": {
      "n": 432,
      "rate": 0.8194444444444444,
      "rejected": 354,
      "wilson95": [
        0.7804075125042365,
        0.8528502797829052
      ]
    },
    "biased_counts/noise=0/shots=256": {
      "n": 432,
      "rate": 0.7476851851851852,
      "rejected": 323,
      "wilson95": [
        0.7046667840233413,
        0.7863374462282832
      ]
    },
    "clean/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 0.04398148148148148,
      "rejected": 19,
      "wilson95": [
        0.028334732491662844,
        0.06766682489553325
      ]
    },
    "clean/noise=0.005/shots=256": {
      "n": 432,
      "rate": 0.034722222222222224,
      "rejected": 15,
      "wilson95": [
        0.02115312712491233,
        0.056493131935120226
      ]
    },
    "clean/noise=0/shots=1024": {
      "n": 432,
      "rate": 0.02546296296296296,
      "rejected": 11,
      "wilson95": [
        0.01427646546230223,
        0.04501449527056678
      ]
    },
    "clean/noise=0/shots=256": {
      "n": 432,
      "rate": 0.023148148148148147,
      "rejected": 10,
      "wilson95": [
        0.012621269412727147,
        0.04208086673835098
      ]
    },
    "drop_gate/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 0.7222222222222222,
      "rejected": 312,
      "wilson95": [
        0.6781677960595972,
        0.7623593637923275
      ]
    },
    "drop_gate/noise=0.005/shots=256": {
      "n": 432,
      "rate": 0.6550925925925926,
      "rejected": 283,
      "wilson95": [
        0.6090788497464877,
        0.6983723972335013
      ]
    },
    "drop_gate/noise=0/shots=1024": {
      "n": 432,
      "rate": 0.7662037037037037,
      "rejected": 331,
      "wilson95": [
        0.7240530605939565,
        0.803661766311995
      ]
    },
    "drop_gate/noise=0/shots=256": {
      "n": 432,
      "rate": 0.6597222222222222,
      "rejected": 285,
      "wilson95": [
        0.6138104977341333,
        0.7028183984094375
      ]
    },
    "equivalent_rewrite/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 0.14351851851851852,
      "rejected": 62,
      "wilson95": [
        0.11359566340301239,
        0.17972535100119166
      ]
    },
    "equivalent_rewrite/noise=0.005/shots=256": {
      "n": 432,
      "rate": 0.07407407407407407,
      "rejected": 32,
      "wilson95": [
        0.052956222047894755,
        0.10270005490258279
      ]
    },
    "equivalent_rewrite/noise=0/shots=1024": {
      "n": 432,
      "rate": 0.034722222222222224,
      "rejected": 15,
      "wilson95": [
        0.02115312712491233,
        0.056493131935120226
      ]
    },
    "equivalent_rewrite/noise=0/shots=256": {
      "n": 432,
      "rate": 0.037037037037037035,
      "rejected": 16,
      "wilson95": [
        0.022924122201269433,
        0.05931096144055399
      ]
    },
    "forged_uniform/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "forged_uniform/noise=0.005/shots=256": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "forged_uniform/noise=0/shots=1024": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "forged_uniform/noise=0/shots=256": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "forged_zero/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 0.8333333333333334,
      "rejected": 360,
      "wilson95": [
        0.7952843593030465,
        0.8655063804748406
      ]
    },
    "forged_zero/noise=0.005/shots=256": {
      "n": 432,
      "rate": 0.8333333333333334,
      "rejected": 360,
      "wilson95": [
        0.7952843593030465,
        0.8655063804748406
      ]
    },
    "forged_zero/noise=0/shots=1024": {
      "n": 432,
      "rate": 0.8333333333333334,
      "rejected": 360,
      "wilson95": [
        0.7952843593030465,
        0.8655063804748406
      ]
    },
    "forged_zero/noise=0/shots=256": {
      "n": 432,
      "rate": 0.8333333333333334,
      "rejected": 360,
      "wilson95": [
        0.7952843593030465,
        0.8655063804748406
      ]
    },
    "network_tamper/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "network_tamper/noise=0.005/shots=256": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "network_tamper/noise=0/shots=1024": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "network_tamper/noise=0/shots=256": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "parameter_shift/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 0.25462962962962965,
      "rejected": 110,
      "wilson95": [
        0.2158350611743695,
        0.29774953315579683
      ]
    },
    "parameter_shift/noise=0.005/shots=256": {
      "n": 432,
      "rate": 0.2523148148148148,
      "rejected": 109,
      "wilson95": [
        0.21366255377171675,
        0.29533321597665874
      ]
    },
    "parameter_shift/noise=0/shots=1024": {
      "n": 432,
      "rate": 0.2638888888888889,
      "rejected": 114,
      "wilson95": [
        0.22453997409878773,
        0.30739991855854215
      ]
    },
    "parameter_shift/noise=0/shots=256": {
      "n": 432,
      "rate": 0.24768518518518517,
      "rejected": 107,
      "wilson95": [
        0.20932209726315054,
        0.29049602332164315
      ]
    },
    "passive_hook/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 0.04398148148148148,
      "rejected": 19,
      "wilson95": [
        0.028334732491662844,
        0.06766682489553325
      ]
    },
    "passive_hook/noise=0.005/shots=256": {
      "n": 432,
      "rate": 0.034722222222222224,
      "rejected": 15,
      "wilson95": [
        0.02115312712491233,
        0.056493131935120226
      ]
    },
    "passive_hook/noise=0/shots=1024": {
      "n": 432,
      "rate": 0.02546296296296296,
      "rejected": 11,
      "wilson95": [
        0.01427646546230223,
        0.04501449527056678
      ]
    },
    "passive_hook/noise=0/shots=256": {
      "n": 432,
      "rate": 0.023148148148148147,
      "rejected": 10,
      "wilson95": [
        0.012621269412727147,
        0.04208086673835098
      ]
    },
    "replay/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "replay/noise=0.005/shots=256": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "replay/noise=0/shots=1024": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "replay/noise=0/shots=256": {
      "n": 432,
      "rate": 1.0,
      "rejected": 432,
      "wilson95": [
        0.9911861096668306,
        1.0
      ]
    },
    "small_rotation/noise=0.005/shots=1024": {
      "n": 432,
      "rate": 0.06944444444444445,
      "rejected": 30,
      "wilson95": [
        0.04907381175608644,
        0.09740481603080933
      ]
    },
    "small_rotation/noise=0.005/shots=256": {
      "n": 432,
      "rate": 0.04398148148148148,
      "rejected": 19,
      "wilson95": [
        0.028334732491662844,
        0.06766682489553325
      ]
    },
    "small_rotation/noise=0/shots=1024": {
      "n": 432,
      "rate": 0.12731481481481483,
      "rejected": 55,
      "wilson95": [
        0.09913434073121853,
        0.16206490160044934
      ]
    },
    "small_rotation/noise=0/shots=256": {
      "n": 432,
      "rate": 0.05787037037037037,
      "rejected": 25,
      "wilson95": [
        0.039502229478095445,
        0.08403227539984592
      ]
    }
  },
  "note": "Native RNG differs from archived Python run; conservative native optimizer is not Qiskit transpiler."
}
```

## context

```json
{
  "H44_pass": true,
  "comparisons": [
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "x",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "x",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "x",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "z",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "z",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "z",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cx",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cx",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cx",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "xz",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "xz",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "xz",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cxz",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cxz",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cxz",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "weak_rotation",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "weak_rotation",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "weak_rotation",
        "noise": 0.0,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "x",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "x",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "x",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "z",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "z",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "z",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cx",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cx",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cx",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "xz",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "xz",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "xz",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cxz",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cxz",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cxz",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "weak_rotation",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "weak_rotation",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "weak_rotation",
        "noise": 0.0,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "x",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "x",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "x",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "z",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "z",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "z",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cx",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cx",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cx",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "xz",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "xz",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "xz",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cxz",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cxz",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cxz",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "weak_rotation",
        "noise": 0.005,
        "padded": false
      },
      "delta": 0.00390625,
      "gains": 1,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 0.5
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "weak_rotation",
        "noise": 0.005,
        "padded": false
      },
      "delta": -0.06640625,
      "gains": 0,
      "losses": 17,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "weak_rotation",
        "noise": 0.005,
        "padded": false
      },
      "delta": -0.078125,
      "gains": 0,
      "losses": 20,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "x",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "x",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "x",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "z",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "z",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "z",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cx",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cx",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cx",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "xz",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "xz",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "xz",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cxz",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cxz",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cxz",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "weak_rotation",
        "noise": 0.005,
        "padded": true
      },
      "delta": 0.0,
      "gains": 2,
      "losses": 2,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 0.6875
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "weak_rotation",
        "noise": 0.005,
        "padded": true
      },
      "delta": -0.0390625,
      "gains": 0,
      "losses": 10,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "weak_rotation",
        "noise": 0.005,
        "padded": true
      },
      "delta": -0.05859375,
      "gains": 0,
      "losses": 15,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "x",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "x",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "x",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "z",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "z",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "z",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cx",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cx",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cx",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "xz",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "xz",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "xz",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cxz",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cxz",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cxz",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "weak_rotation",
        "noise": 0.01,
        "padded": false
      },
      "delta": 0.03125,
      "gains": 10,
      "losses": 2,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 0.019287109375000007
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "weak_rotation",
        "noise": 0.01,
        "padded": false
      },
      "delta": -0.03515625,
      "gains": 3,
      "losses": 12,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 0.9963073730468768
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "weak_rotation",
        "noise": 0.01,
        "padded": false
      },
      "delta": -0.07421875,
      "gains": 0,
      "losses": 19,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "x",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "x",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "x",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "z",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "z",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "z",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cx",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cx",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cx",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "xz",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "xz",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "xz",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "cxz",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "cxz",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "cxz",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "ideal_0.01",
        "case": "weak_rotation",
        "noise": 0.01,
        "padded": true
      },
      "delta": 0.00390625,
      "gains": 1,
      "losses": 0,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 0.5
    },
    {
      "condition": {
        "baseline": "ideal_0.05",
        "case": "weak_rotation",
        "noise": 0.01,
        "padded": true
      },
      "delta": -0.0859375,
      "gains": 1,
      "losses": 23,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 0.9999999403953562
    },
    {
      "condition": {
        "baseline": "trusted_noisy_0.05",
        "case": "weak_rotation",
        "noise": 0.01,
        "padded": true
      },
      "delta": -0.0703125,
      "gains": 0,
      "losses": 18,
      "n": 256,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    }
  ],
  "conditions": [
    {
      "H44_pass": true,
      "methods": {
        "ideal_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.57421875,
              "rejected": 147,
              "wilson95": [
                0.5129905510088384,
                0.6332524703029463
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 2.220446049250313e-16
        },
        "ideal_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.57421875,
              "rejected": 147,
              "wilson95": [
                0.5129905510088384,
                0.6332524703029463
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 2.220446049250313e-16
        },
        "trusted_noisy_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.57421875,
              "rejected": 147,
              "wilson95": [
                0.5129905510088384,
                0.6332524703029463
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.0
        },
        "trusted_noisy_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.57421875,
              "rejected": 147,
              "wilson95": [
                0.5129905510088384,
                0.6332524703029463
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.0
        }
      },
      "noise": 0.0,
      "padded": false
    },
    {
      "H44_pass": true,
      "methods": {
        "ideal_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.55859375,
              "rejected": 143,
              "wilson95": [
                0.49734573849859953,
                0.6181092783264936
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 2.220446049250313e-16
        },
        "ideal_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.55859375,
              "rejected": 143,
              "wilson95": [
                0.49734573849859953,
                0.6181092783264936
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 2.220446049250313e-16
        },
        "trusted_noisy_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.55859375,
              "rejected": 143,
              "wilson95": [
                0.49734573849859953,
                0.6181092783264936
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.0
        },
        "trusted_noisy_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.55859375,
              "rejected": 143,
              "wilson95": [
                0.49734573849859953,
                0.6181092783264936
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.0
        }
      },
      "noise": 0.0,
      "padded": true
    },
    {
      "H44_pass": true,
      "methods": {
        "ideal_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0078125,
              "rejected": 2,
              "wilson95": [
                0.002145090367681419,
                0.028032768301535768
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.00390625,
              "rejected": 1,
              "wilson95": [
                0.0006898821669617927,
                0.021790975380582514
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.07421875000000022
        },
        "ideal_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.03515625,
              "rejected": 9,
              "wilson95": [
                0.018604096092635542,
                0.0654527704282918
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.02734375,
              "rejected": 7,
              "wilson95": [
                0.013307156355121155,
                0.05535570792246043
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.0703125,
              "rejected": 18,
              "wilson95": [
                0.04493554746197091,
                0.10839432915401236
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.0625
        },
        "trusted_noisy_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0078125,
              "rejected": 2,
              "wilson95": [
                0.002145090367681419,
                0.028032768301535768
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.00390625,
              "rejected": 1,
              "wilson95": [
                0.0006898821669617927,
                0.021790975380582514
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.00390625,
              "rejected": 1,
              "wilson95": [
                0.0006898821669617927,
                0.021790975380582514
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              0.9801025390625,
              0.0045166015625,
              0.005615234375,
              0.0,
              0.0045166015625,
              0.0,
              0.0,
              0.0,
              0.0050048828125,
              0.0,
              0.0001220703125,
              0.0,
              0.0001220703125,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0001220703125,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.008056640625,
              0.0,
              0.0,
              0.0,
              0.0084228515625,
              0.0,
              0.008056640625,
              0.006591796875,
              0.96875
            ],
            [
              0.9593505859375,
              0.01025390625,
              0.010986328125,
              0.0,
              0.00927734375,
              0.0,
              0.0,
              0.0,
              0.0098876953125,
              0.000244140625,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.0369873046875
        },
        "trusted_noisy_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.05078125,
              "rejected": 13,
              "wilson95": [
                0.029913811386553534,
                0.08493105962106533
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.04296875,
              "rejected": 11,
              "wilson95": [
                0.024159813300607562,
                0.07529105546366555
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.08203125,
              "rejected": 21,
              "wilson95": [
                0.05427759190690546,
                0.12214328807409647
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              0.9801025390625,
              0.0045166015625,
              0.005615234375,
              0.0,
              0.0045166015625,
              0.0,
              0.0,
              0.0,
              0.0050048828125,
              0.0,
              0.0001220703125,
              0.0,
              0.0001220703125,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0001220703125,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.008056640625,
              0.0,
              0.0,
              0.0,
              0.0084228515625,
              0.0,
              0.008056640625,
              0.006591796875,
              0.96875
            ],
            [
              0.9593505859375,
              0.01025390625,
              0.010986328125,
              0.0,
              0.00927734375,
              0.0,
              0.0,
              0.0,
              0.0098876953125,
              0.000244140625,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.0289306640625
        }
      },
      "noise": 0.005,
      "padded": false
    },
    {
      "H44_pass": true,
      "methods": {
        "ideal_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.00390625,
              "rejected": 1,
              "wilson95": [
                0.0006898821669617927,
                0.021790975380582514
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.01171875,
              "rejected": 3,
              "wilson95": [
                0.003993287123007758,
                0.033881572667882304
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.08203125
        },
        "ideal_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.02734375,
              "rejected": 7,
              "wilson95": [
                0.013307156355121155,
                0.05535570792246043
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.01953125,
              "rejected": 5,
              "wilson95": [
                0.008370729784579391,
                0.04489813224965644
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.05078125,
              "rejected": 13,
              "wilson95": [
                0.029913811386553534,
                0.08493105962106533
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.07031250000000022
        },
        "trusted_noisy_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.00390625,
              "rejected": 1,
              "wilson95": [
                0.0006898821669617927,
                0.021790975380582514
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0078125,
              "rejected": 2,
              "wilson95": [
                0.002145090367681419,
                0.028032768301535768
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.01171875,
              "rejected": 3,
              "wilson95": [
                0.003993287123007758,
                0.033881572667882304
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              0.975830078125,
              0.0040283203125,
              0.004638671875,
              0.0,
              0.00634765625,
              0.0001220703125,
              0.0,
              0.0,
              0.009033203125,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0001220703125,
              0.0,
              0.0,
              0.0,
              0.0135498046875,
              0.0,
              0.0001220703125,
              0.0001220703125,
              0.0072021484375,
              0.0001220703125,
              0.007568359375,
              0.0068359375,
              0.96435546875
            ],
            [
              0.9564208984375,
              0.0096435546875,
              0.0106201171875,
              0.0,
              0.0093994140625,
              0.0,
              0.0,
              0.0,
              0.0135498046875,
              0.0001220703125,
              0.000244140625,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.04296875
        },
        "trusted_noisy_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.02734375,
              "rejected": 7,
              "wilson95": [
                0.013307156355121155,
                0.05535570792246043
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.02734375,
              "rejected": 7,
              "wilson95": [
                0.013307156355121155,
                0.05535570792246043
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.0703125,
              "rejected": 18,
              "wilson95": [
                0.04493554746197091,
                0.10839432915401236
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              0.975830078125,
              0.0040283203125,
              0.004638671875,
              0.0,
              0.00634765625,
              0.0001220703125,
              0.0,
              0.0,
              0.009033203125,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0001220703125,
              0.0,
              0.0,
              0.0,
              0.0135498046875,
              0.0,
              0.0001220703125,
              0.0001220703125,
              0.0072021484375,
              0.0001220703125,
              0.007568359375,
              0.0068359375,
              0.96435546875
            ],
            [
              0.9564208984375,
              0.0096435546875,
              0.0106201171875,
              0.0,
              0.0093994140625,
              0.0,
              0.0,
              0.0,
              0.0135498046875,
              0.0001220703125,
              0.000244140625,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.0318603515625
        }
      },
      "noise": 0.005,
      "padded": true
    },
    {
      "H44_pass": true,
      "methods": {
        "ideal_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.0234375,
              "rejected": 6,
              "wilson95": [
                0.010784867555857603,
                0.0501809956000511
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.11718750000000022
        },
        "ideal_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.00390625,
              "rejected": 1,
              "wilson95": [
                0.0006898821669617927,
                0.021790975380582514
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.01953125,
              "rejected": 5,
              "wilson95": [
                0.008370729784579391,
                0.04489813224965644
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.08984375,
              "rejected": 23,
              "wilson95": [
                0.06061336726621258,
                0.1312015149581351
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.10937500000000022
        },
        "trusted_noisy_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0078125,
              "rejected": 2,
              "wilson95": [
                0.002145090367681419,
                0.028032768301535768
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.01478385642587143
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.0546875,
              "rejected": 14,
              "wilson95": [
                0.03285229140400938,
                0.08968958072528235
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              0.9578857421875,
              0.0108642578125,
              0.0108642578125,
              0.000244140625,
              0.00830078125,
              0.0001220703125,
              0.0001220703125,
              0.0,
              0.0113525390625,
              0.0001220703125,
              0.0001220703125,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0001220703125,
              0.0,
              0.0003662109375,
              0.000244140625,
              0.0152587890625,
              0.0,
              0.0,
              0.000244140625,
              0.0135498046875,
              0.000244140625,
              0.015380859375,
              0.0123291015625,
              0.9422607421875
            ],
            [
              0.9246826171875,
              0.0172119140625,
              0.0198974609375,
              0.0003662109375,
              0.0184326171875,
              0.0001220703125,
              0.0003662109375,
              0.0,
              0.0181884765625,
              0.0003662109375,
              0.0,
              0.0,
              0.0003662109375,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.048583984375
        },
        "trusted_noisy_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.015625,
              "rejected": 4,
              "wilson95": [
                0.006092598203366294,
                0.039479262709196654
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.02734375,
              "rejected": 7,
              "wilson95": [
                0.013307156355121155,
                0.05535570792246043
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.12890625,
              "rejected": 33,
              "wilson95": [
                0.09328077992555084,
                0.17550411351552564
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              0.9578857421875,
              0.0108642578125,
              0.0108642578125,
              0.000244140625,
              0.00830078125,
              0.0001220703125,
              0.0001220703125,
              0.0,
              0.0113525390625,
              0.0001220703125,
              0.0001220703125,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0001220703125,
              0.0,
              0.0003662109375,
              0.000244140625,
              0.0152587890625,
              0.0,
              0.0,
              0.000244140625,
              0.0135498046875,
              0.000244140625,
              0.015380859375,
              0.0123291015625,
              0.9422607421875
            ],
            [
              0.9246826171875,
              0.0172119140625,
              0.0198974609375,
              0.0003662109375,
              0.0184326171875,
              0.0001220703125,
              0.0003662109375,
              0.0,
              0.0181884765625,
              0.0003662109375,
              0.0,
              0.0,
              0.0003662109375,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.040771484375
        }
      },
      "noise": 0.01,
      "padded": false
    },
    {
      "H44_pass": true,
      "methods": {
        "ideal_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0078125,
              "rejected": 2,
              "wilson95": [
                0.002145090367681419,
                0.028032768301535768
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.01953125,
              "rejected": 5,
              "wilson95": [
                0.008370729784579391,
                0.04489813224965644
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.015625,
              "rejected": 4,
              "wilson95": [
                0.006092598203366294,
                0.039479262709196654
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.12890625000000022
        },
        "ideal_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.03125,
              "rejected": 8,
              "wilson95": [
                0.015918258891511296,
                0.06044160650774316
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.05078125,
              "rejected": 13,
              "wilson95": [
                0.029913811386553534,
                0.08493105962106533
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.10546875,
              "rejected": 27,
              "wilson95": [
                0.07350191137657121,
                0.14910097533446798
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              1.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              1.0
            ],
            [
              1.0000000000000004,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.11328125000000022
        },
        "trusted_noisy_0.01": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.01171875,
              "rejected": 3,
              "wilson95": [
                0.003993287123007758,
                0.033881572667882304
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.01953125,
              "rejected": 5,
              "wilson95": [
                0.008370729784579391,
                0.04489813224965644
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.01953125,
              "rejected": 5,
              "wilson95": [
                0.008370729784579391,
                0.04489813224965644
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              0.947509765625,
              0.0113525390625,
              0.010009765625,
              0.0001220703125,
              0.0101318359375,
              0.0001220703125,
              0.0001220703125,
              0.0,
              0.020263671875,
              0.0,
              0.0001220703125,
              0.0,
              0.000244140625,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0001220703125,
              0.0,
              0.0003662109375,
              0.0,
              0.0006103515625,
              0.0006103515625,
              0.0230712890625,
              0.0,
              0.000244140625,
              0.0003662109375,
              0.0135498046875,
              0.0003662109375,
              0.016357421875,
              0.012939453125,
              0.931396484375
            ],
            [
              0.9141845703125,
              0.01904296875,
              0.0201416015625,
              0.0008544921875,
              0.0174560546875,
              0.0003662109375,
              0.0001220703125,
              0.0,
              0.0264892578125,
              0.0003662109375,
              0.00048828125,
              0.0,
              0.00048828125,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.05224609375
        },
        "trusted_noisy_0.05": {
          "cases": {
            "benign_noise_drift": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "clean": {
              "n": 256,
              "rate": 0.0390625,
              "rejected": 10,
              "wilson95": [
                0.0213540358427329,
                0.07039983179986732
              ]
            },
            "cx": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "cxz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 256,
              "rate": 0.05078125,
              "rejected": 13,
              "wilson95": [
                0.029913811386553534,
                0.08493105962106533
              ]
            },
            "weak_rotation": {
              "n": 256,
              "rate": 0.08984375,
              "rejected": 23,
              "wilson95": [
                0.06061336726621258,
                0.1312015149581351
              ]
            },
            "x": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "xz": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            },
            "z": {
              "n": 256,
              "rate": 1.0,
              "rejected": 256,
              "wilson95": [
                0.9852161435741286,
                0.9999999999999999
              ]
            }
          },
          "reference": [
            [
              0.947509765625,
              0.0113525390625,
              0.010009765625,
              0.0001220703125,
              0.0101318359375,
              0.0001220703125,
              0.0001220703125,
              0.0,
              0.020263671875,
              0.0,
              0.0001220703125,
              0.0,
              0.000244140625,
              0.0,
              0.0,
              0.0
            ],
            [
              0.0,
              0.0001220703125,
              0.0,
              0.0003662109375,
              0.0,
              0.0006103515625,
              0.0006103515625,
              0.0230712890625,
              0.0,
              0.000244140625,
              0.0003662109375,
              0.0135498046875,
              0.0003662109375,
              0.016357421875,
              0.012939453125,
              0.931396484375
            ],
            [
              0.9141845703125,
              0.01904296875,
              0.0201416015625,
              0.0008544921875,
              0.0174560546875,
              0.0003662109375,
              0.0001220703125,
              0.0,
              0.0264892578125,
              0.0003662109375,
              0.00048828125,
              0.0,
              0.00048828125,
              0.0,
              0.0,
              0.0
            ]
          ],
          "threshold": 0.04248046875
        }
      },
      "noise": 0.01,
      "padded": true
    }
  ],
  "warning": "Noisy-reference advantage must be judged against all three ablations; passing the candidate alone is insufficient."
}
```

## extensions

```json
{
  "H41_pass": false,
  "H42_pass": true,
  "comparisons": [
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "x",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "x",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "x",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "z",
        "noise": 0.0
      },
      "delta": 1.0,
      "gains": 128,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0579449157400619e-37,
      "p_one_sided": 2.938735877055727e-39
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "z",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "z",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "cx",
        "noise": 0.0
      },
      "delta": 1.0,
      "gains": 128,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0579449157400619e-37,
      "p_one_sided": 2.938735877055727e-39
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "cx",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "cx",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "xz",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "xz",
        "noise": 0.0
      },
      "delta": 1.0,
      "gains": 128,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0579449157400619e-37,
      "p_one_sided": 2.938735877055727e-39
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "xz",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "cxz",
        "noise": 0.0
      },
      "delta": 1.0,
      "gains": 128,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0579449157400619e-37,
      "p_one_sided": 2.938735877055727e-39
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "cxz",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "cxz",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "weak_rotation",
        "noise": 0.0
      },
      "delta": 0.6171875,
      "gains": 79,
      "losses": 0,
      "n": 128,
      "p_holm": 4.963083675318176e-23,
      "p_one_sided": 1.6543612251060587e-24
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "weak_rotation",
        "noise": 0.0
      },
      "delta": -0.34375,
      "gains": 3,
      "losses": 47,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 0.9999999999988621
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "weak_rotation",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "x",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "x",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "x",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "z",
        "noise": 0.005
      },
      "delta": 0.90625,
      "gains": 116,
      "losses": 0,
      "n": 128,
      "p_holm": 3.851859888774405e-34,
      "p_one_sided": 1.2037062152420016e-35
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "z",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "z",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "cx",
        "noise": 0.005
      },
      "delta": 0.515625,
      "gains": 66,
      "losses": 0,
      "n": 128,
      "p_holm": 3.930232875259973e-19,
      "p_one_sided": 1.3552527156068872e-20
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "cx",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "cx",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "xz",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "xz",
        "noise": 0.005
      },
      "delta": 0.8125,
      "gains": 104,
      "losses": 0,
      "n": 128,
      "p_holm": 1.5284180038657183e-30,
      "p_one_sided": 4.930380657631349e-32
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "xz",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "cxz",
        "noise": 0.005
      },
      "delta": 0.3515625,
      "gains": 45,
      "losses": 0,
      "n": 128,
      "p_holm": 7.958078640513132e-13,
      "p_one_sided": 2.8421709430404045e-14
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "cxz",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "cxz",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed_0Z",
        "case": "weak_rotation",
        "noise": 0.005
      },
      "delta": -0.0390625,
      "gains": 8,
      "losses": 13,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 0.9053764343261708
    },
    {
      "condition": {
        "baseline": "trained_single",
        "case": "weak_rotation",
        "noise": 0.005
      },
      "delta": -0.0546875,
      "gains": 9,
      "losses": 16,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 0.9461239278316474
    },
    {
      "condition": {
        "baseline": "portfolio_naive_or",
        "case": "weak_rotation",
        "noise": 0.005
      },
      "delta": -0.078125,
      "gains": 0,
      "losses": 10,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    }
  ],
  "conditions": [
    {
      "H41_pass": true,
      "H42_pass": true,
      "gains": [
        1.0,
        1.0
      ],
      "methods": {
        "fixed_0Z": {
          "cases": {
            "benign_noise_drift": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "cx": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "cxz": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "equivalent_x_pair": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "weak_rotation": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "x": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "xz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "z": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            }
          },
          "shots_per_decision": 768,
          "threshold": 0.0
        },
        "portfolio_joint": {
          "cases": {
            "benign_noise_drift": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "cx": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "cxz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "equivalent_x_pair": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "weak_rotation": {
              "n": 128,
              "rate": 0.6171875,
              "rejected": 79,
              "wilson95": [
                0.5307322844502991,
                0.6968137414230975
              ]
            },
            "x": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "xz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "z": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            }
          },
          "shots_per_decision": 768,
          "threshold": 2.220446049250313e-16
        },
        "portfolio_naive_or": {
          "cases": {
            "benign_noise_drift": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "cx": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "cxz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "equivalent_x_pair": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "weak_rotation": {
              "n": 128,
              "rate": 0.6171875,
              "rejected": 79,
              "wilson95": [
                0.5307322844502991,
                0.6968137414230975
              ]
            },
            "x": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "xz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "z": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            }
          },
          "shots_per_decision": 768
        },
        "trained_single": {
          "cases": {
            "benign_noise_drift": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "cx": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "cxz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "equivalent_x_pair": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "weak_rotation": {
              "n": 128,
              "rate": 0.9609375,
              "rejected": 123,
              "wilson95": [
                0.9118129792331631,
                0.9832013892021962
              ]
            },
            "x": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "xz": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "z": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            }
          },
          "shots_per_decision": 768,
          "threshold": 2.220446049250313e-16
        }
      },
      "noise": 0.0,
      "worst_strong_tpr": 1.0
    },
    {
      "H41_pass": false,
      "H42_pass": true,
      "gains": [
        0.90625,
        0.8125
      ],
      "methods": {
        "fixed_0Z": {
          "cases": {
            "benign_noise_drift": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0390625,
              "rejected": 5,
              "wilson95": [
                0.016798610797803704,
                0.08818702076683686
              ]
            },
            "cx": {
              "n": 128,
              "rate": 0.484375,
              "rejected": 62,
              "wilson95": [
                0.39952298025969557,
                0.5701375496238515
              ]
            },
            "cxz": {
              "n": 128,
              "rate": 0.6484375,
              "rejected": 83,
              "wilson95": [
                0.5624978389203745,
                0.7257271271859276
              ]
            },
            "equivalent_x_pair": {
              "n": 128,
              "rate": 0.265625,
              "rejected": 34,
              "wilson95": [
                0.19675501016357286,
                0.3481529380896342
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 128,
              "rate": 0.03125,
              "rejected": 4,
              "wilson95": [
                0.01221826524505338,
                0.07759763126136077
              ]
            },
            "weak_rotation": {
              "n": 128,
              "rate": 0.1171875,
              "rejected": 15,
              "wilson95": [
                0.07231682349355734,
                0.18436615865334754
              ]
            },
            "x": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "xz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "z": {
              "n": 128,
              "rate": 0.09375,
              "rejected": 12,
              "wilson95": [
                0.054443804399147026,
                0.15672997257307855
              ]
            }
          },
          "shots_per_decision": 768,
          "threshold": 0.02213541666666665
        },
        "portfolio_joint": {
          "cases": {
            "benign_noise_drift": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.03125,
              "rejected": 4,
              "wilson95": [
                0.01221826524505338,
                0.07759763126136077
              ]
            },
            "cx": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "cxz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "equivalent_x_pair": {
              "n": 128,
              "rate": 0.109375,
              "rejected": 14,
              "wilson95": [
                0.06627879570700111,
                0.17523445138167731
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 128,
              "rate": 0.09375,
              "rejected": 12,
              "wilson95": [
                0.054443804399147026,
                0.15672997257307855
              ]
            },
            "weak_rotation": {
              "n": 128,
              "rate": 0.078125,
              "rejected": 10,
              "wilson95": [
                0.04298737505098223,
                0.13784693180479052
              ]
            },
            "x": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "xz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "z": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            }
          },
          "shots_per_decision": 768,
          "threshold": 0.04687500000000022
        },
        "portfolio_naive_or": {
          "cases": {
            "benign_noise_drift": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0859375,
              "rejected": 11,
              "wilson95": [
                0.04866313137138619,
                0.14734091054261295
              ]
            },
            "cx": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "cxz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "equivalent_x_pair": {
              "n": 128,
              "rate": 0.265625,
              "rejected": 34,
              "wilson95": [
                0.19675501016357286,
                0.3481529380896342
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 128,
              "rate": 0.1328125,
              "rejected": 17,
              "wilson95": [
                0.08460228531840389,
                0.20242016694495388
              ]
            },
            "weak_rotation": {
              "n": 128,
              "rate": 0.15625,
              "rejected": 20,
              "wilson95": [
                0.10348354800906658,
                0.22904810942897047
              ]
            },
            "x": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "xz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "z": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            }
          },
          "shots_per_decision": 768
        },
        "trained_single": {
          "cases": {
            "benign_noise_drift": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.046875,
              "rejected": 6,
              "wilson95": [
                0.021657844171923776,
                0.09849752245094322
              ]
            },
            "cx": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "cxz": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "equivalent_x_pair": {
              "n": 128,
              "rate": 0.15625,
              "rejected": 20,
              "wilson95": [
                0.10348354800906658,
                0.22904810942897047
              ]
            },
            "public_probe_adaptive_fabrication": {
              "n": 128,
              "rate": 0.0703125,
              "rejected": 9,
              "wilson95": [
                0.03742976830393997,
                0.12823480349360633
              ]
            },
            "weak_rotation": {
              "n": 128,
              "rate": 0.1328125,
              "rejected": 17,
              "wilson95": [
                0.08460228531840389,
                0.20242016694495388
              ]
            },
            "x": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "xz": {
              "n": 128,
              "rate": 0.1875,
              "rejected": 24,
              "wilson95": [
                0.12936141696376852,
                0.2638491807071743
              ]
            },
            "z": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            }
          },
          "shots_per_decision": 768,
          "threshold": 0.05598958333333358
        }
      },
      "noise": 0.005,
      "worst_strong_tpr": 1.0
    }
  ],
  "single_selection": {
    "objective": 0.916666666666667,
    "probes": [
      {
        "basis": "YY",
        "input_state": "ii"
      }
    ],
    "training_tvd_matrix": [
      [
        1.0,
        0.0,
        0.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        1.0,
        0.0,
        1.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        0.0,
        1.0000000000000004,
        0.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        2.220446049250313e-16,
        0.0,
        1.1102230246251565e-16
      ],
      [
        0.0,
        1.1102230246251565e-16,
        1.6653345369377348e-16
      ],
      [
        1.0000000000000004,
        1.0000000000000004,
        0.7500000000000002
      ]
    ]
  },
  "training_selection": {
    "objective": 1.0,
    "probes": [
      {
        "basis": "ZZ",
        "input_state": "00"
      },
      {
        "basis": "ZZ",
        "input_state": "11"
      },
      {
        "basis": "XX",
        "input_state": "++"
      }
    ],
    "training_tvd_matrix": [
      [
        1.0,
        0.0,
        0.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        1.0,
        0.0,
        1.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        0.0,
        1.0000000000000004,
        0.0
      ],
      [
        0.0,
        0.0,
        0.0
      ],
      [
        2.220446049250313e-16,
        0.0,
        1.1102230246251565e-16
      ],
      [
        0.0,
        1.1102230246251565e-16,
        1.6653345369377348e-16
      ],
      [
        1.0000000000000004,
        1.0000000000000004,
        0.7500000000000002
      ]
    ]
  }
}
```

## hypotheses

```json
{
  "checks": {
    "conditions": [
      {
        "layers": 1,
        "noise": 0.0,
        "payload": "i",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "practical_pass": false
        },
        "resources": {
          "depth": 7,
          "gates": 8,
          "two_qubit": 2
        },
        "threshold": 0.0
      },
      {
        "layers": 1,
        "noise": 0.0,
        "payload": "x",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "practical_pass": false
        },
        "resources": {
          "depth": 8,
          "gates": 9,
          "two_qubit": 2
        },
        "threshold": 0.0
      },
      {
        "layers": 1,
        "noise": 0.0,
        "payload": "y",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 1.0,
            "rejected": 128,
            "wilson95": [
              0.9708630437264916,
              1.0
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "practical_pass": true
        },
        "resources": {
          "depth": 9,
          "gates": 10,
          "two_qubit": 2
        },
        "threshold": 0.0
      },
      {
        "layers": 1,
        "noise": 0.0,
        "payload": "z",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 1.0,
            "rejected": 128,
            "wilson95": [
              0.9708630437264916,
              1.0
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "practical_pass": true
        },
        "resources": {
          "depth": 8,
          "gates": 9,
          "two_qubit": 2
        },
        "threshold": 0.0
      },
      {
        "layers": 2,
        "noise": 0.0,
        "payload": "i",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "practical_pass": false
        },
        "resources": {
          "depth": 11,
          "gates": 14,
          "two_qubit": 4
        },
        "threshold": 0.0
      },
      {
        "layers": 2,
        "noise": 0.0,
        "payload": "x",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 1.0,
            "rejected": 128,
            "wilson95": [
              0.9708630437264916,
              1.0
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "practical_pass": true
        },
        "resources": {
          "depth": 12,
          "gates": 15,
          "two_qubit": 4
        },
        "threshold": 0.0
      },
      {
        "layers": 2,
        "noise": 0.0,
        "payload": "y",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 1.0,
            "rejected": 128,
            "wilson95": [
              0.9708630437264916,
              1.0
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "practical_pass": true
        },
        "resources": {
          "depth": 13,
          "gates": 16,
          "two_qubit": 4
        },
        "threshold": 0.0
      },
      {
        "layers": 2,
        "noise": 0.0,
        "payload": "z",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 1.0,
            "rejected": 128,
            "wilson95": [
              0.9708630437264916,
              1.0
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.0,
            "rejected": 0,
            "wilson95": [
              0.0,
              0.029136956273508416
            ]
          },
          "practical_pass": true
        },
        "resources": {
          "depth": 12,
          "gates": 15,
          "two_qubit": 4
        },
        "threshold": 0.0
      },
      {
        "layers": 1,
        "noise": 0.005,
        "payload": "i",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 0.0703125,
            "rejected": 9,
            "wilson95": [
              0.03742976830393997,
              0.12823480349360633
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.015625,
            "rejected": 2,
            "wilson95": [
              0.004295449710144082,
              0.055180976679817195
            ]
          },
          "practical_pass": false
        },
        "resources": {
          "depth": 7,
          "gates": 8,
          "two_qubit": 2
        },
        "threshold": 0.0322265625
      },
      {
        "layers": 1,
        "noise": 0.005,
        "payload": "x",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 0.1015625,
            "rejected": 13,
            "wilson95": [
              0.060318648726840854,
              0.16602486330361116
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.015625,
            "rejected": 2,
            "wilson95": [
              0.004295449710144082,
              0.055180976679817195
            ]
          },
          "practical_pass": false
        },
        "resources": {
          "depth": 8,
          "gates": 9,
          "two_qubit": 2
        },
        "threshold": 0.0322265625
      },
      {
        "layers": 1,
        "noise": 0.005,
        "payload": "y",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 1.0,
            "rejected": 128,
            "wilson95": [
              0.9708630437264916,
              1.0
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.015625,
            "rejected": 2,
            "wilson95": [
              0.004295449710144082,
              0.055180976679817195
            ]
          },
          "practical_pass": true
        },
        "resources": {
          "depth": 9,
          "gates": 10,
          "two_qubit": 2
        },
        "threshold": 0.0322265625
      },
      {
        "layers": 1,
        "noise": 0.005,
        "payload": "z",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 1.0,
            "rejected": 128,
            "wilson95": [
              0.9708630437264916,
              1.0
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.015625,
            "rejected": 2,
            "wilson95": [
              0.004295449710144082,
              0.055180976679817195
            ]
          },
          "practical_pass": true
        },
        "resources": {
          "depth": 8,
          "gates": 9,
          "two_qubit": 2
        },
        "threshold": 0.0322265625
      },
      {
        "layers": 2,
        "noise": 0.005,
        "payload": "i",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 0.0078125,
            "rejected": 1,
            "wilson95": [
              0.0013804318928742412,
              0.04292625943886061
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.015625,
            "rejected": 2,
            "wilson95": [
              0.004295449710144082,
              0.055180976679817195
            ]
          },
          "practical_pass": false
        },
        "resources": {
          "depth": 11,
          "gates": 14,
          "two_qubit": 4
        },
        "threshold": 0.0693359375
      },
      {
        "layers": 2,
        "noise": 0.005,
        "payload": "x",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 1.0,
            "rejected": 128,
            "wilson95": [
              0.9708630437264916,
              1.0
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.015625,
            "rejected": 2,
            "wilson95": [
              0.004295449710144082,
              0.055180976679817195
            ]
          },
          "practical_pass": true
        },
        "resources": {
          "depth": 12,
          "gates": 15,
          "two_qubit": 4
        },
        "threshold": 0.0693359375
      },
      {
        "layers": 2,
        "noise": 0.005,
        "payload": "y",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 1.0,
            "rejected": 128,
            "wilson95": [
              0.9708630437264916,
              1.0
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.015625,
            "rejected": 2,
            "wilson95": [
              0.004295449710144082,
              0.055180976679817195
            ]
          },
          "practical_pass": true
        },
        "resources": {
          "depth": 13,
          "gates": 16,
          "two_qubit": 4
        },
        "threshold": 0.0693359375
      },
      {
        "layers": 2,
        "noise": 0.005,
        "payload": "z",
        "rates": {
          "attack": {
            "n": 128,
            "rate": 1.0,
            "rejected": 128,
            "wilson95": [
              0.9708630437264916,
              1.0
            ]
          },
          "clean": {
            "n": 128,
            "rate": 0.015625,
            "rejected": 2,
            "wilson95": [
              0.004295449710144082,
              0.055180976679817195
            ]
          },
          "practical_pass": true
        },
        "resources": {
          "depth": 12,
          "gates": 15,
          "two_qubit": 4
        },
        "threshold": 0.0693359375
      }
    ],
    "dynamic": {
      "all_data_reset": true,
      "counts": [
        [
          0,
          0,
          1024,
          0,
          0,
          0,
          0,
          0
        ]
      ],
      "flag_preserved": true
    },
    "note": "CZ and Y lowered to supported primitive gates; terminal reset mapping exact for this restricted gadget, not arbitrary dynamic circuits."
  },
  "comparisons": [
    {
      "condition": {
        "baseline": "fixed",
        "name": "phase",
        "noise": 0.0
      },
      "delta": 1.0,
      "gains": 128,
      "losses": 0,
      "n": 128,
      "p_holm": 7.640713280344891e-38,
      "p_one_sided": 2.938735877055727e-39
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "phase",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "bitflip",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "bitflip",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "small_rotation",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 1,
      "losses": 1,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 0.75
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "small_rotation",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 1,
      "losses": 1,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 0.75
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "dormant_cx",
        "noise": 0.0
      },
      "delta": 1.0,
      "gains": 128,
      "losses": 0,
      "n": 128,
      "p_holm": 7.640713280344891e-38,
      "p_one_sided": 2.938735877055727e-39
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "dormant_cx",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "legit_hh",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "legit_hh",
        "noise": 0.0
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "legit_rz_composition",
        "noise": 0.0
      },
      "delta": 0.046875,
      "gains": 6,
      "losses": 0,
      "n": 128,
      "p_holm": 0.25069469501619424,
      "p_one_sided": 0.015625000000000007
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "legit_rz_composition",
        "noise": 0.0
      },
      "delta": -0.0234375,
      "gains": 5,
      "losses": 8,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 0.8665771484374998
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "phase",
        "noise": 0.005
      },
      "delta": 0.90625,
      "gains": 116,
      "losses": 0,
      "n": 128,
      "p_holm": 2.8888949165808037e-34,
      "p_one_sided": 1.2037062152420016e-35
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "phase",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "bitflip",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "bitflip",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "small_rotation",
        "noise": 0.005
      },
      "delta": 0.1015625,
      "gains": 14,
      "losses": 1,
      "n": 128,
      "p_holm": 0.008789062500000007,
      "p_one_sided": 0.0004882812500000004
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "small_rotation",
        "noise": 0.005
      },
      "delta": 0.6796875,
      "gains": 88,
      "losses": 1,
      "n": 128,
      "p_holm": 3.344265367157769e-24,
      "p_one_sided": 1.454028420503378e-25
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "dormant_cx",
        "noise": 0.005
      },
      "delta": 0.3671875,
      "gains": 47,
      "losses": 0,
      "n": 128,
      "p_holm": 1.421085471520207e-13,
      "p_one_sided": 7.105427357601035e-15
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "dormant_cx",
        "noise": 0.005
      },
      "delta": 0.0,
      "gains": 0,
      "losses": 0,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 1.0
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "legit_hh",
        "noise": 0.005
      },
      "delta": 0.1484375,
      "gains": 44,
      "losses": 25,
      "n": 128,
      "p_holm": 0.25069469501619424,
      "p_one_sided": 0.014746746765658486
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "legit_hh",
        "noise": 0.005
      },
      "delta": 0.3359375,
      "gains": 51,
      "losses": 8,
      "n": 128,
      "p_holm": 8.599771608199004e-8,
      "p_one_sided": 4.526195583262634e-9
    },
    {
      "condition": {
        "baseline": "fixed",
        "name": "legit_rz_composition",
        "noise": 0.005
      },
      "delta": 0.0390625,
      "gains": 5,
      "losses": 0,
      "n": 128,
      "p_holm": 0.46875,
      "p_one_sided": 0.03125
    },
    {
      "condition": {
        "baseline": "multiple",
        "name": "legit_rz_composition",
        "noise": 0.005
      },
      "delta": 0.0234375,
      "gains": 5,
      "losses": 2,
      "n": 128,
      "p_holm": 1.0,
      "p_one_sided": 0.22656250000000008
    },
    {
      "condition": {
        "baseline": 256,
        "candidate": 4096,
        "noise": 0.0,
        "study": "shots"
      },
      "delta": 0.390625,
      "gains": 50,
      "losses": 0,
      "n": 128,
      "p_holm": 1.8651746813702614e-14,
      "p_one_sided": 8.881784197001244e-16
    },
    {
      "condition": {
        "baseline": 256,
        "candidate": 4096,
        "noise": 0.005,
        "study": "shots"
      },
      "delta": 0.5,
      "gains": 64,
      "losses": 0,
      "n": 128,
      "p_holm": 1.1926223897340568e-18,
      "p_one_sided": 5.42101086242753e-20
    }
  ],
  "deferred": {
    "H33": "private watermark not implemented",
    "H35": "quantum authentication protocol not implemented",
    "H36": "not applicable to classical API",
    "H37": "quantum advantage not demonstrated"
  },
  "factorial": {
    "effects": [
      0.0,
      0.0056144610319787845,
      0.0056144610319787845,
      0.022331755437196833
    ],
    "interaction": 0.011102833373239262
  },
  "genome": {
    "dependencies": [
      [
        0,
        1
      ],
      [
        1,
        2
      ]
    ],
    "located": [
      "build",
      "compile",
      "submit"
    ],
    "recognized": 12
  },
  "mutations": {
    "holdouts": [
      {
        "balanced_accuracy": 0.5,
        "family": "basis",
        "model": {
          "ry": true,
          "x": true,
          "z": true
        },
        "test_n": 57,
        "train_n": 144
      },
      {
        "balanced_accuracy": 0.125,
        "family": "ghz",
        "model": {
          "ry": false,
          "x": true,
          "z": true
        },
        "test_n": 72,
        "train_n": 117
      },
      {
        "balanced_accuracy": 0.3064516129032258,
        "family": "variational",
        "model": {
          "ry": true,
          "x": true,
          "z": false
        },
        "test_n": 72,
        "train_n": 117
      }
    ],
    "n": 390,
    "reversals": 390
  },
  "normalization": {
    "incomplete_counterexample": true,
    "matches": 360,
    "n": 360,
    "operator_preserved": 360
  },
  "probes": [
    {
      "combined_rejections": 128,
      "exact_operator_changed": true,
      "fixture": "phase",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.0
        },
        "multiple": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            },
            {
              "basis": "Z",
              "input_state": "1"
            },
            {
              "basis": "X",
              "input_state": "+"
            },
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 65536,
          "threshold": 0.0
        },
        "selected": {
          "probes": [
            {
              "basis": "X",
              "input_state": "+"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 16384,
          "threshold": 0.0
        }
      },
      "noise": 0.0,
      "structural_unknown": true
    },
    {
      "combined_rejections": 128,
      "exact_operator_changed": true,
      "fixture": "bitflip",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 16384,
          "threshold": 0.0
        },
        "multiple": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            },
            {
              "basis": "Z",
              "input_state": "1"
            },
            {
              "basis": "X",
              "input_state": "+"
            },
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 65536,
          "threshold": 0.0
        },
        "selected": {
          "probes": [
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 16384,
          "threshold": 0.0
        }
      },
      "noise": 0.0,
      "structural_unknown": true
    },
    {
      "combined_rejections": 128,
      "exact_operator_changed": true,
      "fixture": "small_rotation",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.9921875,
              "rejected": 127,
              "wilson95": [
                0.9570737405611394,
                0.9986195681071257
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 16384,
          "threshold": 0.0
        },
        "multiple": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            },
            {
              "basis": "Z",
              "input_state": "1"
            },
            {
              "basis": "X",
              "input_state": "+"
            },
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.9921875,
              "rejected": 127,
              "wilson95": [
                0.9570737405611394,
                0.9986195681071257
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 65536,
          "threshold": 0.0
        },
        "selected": {
          "probes": [
            {
              "basis": "X",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.9921875,
              "rejected": 127,
              "wilson95": [
                0.9570737405611394,
                0.9986195681071257
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0390625,
              "rejected": 5,
              "wilson95": [
                0.016798610797803704,
                0.08818702076683686
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 16384,
          "threshold": 0.032958984375
        }
      },
      "noise": 0.0,
      "structural_unknown": true
    },
    {
      "combined_rejections": 128,
      "exact_operator_changed": true,
      "fixture": "dormant_cx",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "ZZ",
              "input_state": "00"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.0
        },
        "multiple": {
          "probes": [
            {
              "basis": "ZZ",
              "input_state": "00"
            },
            {
              "basis": "ZZ",
              "input_state": "11"
            },
            {
              "basis": "XX",
              "input_state": "++"
            },
            {
              "basis": "YY",
              "input_state": "ii"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 65536,
          "threshold": 0.0
        },
        "selected": {
          "probes": [
            {
              "basis": "ZZ",
              "input_state": "11"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 16384,
          "threshold": 0.0
        }
      },
      "noise": 0.0,
      "structural_unknown": true
    },
    {
      "combined_rejections": 0,
      "exact_operator_changed": false,
      "fixture": "legit_hh",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.0
        },
        "multiple": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            },
            {
              "basis": "Z",
              "input_state": "1"
            },
            {
              "basis": "X",
              "input_state": "+"
            },
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 65536,
          "threshold": 0.0
        },
        "selected": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.0
        }
      },
      "noise": 0.0,
      "structural_unknown": false
    },
    {
      "combined_rejections": 128,
      "exact_operator_changed": false,
      "fixture": "legit_rz_composition",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.0
        },
        "multiple": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            },
            {
              "basis": "Z",
              "input_state": "1"
            },
            {
              "basis": "X",
              "input_state": "+"
            },
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.0703125,
              "rejected": 9,
              "wilson95": [
                0.03742976830393997,
                0.12823480349360633
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0390625,
              "rejected": 5,
              "wilson95": [
                0.016798610797803704,
                0.08818702076683686
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 65536,
          "threshold": 0.03277587890625
        },
        "selected": {
          "probes": [
            {
              "basis": "X",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.046875,
              "rejected": 6,
              "wilson95": [
                0.021657844171923776,
                0.09849752245094322
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0546875,
              "rejected": 7,
              "wilson95": [
                0.026740201381346355,
                0.10858490029974707
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.02984619140625
        }
      },
      "noise": 0.0,
      "structural_unknown": true
    },
    {
      "combined_rejections": 128,
      "exact_operator_changed": true,
      "fixture": "phase",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.09375,
              "rejected": 12,
              "wilson95": [
                0.054443804399147026,
                0.15672997257307855
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0078125,
              "rejected": 1,
              "wilson95": [
                0.0013804318928742412,
                0.04292625943886061
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.0048828125
        },
        "multiple": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            },
            {
              "basis": "Z",
              "input_state": "1"
            },
            {
              "basis": "X",
              "input_state": "+"
            },
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0546875,
              "rejected": 7,
              "wilson95": [
                0.026740201381346355,
                0.10858490029974707
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 65536,
          "threshold": 0.0167236328125
        },
        "selected": {
          "probes": [
            {
              "basis": "X",
              "input_state": "+"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0234375,
              "rejected": 3,
              "wilson95": [
                0.008002416396755481,
                0.06664374505143222
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 16384,
          "threshold": 0.0057373046875
        }
      },
      "noise": 0.005,
      "structural_unknown": true
    },
    {
      "combined_rejections": 128,
      "exact_operator_changed": true,
      "fixture": "bitflip",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.015625,
              "rejected": 2,
              "wilson95": [
                0.004295449710144082,
                0.055180976679817195
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 16384,
          "threshold": 0.0047607421875
        },
        "multiple": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            },
            {
              "basis": "Z",
              "input_state": "1"
            },
            {
              "basis": "X",
              "input_state": "+"
            },
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.046875,
              "rejected": 6,
              "wilson95": [
                0.021657844171923776,
                0.09849752245094322
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 65536,
          "threshold": 0.01611328125
        },
        "selected": {
          "probes": [
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0078125,
              "rejected": 1,
              "wilson95": [
                0.0013804318928742412,
                0.04292625943886061
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 16384,
          "threshold": 0.00836181640625
        }
      },
      "noise": 0.005,
      "structural_unknown": true
    },
    {
      "combined_rejections": 128,
      "exact_operator_changed": true,
      "fixture": "small_rotation",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.890625,
              "rejected": 114,
              "wilson95": [
                0.8247655486183226,
                0.9337212042929989
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.078125,
              "rejected": 10,
              "wilson95": [
                0.04298737505098223,
                0.13784693180479052
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.00347900390625
        },
        "multiple": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            },
            {
              "basis": "Z",
              "input_state": "1"
            },
            {
              "basis": "X",
              "input_state": "+"
            },
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.3125,
              "rejected": 40,
              "wilson95": [
                0.23865530278536204,
                0.39727105581720357
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0234375,
              "rejected": 3,
              "wilson95": [
                0.008002416396755481,
                0.06664374505143222
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 65536,
          "threshold": 0.01727294921875
        },
        "selected": {
          "probes": [
            {
              "basis": "X",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.9921875,
              "rejected": 127,
              "wilson95": [
                0.9570737405611394,
                0.9986195681071257
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0625,
              "rejected": 8,
              "wilson95": [
                0.03200692569506119,
                0.11848791104425865
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.0313720703125
        }
      },
      "noise": 0.005,
      "structural_unknown": true
    },
    {
      "combined_rejections": 128,
      "exact_operator_changed": true,
      "fixture": "dormant_cx",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "ZZ",
              "input_state": "00"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.6328125,
              "rejected": 81,
              "wilson95": [
                0.5465700424986025,
                0.7113154534912468
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.046875,
              "rejected": 6,
              "wilson95": [
                0.021657844171923776,
                0.09849752245094322
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.006591796875
        },
        "multiple": {
          "probes": [
            {
              "basis": "ZZ",
              "input_state": "00"
            },
            {
              "basis": "ZZ",
              "input_state": "11"
            },
            {
              "basis": "XX",
              "input_state": "++"
            },
            {
              "basis": "YY",
              "input_state": "ii"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.015625,
              "rejected": 2,
              "wilson95": [
                0.004295449710144082,
                0.055180976679817195
              ]
            },
            "practical_pass": true
          },
          "reference_shots": 65536,
          "threshold": 0.02545166015625
        },
        "selected": {
          "probes": [
            {
              "basis": "ZZ",
              "input_state": "11"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 1.0,
              "rejected": 128,
              "wilson95": [
                0.9708630437264916,
                1.0
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0546875,
              "rejected": 7,
              "wilson95": [
                0.026740201381346355,
                0.10858490029974707
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.00689697265625
        }
      },
      "noise": 0.005,
      "structural_unknown": true
    },
    {
      "combined_rejections": 67,
      "exact_operator_changed": false,
      "fixture": "legit_hh",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.375,
              "rejected": 48,
              "wilson95": [
                0.29592429097646555,
                0.4613599480919116
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.03125,
              "rejected": 4,
              "wilson95": [
                0.01221826524505338,
                0.07759763126136077
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.0047607421875
        },
        "multiple": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            },
            {
              "basis": "Z",
              "input_state": "1"
            },
            {
              "basis": "X",
              "input_state": "+"
            },
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.1875,
              "rejected": 24,
              "wilson95": [
                0.12936141696376852,
                0.2638491807071743
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.03125,
              "rejected": 4,
              "wilson95": [
                0.01221826524505338,
                0.07759763126136077
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 65536,
          "threshold": 0.0164794921875
        },
        "selected": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.5234375,
              "rejected": 67,
              "wilson95": [
                0.43749793124049374,
                0.6080112739341855
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.03125,
              "rejected": 4,
              "wilson95": [
                0.01221826524505338,
                0.07759763126136077
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.00390625
        }
      },
      "noise": 0.005,
      "structural_unknown": false
    },
    {
      "combined_rejections": 128,
      "exact_operator_changed": false,
      "fixture": "legit_rz_composition",
      "methods": {
        "fixed": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.0,
              "rejected": 0,
              "wilson95": [
                0.0,
                0.029136956273508416
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0390625,
              "rejected": 5,
              "wilson95": [
                0.016798610797803704,
                0.08818702076683686
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.00762939453125
        },
        "multiple": {
          "probes": [
            {
              "basis": "Z",
              "input_state": "0"
            },
            {
              "basis": "Z",
              "input_state": "1"
            },
            {
              "basis": "X",
              "input_state": "+"
            },
            {
              "basis": "Y",
              "input_state": "i"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.015625,
              "rejected": 2,
              "wilson95": [
                0.004295449710144082,
                0.055180976679817195
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.0703125,
              "rejected": 9,
              "wilson95": [
                0.03742976830393997,
                0.12823480349360633
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 65536,
          "threshold": 0.0379638671875
        },
        "selected": {
          "probes": [
            {
              "basis": "X",
              "input_state": "0"
            }
          ],
          "rates": {
            "attack": {
              "n": 128,
              "rate": 0.0390625,
              "rejected": 5,
              "wilson95": [
                0.016798610797803704,
                0.08818702076683686
              ]
            },
            "clean": {
              "n": 128,
              "rate": 0.046875,
              "rejected": 6,
              "wilson95": [
                0.021657844171923776,
                0.09849752245094322
              ]
            },
            "practical_pass": false
          },
          "reference_shots": 16384,
          "threshold": 0.03448486328125
        }
      },
      "noise": 0.005,
      "structural_unknown": true
    }
  ],
  "supplements": {
    "diversity": {
      "diverse_adaptive_tvd": 0.09862664878510274,
      "diverse_fixed_tvd": 0.0,
      "equivalent": true,
      "original_fixed_tvd": 0.09862664878510274
    },
    "drift": {
      "benign_drift": {
        "n": 128,
        "rate": 1.0,
        "rejected": 128,
        "wilson95": [
          0.9708630437264916,
          1.0
        ]
      },
      "intent": "not identifiable from counts alone",
      "stable": {
        "n": 128,
        "rate": 0.0390625,
        "rejected": 5,
        "wilson95": [
          0.016798610797803704,
          0.08818702076683686
        ]
      }
    },
    "inspection": {
      "adaptive_visible_detection": 0,
      "binomial_p": 0.7904476831154448,
      "hits": 6708,
      "theory": 0.6695237889132748,
      "trials": 10000,
      "wilson95": [
        0.6615256320468428,
        0.679943194109734
      ]
    },
    "metamorphic": {
      "clean_tvd": 0.0,
      "correlated_forgery_tvd": 0.0,
      "forged_vs_truth_tvd": 0.8,
      "selective_tvd": 1.0
    },
    "resources": [
      {
        "clean": {
          "n": 128,
          "rate": 0.0,
          "rejected": 0,
          "wilson95": [
            0.0,
            0.029136956273508416
          ]
        },
        "noise": 0.0,
        "normalized_equal": true,
        "padded": {
          "n": 128,
          "rate": 0.0,
          "rejected": 0,
          "wilson95": [
            0.0,
            0.029136956273508416
          ]
        },
        "resources": {
          "depth": 100,
          "gates": 100,
          "two_qubit": 0
        }
      },
      {
        "clean": {
          "n": 128,
          "rate": 0.0234375,
          "rejected": 3,
          "wilson95": [
            0.008002416396755481,
            0.06664374505143222
          ]
        },
        "noise": 0.005,
        "normalized_equal": true,
        "padded": {
          "n": 128,
          "rate": 1.0,
          "rejected": 128,
          "wilson95": [
            0.9708630437264916,
            1.0
          ]
        },
        "resources": {
          "depth": 100,
          "gates": 100,
          "two_qubit": 0
        }
      }
    ],
    "scaling": [
      {
        "matrix_bytes": 256,
        "seconds": 3.499e-6,
        "width": 2
      },
      {
        "matrix_bytes": 1024,
        "seconds": 5.765e-6,
        "width": 3
      },
      {
        "matrix_bytes": 4096,
        "seconds": 0.000013516,
        "width": 4
      },
      {
        "matrix_bytes": 16384,
        "seconds": 0.000048542,
        "width": 5
      },
      {
        "matrix_bytes": 65536,
        "seconds": 0.000202442,
        "width": 6
      },
      {
        "matrix_bytes": 262144,
        "seconds": 0.000697963,
        "width": 7
      },
      {
        "matrix_bytes": 1048576,
        "seconds": 0.003165855,
        "width": 8
      }
    ],
    "shot_comparisons": [
      {
        "condition": {
          "baseline": 256,
          "candidate": 4096,
          "noise": 0.0,
          "study": "shots"
        },
        "delta": 0.390625,
        "gains": 50,
        "losses": 0,
        "n": 128,
        "p_one_sided": 8.881784197001244e-16
      },
      {
        "condition": {
          "baseline": 256,
          "candidate": 4096,
          "noise": 0.005,
          "study": "shots"
        },
        "delta": 0.5,
        "gains": 64,
        "losses": 0,
        "n": 128,
        "p_one_sided": 5.42101086242753e-20
      }
    ],
    "shot_scaling": [
      {
        "attack": {
          "n": 128,
          "rate": 0.609375,
          "rejected": 78,
          "wilson95": [
            0.5228464818201277,
            0.6895298089950423
          ]
        },
        "clean": {
          "n": 128,
          "rate": 0.046875,
          "rejected": 6,
          "wilson95": [
            0.021657844171923776,
            0.09849752245094322
          ]
        },
        "noise": 0.0,
        "shots": 256
      },
      {
        "attack": {
          "n": 128,
          "rate": 1.0,
          "rejected": 128,
          "wilson95": [
            0.9708630437264916,
            1.0
          ]
        },
        "clean": {
          "n": 128,
          "rate": 0.0234375,
          "rejected": 3,
          "wilson95": [
            0.008002416396755481,
            0.06664374505143222
          ]
        },
        "noise": 0.0,
        "shots": 4096
      },
      {
        "attack": {
          "n": 128,
          "rate": 0.5,
          "rejected": 64,
          "wilson95": [
            0.41465224626050723,
            0.5853477537394928
          ]
        },
        "clean": {
          "n": 128,
          "rate": 0.0078125,
          "rejected": 1,
          "wilson95": [
            0.0013804318928742412,
            0.04292625943886061
          ]
        },
        "noise": 0.005,
        "shots": 256
      },
      {
        "attack": {
          "n": 128,
          "rate": 1.0,
          "rejected": 128,
          "wilson95": [
            0.9708630437264916,
            1.0
          ]
        },
        "clean": {
          "n": 128,
          "rate": 0.0390625,
          "rejected": 5,
          "wilson95": [
            0.016798610797803704,
            0.08818702076683686
          ]
        },
        "noise": 0.005,
        "shots": 4096
      }
    ],
    "symmetry": [
      {
        "basis": "Z",
        "odd_probability": 0.0,
        "payload": "i"
      },
      {
        "basis": "X",
        "odd_probability": 0.0,
        "payload": "i"
      },
      {
        "basis": "Z",
        "odd_probability": 1.0000000000000002,
        "payload": "x"
      },
      {
        "basis": "X",
        "odd_probability": 0.0,
        "payload": "x"
      },
      {
        "basis": "Z",
        "odd_probability": 0.0,
        "payload": "z"
      },
      {
        "basis": "X",
        "odd_probability": 1.0000000000000004,
        "payload": "z"
      }
    ]
  }
}
```

## supplemental

```json
{
  "boundaries": [
    {
      "case": "modified_dependency",
      "rejected": true
    },
    {
      "accepted": true,
      "case": "trusted_manifest_already_malicious"
    },
    {
      "case": "check_then_mutate",
      "disk_matches": true,
      "memory_not_attested": true
    },
    {
      "case": "memory_ledger_restart",
      "replay_accepted": true
    },
    {
      "case": "function_guard",
      "closure_or_external_state_not_measured": true,
      "replacement_detected": true,
      "supplied_code_change_detected": true
    },
    {
      "case": "passive_hook",
      "counts_copied": true,
      "decision": 0
    },
    {
      "case": "post_validation_hook",
      "detected": false,
      "tampered_decision": 1,
      "validated_decision": 0
    },
    {
      "case": "supervisor_bypass",
      "direct_consumer_accepts_without_guard": true,
      "python_monkey_patch": "language-specific; modeled as explicit bypass in native code"
    },
    {
      "basis": "Z",
      "case": "phase_basis",
      "rate": {
        "n": 100,
        "rate": 0.09,
        "rejected": 9,
        "wilson95": [
          0.04807254000256516,
          0.1622621285271631
        ]
      }
    },
    {
      "basis": "X",
      "case": "phase_basis",
      "rate": {
        "n": 100,
        "rate": 1.0,
        "rejected": 100,
        "wilson95": [
          0.9630065017930143,
          1.0
        ]
      }
    }
  ],
  "countermeasures": [
    {
      "attack": {
        "n": 40,
        "rate": 0.075,
        "rejected": 3,
        "wilson95": [
          0.025836025774588198,
          0.19864233524310543
        ]
      },
      "case": "shot_budget",
      "clean": {
        "n": 40,
        "rate": 0.075,
        "rejected": 3,
        "wilson95": [
          0.025836025774588198,
          0.19864233524310543
        ]
      },
      "noise": 0.0,
      "shots": 256,
      "threshold": 0.1267626907450124
    },
    {
      "attack": {
        "n": 40,
        "rate": 0.05,
        "rejected": 2,
        "wilson95": [
          0.013820667386148344,
          0.16503877369140962
        ]
      },
      "case": "shot_budget",
      "clean": {
        "n": 40,
        "rate": 0.025,
        "rejected": 1,
        "wilson95": [
          0.0044268315026814095,
          0.1288136896347409
        ]
      },
      "noise": 0.0,
      "shots": 1024,
      "threshold": 0.06300567864530829
    },
    {
      "attack": {
        "n": 40,
        "rate": 0.275,
        "rejected": 11,
        "wilson95": [
          0.16108016247667337,
          0.42834955806210573
        ]
      },
      "case": "shot_budget",
      "clean": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "noise": 0.0,
      "shots": 8192,
      "threshold": 0.022037005863933427
    },
    {
      "attack": {
        "n": 40,
        "rate": 0.925,
        "rejected": 37,
        "wilson95": [
          0.8013576647568946,
          0.9741639742254119
        ]
      },
      "case": "shot_budget",
      "clean": {
        "n": 40,
        "rate": 0.125,
        "rejected": 5,
        "wilson95": [
          0.05459500250945401,
          0.261121198388511
        ]
      },
      "noise": 0.0,
      "shots": 32768,
      "threshold": 0.010478652101198617
    },
    {
      "attack": {
        "n": 40,
        "rate": 0.025,
        "rejected": 1,
        "wilson95": [
          0.0044268315026814095,
          0.1288136896347409
        ]
      },
      "case": "shot_budget",
      "clean": {
        "n": 40,
        "rate": 0.05,
        "rejected": 2,
        "wilson95": [
          0.013820667386148344,
          0.16503877369140962
        ]
      },
      "noise": 0.005,
      "shots": 256,
      "threshold": 0.13554478474678242
    },
    {
      "attack": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "case": "shot_budget",
      "clean": {
        "n": 40,
        "rate": 0.025,
        "rejected": 1,
        "wilson95": [
          0.0044268315026814095,
          0.1288136896347409
        ]
      },
      "noise": 0.005,
      "shots": 1024,
      "threshold": 0.08059583106430547
    },
    {
      "attack": {
        "n": 40,
        "rate": 0.025,
        "rejected": 1,
        "wilson95": [
          0.0044268315026814095,
          0.1288136896347409
        ]
      },
      "case": "shot_budget",
      "clean": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "noise": 0.005,
      "shots": 8192,
      "threshold": 0.04361488938963813
    },
    {
      "attack": {
        "n": 40,
        "rate": 0.35,
        "rejected": 14,
        "wilson95": [
          0.2213452887317629,
          0.5049411916274231
        ]
      },
      "case": "shot_budget",
      "clean": {
        "n": 40,
        "rate": 0.075,
        "rejected": 3,
        "wilson95": [
          0.025836025774588198,
          0.19864233524310543
        ]
      },
      "noise": 0.005,
      "shots": 32768,
      "threshold": 0.03687359415148643
    },
    {
      "case": "random_expected_traps",
      "n": 30,
      "public_inference_rejected": 0,
      "zero_rejected": 30
    },
    {
      "case": "benign_noise_drift",
      "rate": {
        "n": 100,
        "rate": 1.0,
        "rejected": 100,
        "wilson95": [
          0.9630065017930143,
          1.0
        ]
      }
    },
    {
      "attack": {
        "n": 40,
        "rate": 0.075,
        "rejected": 3,
        "wilson95": [
          0.025836025774588198,
          0.19864233524310543
        ]
      },
      "case": "learned_noise_reference",
      "clean": {
        "n": 40,
        "rate": 0.025,
        "rejected": 1,
        "wilson95": [
          0.0044268315026814095,
          0.1288136896347409
        ]
      },
      "shots": 1024
    },
    {
      "attack": {
        "n": 40,
        "rate": 0.15,
        "rejected": 6,
        "wilson95": [
          0.07061187717320358,
          0.2907232436648971
        ]
      },
      "case": "learned_noise_reference",
      "clean": {
        "n": 40,
        "rate": 0.05,
        "rejected": 2,
        "wilson95": [
          0.013820667386148344,
          0.16503877369140962
        ]
      },
      "shots": 8192
    },
    {
      "attack": {
        "n": 40,
        "rate": 0.25,
        "rejected": 10,
        "wilson95": [
          0.14187118639096302,
          0.40193961420768026
        ]
      },
      "case": "learned_noise_reference",
      "clean": {
        "n": 40,
        "rate": 0.025,
        "rejected": 1,
        "wilson95": [
          0.0044268315026814095,
          0.1288136896347409
        ]
      },
      "shots": 32768
    }
  ],
  "noisy_masks": [
    {
      "mean_recovered_tvd": 0.051171875,
      "noise": 0.0,
      "pairs": 50
    },
    {
      "mean_recovered_tvd": 0.0512109375,
      "noise": 0.005,
      "pairs": 50
    }
  ],
  "obfuscation": [
    {
      "family": "basis",
      "known_template_keys": [
        1
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 0,
      "optimized_padded": 0,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 0,
      "padded_gates": 0,
      "padding_equivalent": true,
      "seed": 17,
      "width": 2
    },
    {
      "family": "basis",
      "known_template_keys": [
        1
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 1,
      "optimized_padded": 1,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 1,
      "padded_gates": 3,
      "padding_equivalent": true,
      "seed": 71,
      "width": 2
    },
    {
      "family": "basis",
      "known_template_keys": [
        3
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 0,
      "optimized_padded": 0,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 0,
      "padded_gates": 0,
      "padding_equivalent": true,
      "seed": 17,
      "width": 3
    },
    {
      "family": "basis",
      "known_template_keys": [
        3
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 2,
      "optimized_padded": 2,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 2,
      "padded_gates": 6,
      "padding_equivalent": true,
      "seed": 71,
      "width": 3
    },
    {
      "family": "basis",
      "known_template_keys": [
        7
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 0,
      "optimized_padded": 0,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 0,
      "padded_gates": 0,
      "padding_equivalent": true,
      "seed": 17,
      "width": 4
    },
    {
      "family": "basis",
      "known_template_keys": [
        7
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 3,
      "optimized_padded": 3,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 3,
      "padded_gates": 9,
      "padding_equivalent": true,
      "seed": 71,
      "width": 4
    },
    {
      "family": "ghz",
      "known_template_keys": [
        1,
        2
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 2,
      "optimized_padded": 2,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 2,
      "padded_gates": 6,
      "padding_equivalent": true,
      "seed": 17,
      "width": 2
    },
    {
      "family": "ghz",
      "known_template_keys": [
        1,
        2
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 2,
      "optimized_padded": 2,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 2,
      "padded_gates": 6,
      "padding_equivalent": true,
      "seed": 71,
      "width": 2
    },
    {
      "family": "ghz",
      "known_template_keys": [
        3,
        4
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 3,
      "optimized_padded": 3,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 3,
      "padded_gates": 9,
      "padding_equivalent": true,
      "seed": 17,
      "width": 3
    },
    {
      "family": "ghz",
      "known_template_keys": [
        3,
        4
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 3,
      "optimized_padded": 3,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 3,
      "padded_gates": 9,
      "padding_equivalent": true,
      "seed": 71,
      "width": 3
    },
    {
      "family": "ghz",
      "known_template_keys": [
        7,
        8
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 4,
      "optimized_padded": 4,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 4,
      "padded_gates": 12,
      "padding_equivalent": true,
      "seed": 17,
      "width": 4
    },
    {
      "family": "ghz",
      "known_template_keys": [
        7,
        8
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 4,
      "optimized_padded": 4,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 4,
      "padded_gates": 12,
      "padding_equivalent": true,
      "seed": 71,
      "width": 4
    },
    {
      "family": "variational",
      "known_template_keys": [
        1
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 10,
      "optimized_padded": 10,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 10,
      "padded_gates": 30,
      "padding_equivalent": true,
      "seed": 17,
      "width": 2
    },
    {
      "family": "variational",
      "known_template_keys": [
        1
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 10,
      "optimized_padded": 10,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 10,
      "padded_gates": 30,
      "padding_equivalent": true,
      "seed": 71,
      "width": 2
    },
    {
      "family": "variational",
      "known_template_keys": [
        3
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 16,
      "optimized_padded": 16,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 16,
      "padded_gates": 48,
      "padding_equivalent": true,
      "seed": 17,
      "width": 3
    },
    {
      "family": "variational",
      "known_template_keys": [
        3
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 16,
      "optimized_padded": 16,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 16,
      "padded_gates": 48,
      "padding_equivalent": true,
      "seed": 71,
      "width": 3
    },
    {
      "family": "variational",
      "known_template_keys": [
        7
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 22,
      "optimized_padded": 22,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 22,
      "padded_gates": 66,
      "padding_equivalent": true,
      "seed": 17,
      "width": 4
    },
    {
      "family": "variational",
      "known_template_keys": [
        7
      ],
      "mask_recovery_error": 0.0,
      "optimized_original": 22,
      "optimized_padded": 22,
      "optimizer": "conservative native rewrites, not Qiskit level 3",
      "original_gates": 22,
      "padded_gates": 66,
      "padding_equivalent": true,
      "seed": 71,
      "width": 4
    }
  ],
  "predicates": [
    {
      "classical_constant_true": 100,
      "noise": 0.0,
      "predicate_true": 100,
      "shots": 256
    },
    {
      "classical_constant_true": 100,
      "noise": 0.0,
      "predicate_true": 100,
      "shots": 1024
    },
    {
      "classical_constant_true": 100,
      "noise": 0.005,
      "predicate_true": 2,
      "shots": 256
    },
    {
      "classical_constant_true": 100,
      "noise": 0.005,
      "predicate_true": 0,
      "shots": 1024
    }
  ],
  "traps": [
    {
      "attack": "clean",
      "noise": 0.0,
      "rate": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "thresholds": [
        0.0,
        0.0,
        0.0
      ],
      "width": 2
    },
    {
      "attack": "indiscriminate_x",
      "noise": 0.0,
      "rate": {
        "n": 40,
        "rate": 1.0,
        "rejected": 40,
        "wilson95": [
          0.9123783988027135,
          1.0000000000000002
        ]
      },
      "thresholds": [
        0.0,
        0.0,
        0.0
      ],
      "width": 2
    },
    {
      "attack": "selective_preserve_traps",
      "noise": 0.0,
      "rate": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "thresholds": [
        0.0,
        0.0,
        0.0
      ],
      "width": 2
    },
    {
      "attack": "forge_all_zeros",
      "noise": 0.0,
      "rate": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "thresholds": [
        0.0,
        0.0,
        0.0
      ],
      "width": 2
    },
    {
      "attack": "clean",
      "noise": 0.0,
      "rate": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "thresholds": [
        0.0,
        0.0,
        0.0
      ],
      "width": 4
    },
    {
      "attack": "indiscriminate_x",
      "noise": 0.0,
      "rate": {
        "n": 40,
        "rate": 1.0,
        "rejected": 40,
        "wilson95": [
          0.9123783988027135,
          1.0000000000000002
        ]
      },
      "thresholds": [
        0.0,
        0.0,
        0.0
      ],
      "width": 4
    },
    {
      "attack": "selective_preserve_traps",
      "noise": 0.0,
      "rate": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "thresholds": [
        0.0,
        0.0,
        0.0
      ],
      "width": 4
    },
    {
      "attack": "forge_all_zeros",
      "noise": 0.0,
      "rate": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "thresholds": [
        0.0,
        0.0,
        0.0
      ],
      "width": 4
    },
    {
      "attack": "clean",
      "noise": 0.005,
      "rate": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "thresholds": [
        0.119140625,
        0.11328125,
        0.107421875
      ],
      "width": 2
    },
    {
      "attack": "indiscriminate_x",
      "noise": 0.005,
      "rate": {
        "n": 40,
        "rate": 1.0,
        "rejected": 40,
        "wilson95": [
          0.9123783988027135,
          1.0000000000000002
        ]
      },
      "thresholds": [
        0.119140625,
        0.11328125,
        0.107421875
      ],
      "width": 2
    },
    {
      "attack": "selective_preserve_traps",
      "noise": 0.005,
      "rate": {
        "n": 40,
        "rate": 0.025,
        "rejected": 1,
        "wilson95": [
          0.0044268315026814095,
          0.1288136896347409
        ]
      },
      "thresholds": [
        0.119140625,
        0.11328125,
        0.107421875
      ],
      "width": 2
    },
    {
      "attack": "forge_all_zeros",
      "noise": 0.005,
      "rate": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "thresholds": [
        0.119140625,
        0.11328125,
        0.107421875
      ],
      "width": 2
    },
    {
      "attack": "clean",
      "noise": 0.005,
      "rate": {
        "n": 40,
        "rate": 0.025,
        "rejected": 1,
        "wilson95": [
          0.0044268315026814095,
          0.1288136896347409
        ]
      },
      "thresholds": [
        0.232421875,
        0.23828125,
        0.21484375
      ],
      "width": 4
    },
    {
      "attack": "indiscriminate_x",
      "noise": 0.005,
      "rate": {
        "n": 40,
        "rate": 1.0,
        "rejected": 40,
        "wilson95": [
          0.9123783988027135,
          1.0000000000000002
        ]
      },
      "thresholds": [
        0.232421875,
        0.23828125,
        0.21484375
      ],
      "width": 4
    },
    {
      "attack": "selective_preserve_traps",
      "noise": 0.005,
      "rate": {
        "n": 40,
        "rate": 0.1,
        "rejected": 4,
        "wilson95": [
          0.03957952868260638,
          0.23051775227522298
        ]
      },
      "thresholds": [
        0.232421875,
        0.23828125,
        0.21484375
      ],
      "width": 4
    },
    {
      "attack": "forge_all_zeros",
      "noise": 0.005,
      "rate": {
        "n": 40,
        "rate": 0.0,
        "rejected": 0,
        "wilson95": [
          0.0,
          0.08762160119728664
        ]
      },
      "thresholds": [
        0.232421875,
        0.23828125,
        0.21484375
      ],
      "width": 4
    }
  ]
}
```

## trojans

```json
{
  "configurations": 36,
  "controlled_trigger": [
    {
      "active": false,
      "ideal_tvd": 0.0,
      "rate": {
        "n": 100,
        "rate": 0.35,
        "rejected": 35,
        "wilson95": [
          0.26364248243097527,
          0.44745556703112027
        ]
      }
    },
    {
      "active": true,
      "ideal_tvd": 1.0,
      "rate": {
        "n": 100,
        "rate": 1.0,
        "rejected": 100,
        "wilson95": [
          0.9630065017930143,
          1.0
        ]
      }
    }
  ],
  "groups": {
    "h/end/0": {
      "n": 216,
      "rate": 1.0,
      "rejected": 216,
      "wilson95": [
        0.9825262312154358,
        1.0000000000000002
      ]
    },
    "h/end/0.005": {
      "n": 216,
      "rate": 1.0,
      "rejected": 216,
      "wilson95": [
        0.9825262312154358,
        1.0000000000000002
      ]
    },
    "h/middle/0": {
      "n": 216,
      "rate": 1.0,
      "rejected": 216,
      "wilson95": [
        0.9825262312154358,
        1.0000000000000002
      ]
    },
    "h/middle/0.005": {
      "n": 216,
      "rate": 0.9953703703703703,
      "rejected": 215,
      "wilson95": [
        0.974246477434219,
        0.99918228867737
      ]
    },
    "h/start/0": {
      "n": 216,
      "rate": 1.0,
      "rejected": 216,
      "wilson95": [
        0.9825262312154358,
        1.0000000000000002
      ]
    },
    "h/start/0.005": {
      "n": 216,
      "rate": 0.9861111111111112,
      "rejected": 213,
      "wilson95": [
        0.9599683531668719,
        0.995265482737024
      ]
    },
    "t/end/0": {
      "n": 216,
      "rate": 0.06018518518518518,
      "rejected": 13,
      "wilson95": [
        0.03550638154440902,
        0.10023443359016143
      ]
    },
    "t/end/0.005": {
      "n": 216,
      "rate": 0.06944444444444445,
      "rejected": 15,
      "wilson95": [
        0.0425325659555028,
        0.11140317938676092
      ]
    },
    "t/middle/0": {
      "n": 216,
      "rate": 0.2175925925925926,
      "rejected": 47,
      "wilson95": [
        0.16776243779763317,
        0.2772921908677226
      ]
    },
    "t/middle/0.005": {
      "n": 216,
      "rate": 0.2222222222222222,
      "rejected": 48,
      "wilson95": [
        0.17190615308775226,
        0.28224594068145015
      ]
    },
    "t/start/0": {
      "n": 216,
      "rate": 0.018518518518518517,
      "rejected": 4,
      "wilson95": [
        0.007224468618003889,
        0.04663916058194693
      ]
    },
    "t/start/0.005": {
      "n": 216,
      "rate": 0.046296296296296294,
      "rejected": 10,
      "wilson95": [
        0.025338742275287277,
        0.0831096775477433
      ]
    },
    "x/end/0": {
      "n": 216,
      "rate": 1.0,
      "rejected": 216,
      "wilson95": [
        0.9825262312154358,
        1.0000000000000002
      ]
    },
    "x/end/0.005": {
      "n": 216,
      "rate": 1.0,
      "rejected": 216,
      "wilson95": [
        0.9825262312154358,
        1.0000000000000002
      ]
    },
    "x/middle/0": {
      "n": 216,
      "rate": 0.6898148148148148,
      "rejected": 149,
      "wilson95": [
        0.6252618589631287,
        0.747734210294583
      ]
    },
    "x/middle/0.005": {
      "n": 216,
      "rate": 0.6759259259259259,
      "rejected": 146,
      "wilson95": [
        0.6109076663545039,
        0.734796007591668
      ]
    },
    "x/start/0": {
      "n": 216,
      "rate": 0.6759259259259259,
      "rejected": 146,
      "wilson95": [
        0.6109076663545039,
        0.734796007591668
      ]
    },
    "x/start/0.005": {
      "n": 216,
      "rate": 0.6666666666666666,
      "rejected": 144,
      "wilson95": [
        0.6013721533218815,
        0.726136590416597
      ]
    },
    "y/end/0": {
      "n": 216,
      "rate": 1.0,
      "rejected": 216,
      "wilson95": [
        0.9825262312154358,
        1.0000000000000002
      ]
    },
    "y/end/0.005": {
      "n": 216,
      "rate": 1.0,
      "rejected": 216,
      "wilson95": [
        0.9825262312154358,
        1.0000000000000002
      ]
    },
    "y/middle/0": {
      "n": 216,
      "rate": 0.7685185185185185,
      "rejected": 166,
      "wilson95": [
        0.7078750781468751,
        0.8197778978762292
      ]
    },
    "y/middle/0.005": {
      "n": 216,
      "rate": 0.7638888888888888,
      "rejected": 165,
      "wilson95": [
        0.7029494511137879,
        0.8156060598054697
      ]
    },
    "y/start/0": {
      "n": 216,
      "rate": 0.6851851851851852,
      "rejected": 148,
      "wilson95": [
        0.6204702363472716,
        0.7434283678065936
      ]
    },
    "y/start/0.005": {
      "n": 216,
      "rate": 0.6712962962962963,
      "rejected": 145,
      "wilson95": [
        0.6061365642936319,
        0.7304696445486932
      ]
    },
    "z/end/0": {
      "n": 216,
      "rate": 0.06018518518518518,
      "rejected": 13,
      "wilson95": [
        0.03550638154440902,
        0.10023443359016143
      ]
    },
    "z/end/0.005": {
      "n": 216,
      "rate": 0.06018518518518518,
      "rejected": 13,
      "wilson95": [
        0.03550638154440902,
        0.10023443359016143
      ]
    },
    "z/middle/0": {
      "n": 216,
      "rate": 0.25925925925925924,
      "rejected": 56,
      "wilson95": [
        0.20538470105804496,
        0.32154711354193044
      ]
    },
    "z/middle/0.005": {
      "n": 216,
      "rate": 0.3055555555555556,
      "rejected": 66,
      "wilson95": [
        0.24796694797587115,
        0.3699395176625706
      ]
    },
    "z/start/0": {
      "n": 216,
      "rate": 0.027777777777777776,
      "rejected": 6,
      "wilson95": [
        0.012791661409577167,
        0.0592668979980669
      ]
    },
    "z/start/0.005": {
      "n": 216,
      "rate": 0.06481481481481481,
      "rejected": 14,
      "wilson95": [
        0.03899791567013807,
        0.10584036456827903
      ]
    }
  },
  "idle_insertion": {
    "changed_depth": 2,
    "optimized_preserves_infection": true,
    "original_depth": 2
  },
  "scope": "Owned fixtures; native T=RZ(pi/4), Y=XZ up to global phase; decomposed noise differs from a native Y gate. No external dataset reproduction.",
  "trials": 6480
}
```

## Measured simulation cost

```json
{
  "circuits": 129200,
  "includes": "all logged native samples including calibration/reference; exact oracle computation not shots",
  "jobs": 1993,
  "seconds": 0.680108067,
  "shots": 77388288
}
```
