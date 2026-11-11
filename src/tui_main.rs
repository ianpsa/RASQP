//! Accessible native terminal notebook; the JSON CLI stays in `rasqp`.
#![forbid(unsafe_code)]
mod tui;
use std::{io, path::PathBuf};

fn main() {
    if let Err(error) = run() {
        eprintln!("rasqp-tui: {error}");
        std::process::exit(2);
    }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|a| a == "--lab-child") {
        if args.len() < 3 || args.len() > 4 || (args.len() == 4 && args[3] != "--unconfined") {
            return Err("internal laboratory usage: --lab-child MODE OUTPUT [--unconfined]".into());
        }
        let mode = &args[1];
        if ![
            "all",
            "batch",
            "hypotheses",
            "trojans",
            "extensions",
            "context",
        ]
        .contains(&mode.as_str())
        {
            return Err("unknown native study".into());
        }
        let lab = rasqp::lab::experiment::ResearchLab::open(std::path::Path::new(&args[2]))?;
        let sandboxed = args.len() != 4;
        if sandboxed {
            rasqp::lab::sandbox::restrict()?;
        }
        let summary = lab.run(mode, sandboxed)?;
        println!("{}", summary["cost"]);
        return Ok(());
    }
    let mut summary = PathBuf::from("results/native-v1/native_summary.json");
    let mut snapshot = None;
    let mut width = 120;
    let mut height = 42;
    let mut demo = 0;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--help" | "-h" => {
                println!(
                    "RASQP terminal lab notebook\n\nUsage: rasqp-tui [--summary PATH]\n       rasqp-tui --snapshot learn|workbench|studies|sdk|glossary [--width N --height N] [--demo 0..8]\n\nMouse: click tabs, topics and buttons; wheel scrolls either panel; drag the scrollbar.\nInteractive keys: Tab / 1-5 pages, arrows topics, PgUp/PgDn scroll, / search, ? help, q quit.\nWorkbench: Enter run, b probe, n noise, s shots, r new seed, v JSON, e save.\nStudies: Enter run, x cancel, u sandbox, v JSON, r reload.\n\nThe native studies use local simulation; Linux seccomp is enabled by default.\nEach run writes a unique directory in results/runs/tui/. Quitting cancels active studies.\nSnapshots use a headless Ratatui backend and print plain text without changing terminal state."
                );
                return Ok(());
            }
            "--version" => {
                println!("rasqp-tui {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            option @ ("--summary" | "--snapshot" | "--width" | "--height" | "--demo") => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| format!("{option} needs a value"))?;
                match option {
                    "--summary" => summary = value.into(),
                    "--snapshot" => {
                        snapshot = Some(match value.as_str() {
                            "learn" => 0,
                            "workbench" => 1,
                            "studies" => 2,
                            "sdk" => 3,
                            "glossary" => 4,
                            _ => return Err(
                                "snapshot page must be learn, workbench, studies, sdk or glossary"
                                    .into(),
                            ),
                        })
                    }
                    "--width" => width = value.parse::<u16>()?,
                    "--height" => height = value.parse::<u16>()?,
                    "--demo" => {
                        demo = value.parse::<usize>()?;
                        if demo > 8 {
                            return Err("demo must be 0..8".into());
                        }
                    }
                    _ => unreachable!(),
                }
                index += 1;
            }
            unknown => return Err(format!("Unknown option {unknown}. Use --help.").into()),
        }
        index += 1;
    }
    if !(1..=300).contains(&width) || !(1..=200).contains(&height) {
        return Err("snapshot dimensions must be 1..300 columns and 1..200 rows".into());
    }
    if let Some(tab) = snapshot {
        use io::Write;
        io::stdout()
            .lock()
            .write_all(tui::snapshot(summary, tab, demo, width, height)?.as_bytes())?;
    } else {
        tui::run(summary)?;
    }
    Ok(())
}
