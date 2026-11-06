fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
fn run() -> rasqp::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: rasqp-lab all|batch|hypotheses|trojans|extensions|context OUTPUT_DIRECTORY [--unconfined]\nLinux seccomp is enabled by default. Output files are opened before filtering."
        );
        return Ok(());
    }
    let lab = rasqp::lab::experiment::ResearchLab::open(std::path::Path::new(&args[2]))?;
    let sandboxed = !args.iter().any(|a| a == "--unconfined");
    if sandboxed {
        rasqp::lab::sandbox::restrict()?;
    }
    let summary = lab.run(&args[1], sandboxed)?;
    println!("{}", summary["cost"]);
    Ok(())
}
