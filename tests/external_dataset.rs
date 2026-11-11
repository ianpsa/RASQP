//! A complete tiny external-data run verifies provenance checks, actual CLI
//! execution, persistent raw counts and shot accounting without external data.
#![cfg(target_os = "linux")]
use rasqp::{Circuit, Gate, artifact_digest};
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader},
    process::Command,
};

fn prepare(path: &std::path::Path) {
    std::fs::create_dir_all(path).unwrap();
    let base = Circuit::new(2, vec![Gate::h(0), Gate::cx(0, 1)]).unwrap();
    let mut samples =
        vec![json!({"sample_id":"golden","label":0,"gate":"","mode":"","circuit":base})];
    for i in 0..10 {
        samples.push(json!({"sample_id":format!("infected-{i}"),"label":1,"gate":"X","mode":"normal","circuit":base.appended([Gate::x(0)]).unwrap()}));
    }
    let input = format!(
        "{}\n",
        json!({"base_circuit_id":"fixture","fold":1,"samples":samples})
    );
    std::fs::write(path.join("input.jsonl"), &input).unwrap();
    std::fs::write(
        path.join("preparation.json"),
        json!({
        "protocol_sha256":artifact_digest(include_bytes!("../docs/EXTERNAL_DATASET_PROTOCOL_V1.json")),
        "input_sha256":artifact_digest(input.as_bytes())})
        .to_string(),
    )
    .unwrap();
}

#[test]
fn external_cli_records_all_counts_and_checks_bound_input() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input");
    let output = directory.path().join("output");
    prepare(&input);
    let result = Command::new(env!("CARGO_BIN_EXE_rasqp-external"))
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let summary: Value =
        serde_json::from_slice(&std::fs::read(output.join("native_summary.json")).unwrap())
            .unwrap();
    assert_eq!(summary["outcomes"].as_array().unwrap().len(), 24);
    let raw = flate2::read::GzDecoder::new(
        std::fs::File::open(output.join("native_trials.jsonl.gz")).unwrap(),
    );
    let mut shots = 0;
    let mut decisions = 0;
    for line in BufReader::new(raw).lines() {
        let row: Value = serde_json::from_str(&line.unwrap()).unwrap();
        if row["kind"] == "samples" {
            let batches = row["counts"].as_array().unwrap();
            let requested = row["shots"].as_u64().unwrap();
            assert!(batches.iter().all(|c| {
                c.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_u64().unwrap())
                    .sum::<u64>()
                    == requested
            }));
            shots += batches.len() as u64 * requested;
        } else if row["kind"] == "decision" {
            decisions += 1;
        }
    }
    assert_eq!(decisions, 24);
    assert_eq!(summary["cost"]["shots"], shots);
    assert_eq!(shots, 3_978_240);
    // A changed input must fail before the evaluation opens output artifacts.
    std::fs::write(input.join("input.jsonl"), b"{}\n").unwrap();
    let bad_output = directory.path().join("tampered-output");
    let failed = Command::new(env!("CARGO_BIN_EXE_rasqp-external"))
        .arg(&input)
        .arg(&bad_output)
        .output()
        .unwrap();
    assert!(!failed.status.success());
    assert!(!bad_output.exists());
}

#[test]
fn infected_candidates_cannot_change_clean_calibration() {
    let directory = tempfile::tempdir().unwrap();
    let first = directory.path().join("infected");
    let second = directory.path().join("replaced");
    prepare(&first);
    prepare(&second);
    let mut group: Value =
        serde_json::from_slice(&std::fs::read(second.join("input.jsonl")).unwrap()).unwrap();
    let golden = group["samples"][0]["circuit"].clone();
    for sample in group["samples"].as_array_mut().unwrap().iter_mut().skip(1) {
        // Keep labels and cardinality, but replace every infected circuit by the
        // clean circuit. Only the evaluation must change, never calibration.
        sample["circuit"] = golden.clone();
    }
    let input = format!("{group}\n");
    std::fs::write(second.join("input.jsonl"), &input).unwrap();
    let mut preparation: Value =
        serde_json::from_slice(&std::fs::read(second.join("preparation.json")).unwrap()).unwrap();
    preparation["input_sha256"] = json!(artifact_digest(input.as_bytes()));
    std::fs::write(second.join("preparation.json"), preparation.to_string()).unwrap();
    let mut calibrations = Vec::new();
    let mut outcomes = Vec::new();
    for (index, path) in [&first, &second].into_iter().enumerate() {
        let output = directory.path().join(format!("output-{index}"));
        let result = Command::new(env!("CARGO_BIN_EXE_rasqp-external"))
            .arg(path)
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let raw = flate2::read::GzDecoder::new(
            std::fs::File::open(output.join("native_trials.jsonl.gz")).unwrap(),
        );
        let rows: Vec<Value> = BufReader::new(raw)
            .lines()
            .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
            .collect();
        calibrations.push(
            rows.iter()
                .filter(|r| {
                    r["kind"] == "calibration"
                        || r["stage"] == "reference"
                        || r["stage"] == "portfolio_calibration"
                        || r["stage"] == "fixed_calibration"
                })
                .cloned()
                .collect::<Vec<_>>(),
        );
        outcomes.push(
            rows.iter()
                .filter(|r| r["kind"] == "decision" && r["outcome"]["case"] == "infected")
                .map(|r| r["flags"].clone())
                .collect::<Vec<_>>(),
        );
    }
    assert_eq!(calibrations[0], calibrations[1]);
    assert_ne!(outcomes[0], outcomes[1]);
}

#[test]
fn explicitly_versioned_control_study_keeps_benign_labels_and_rejects_empty_groups() {
    let directory = tempfile::tempdir().unwrap();
    let input_dir = directory.path().join("input");
    prepare(&input_dir);
    let mut group: Value =
        serde_json::from_slice(&std::fs::read(input_dir.join("input.jsonl")).unwrap()).unwrap();
    group["samples"].as_array_mut().unwrap().truncate(3);
    group["samples"][1]["label"] = json!(0);
    group["samples"][1]["circuit"] = group["samples"][0]["circuit"].clone();
    let protocol: Value = json!({
        "version":"control-contract", "sample_policy":"mixed_controls", "seed":3456,
        "max_qubits":4, "max_lowered_gates":4080, "noise":[0.0],
        "reference_shots_per_probe":1024, "calibration_repetitions":31,
        "evaluation_repetitions":8, "shots_per_decision":768, "alpha":0.05
    });
    let protocol_bytes = serde_json::to_vec(&protocol).unwrap();
    let protocol_path = directory.path().join("protocol.json");
    std::fs::write(&protocol_path, &protocol_bytes).unwrap();
    for empty in [false, true] {
        if empty {
            group["samples"] = json!([]);
        }
        let input = format!("{group}\n");
        std::fs::write(input_dir.join("input.jsonl"), &input).unwrap();
        std::fs::write(
            input_dir.join("preparation.json"),
            json!({"input_sha256":artifact_digest(input.as_bytes()),
                "protocol_sha256":artifact_digest(&protocol_bytes)})
            .to_string(),
        )
        .unwrap();
        let output = directory.path().join(if empty { "empty" } else { "mixed" });
        let result = Command::new(env!("CARGO_BIN_EXE_rasqp-external"))
            .arg(&input_dir)
            .arg(&output)
            .arg(&protocol_path)
            .output()
            .unwrap();
        if empty {
            assert!(!result.status.success());
            assert!(!output.exists());
        } else {
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            let summary: Value =
                serde_json::from_slice(&std::fs::read(output.join("native_summary.json")).unwrap())
                    .unwrap();
            let outcomes = summary["outcomes"].as_array().unwrap();
            assert_eq!(outcomes.len(), 4);
            assert_eq!(outcomes.iter().filter(|r| r["case"] == "clean").count(), 2);
            assert_eq!(
                outcomes.iter().filter(|r| r["case"] == "infected").count(),
                1
            );
        }
    }
}
