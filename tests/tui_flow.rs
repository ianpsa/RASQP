//! Exercise the standalone binary, including its isolated native-lab child path.
use std::process::Command;

fn tui() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rasqp-tui"))
}
#[test]
fn snapshots_use_the_actual_renderer_without_an_interactive_terminal() {
    let output = tui()
        .args([
            "--snapshot",
            "workbench",
            "--demo",
            "2",
            "--width",
            "100",
            "--height",
            "36",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("Phase flip: a blind spot"));
    assert!(text.contains("no behavioral alarm"));
    assert!(text.contains("Inconclusive"));
    assert!(!text.contains('\u{1b}'));
    let invalid = tui().args(["--snapshot", "unknown"]).output().unwrap();
    assert_eq!(invalid.status.code(), Some(2));
    let invalid = tui()
        .args(["--snapshot", "learn", "--width", "0"])
        .output()
        .unwrap();
    assert_eq!(invalid.status.code(), Some(2));
}
#[test]
fn missing_summary_has_an_actionable_empty_state() {
    let d = tempfile::tempdir().unwrap();
    let output = tui()
        .args(["--snapshot", "studies", "--summary"])
        .arg(d.path().join("missing.json"))
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("No recorded native evidence loaded")
    );
}
#[test]
fn standalone_lab_child_writes_reusable_native_evidence() {
    let d = tempfile::tempdir().unwrap();
    let mut command = tui();
    command.arg("--lab-child").arg("context").arg(d.path());
    // Explicitly opt into the existing lab's unconfined mode where seccomp is unavailable.
    if !cfg!(target_os = "linux") {
        command.arg("--unconfined");
    }
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let path = d.path().join("native_summary.json");
    let summary: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(summary["sandboxed"], cfg!(target_os = "linux"));
    assert_eq!(summary["qpu_shots"], 0);
    assert!(summary["studies"]["context"]["H44_pass"].is_boolean());
    assert!(summary["cost"]["shots"].as_u64().unwrap() > 0);
    for file in [
        "native_protocol.json",
        "native_trials.jsonl",
        "native_report.txt",
    ] {
        assert!(d.path().join(file).metadata().unwrap().len() > 0);
    }
    let output = tui()
        .args(["--snapshot", "studies", "--summary"])
        .arg(path)
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("H44: criterion"));
    assert!(text.contains("Included: context"));
}
