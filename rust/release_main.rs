//! Native replacement for scripts/package_release.py; no Python required.
use rasqp::{Error, Result, artifact_digest};
use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};
fn run() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let version = env!("CARGO_PKG_VERSION");
    let tag = std::env::var("RELEASE_TAG").unwrap_or_else(|_| format!("v{version}"));
    if tag != format!("v{version}") {
        return Err(Error::Invalid("release tag must match Cargo version"));
    }
    let platform = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    if std::env::var("RELEASE_PLATFORM").is_ok_and(|p| p != platform) {
        return Err(Error::Invalid("runner platform differs from release label"));
    }
    let suffix = std::env::consts::EXE_SUFFIX;
    let binary = root.join(format!("target/release/rasqp{suffix}"));
    let output = Command::new(&binary).arg("--version").output()?;
    if !output.status.success()
        || serde_json::from_slice::<serde_json::Value>(&output.stdout)?["version"] != version
    {
        return Err(Error::Invalid("compiled binary version"));
    }
    let crate_file = root.join(format!("target/package/rasqp-{version}.crate"));
    let docs = [
        "README.txt",
        "docs/RUST_SDK.txt",
        "docs/RESEARCH_V03.txt",
        "docs/MIGRATION_MATRIX.txt",
    ];
    let mut inputs: Vec<(PathBuf, String)> = vec![
        (binary, format!("rasqp{suffix}")),
        (
            root.join(format!("target/release/rasqp-lab{suffix}")),
            format!("rasqp-lab{suffix}"),
        ),
        (crate_file, format!("rasqp-{version}.crate")),
    ];
    inputs.extend(docs.iter().map(|p| (root.join(p), p.to_string())));
    for (path, _) in &inputs {
        if !path.is_file() {
            return Err(Error::Operation(format!(
                "missing release input {}",
                path.display()
            )));
        }
    }
    let out = root.join("dist");
    std::fs::create_dir_all(&out)?;
    let filename = format!("rasqp-{tag}-{platform}.tar.gz");
    let archive = out.join(&filename);
    let gzip =
        flate2::write::GzEncoder::new(File::create(&archive)?, flate2::Compression::default());
    let mut tar = tar::Builder::new(gzip);
    for (path, name) in inputs {
        tar.append_path_with_name(path, name)?;
    }
    let metadata = serde_json::to_vec_pretty(
        &serde_json::json!({"version":version,"platform":platform,"commit":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local-unpublished".into()),"cargo_lock_sha256":artifact_digest(&std::fs::read(root.join("Cargo.lock"))?),"protocol_sha256":artifact_digest(include_bytes!("../docs/NATIVE_PROTOCOL_V1.json"))}),
    )?;
    let mut header = tar::Header::new_gnu();
    header.set_size(metadata.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    tar.append_data(&mut header, "build.json", metadata.as_slice())?;
    tar.into_inner()?.finish()?;
    std::fs::write(
        out.join(format!("{filename}.sha256")),
        format!(
            "{}  {filename}\n",
            artifact_digest(&std::fs::read(&archive)?)
        ),
    )?;
    println!("{}", archive.display());
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(2);
    }
}
