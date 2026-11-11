use serde_json::Value;

pub fn save_demo(raw: &str) -> io::Result<PathBuf> {
    use io::Write;
    let root = Path::new("results/runs/tui/demos");
    fs::create_dir_all(root)?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)?
        .as_nanos();
    let path = root.join(format!("{stamp}-{}.json", std::process::id()));
    File::create_new(&path)?.write_all(raw.as_bytes())?;
    Ok(path)
}
use std::{
    fs::{self, File},
    io::{self, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

pub struct Evidence {
    pub path: PathBuf,
    pub value: Value,
    pub raw: std::collections::BTreeMap<String, String>,
}
impl Evidence {
    pub fn load(path: &Path) -> Result<Self, String> {
        let mut bytes = Vec::new();
        File::open(path)
            .map_err(|e| format!("{}: {e}", path.display()))?
            .take(32 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        if bytes.len() > 32 * 1024 * 1024 {
            return Err("Summary exceeds 32 MiB".into());
        }
        let value: Value =
            serde_json::from_slice(&bytes).map_err(|e| format!("Invalid summary JSON: {e}"))?;
        let studies = value["studies"]
            .as_object()
            .ok_or("Expected a native summary with a studies object")?;
        if !value["cost"].is_object() || value["protocol_sha256"].as_str().is_none() {
            return Err("Expected native cost and protocol_sha256 fields".into());
        }
        let mut raw = std::collections::BTreeMap::new();
        raw.insert(
            "all".into(),
            serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?,
        );
        for (name, result) in studies {
            raw.insert(
                name.clone(),
                serde_json::to_string_pretty(result).map_err(|e| e.to_string())?,
            );
        }
        Ok(Self {
            path: path.to_owned(),
            value,
            raw,
        })
    }
}

/// The UI spawns itself in lab-only mode, so a standalone TUI can run studies
/// without Cargo or another installed binary. The child applies the existing sandbox.
pub struct LabRun {
    child: Child,
    pub directory: PathBuf,
    pub mode: String,
    pub started: Instant,
    pub unconfined: bool,
    reaped: bool,
}
impl LabRun {
    pub fn start(root: &Path, mode: &str, unconfined: bool) -> io::Result<Self> {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_nanos();
        fs::create_dir_all(root)?;
        let directory = root.join(format!("{stamp}-{}", std::process::id()));
        fs::create_dir(&directory)?;
        let log = File::create(directory.join("runner.log"))?;
        let mut command = Command::new(std::env::current_exe()?);
        command
            .arg("--lab-child")
            .arg(mode)
            .arg(&directory)
            .stdin(Stdio::null())
            .stdout(log.try_clone()?)
            .stderr(log);
        if unconfined {
            command.arg("--unconfined");
        }
        let child = command.spawn()?;
        Ok(Self {
            child,
            directory,
            mode: mode.into(),
            started: Instant::now(),
            unconfined,
            reaped: false,
        })
    }
    pub fn poll(&mut self) -> io::Result<Option<bool>> {
        let status = self.child.try_wait()?;
        if status.is_some() {
            self.reaped = true;
        }
        Ok(status.map(|s| s.success()))
    }
    pub fn log_tail(&self) -> String {
        tail(&self.directory.join("runner.log")).unwrap_or_else(|e| format!("Log unavailable: {e}"))
    }
    pub fn cancel(&mut self) -> io::Result<()> {
        if !self.reaped {
            // Avoid a kill error when the child completed between event polls.
            if self.child.try_wait()?.is_none() {
                self.child.kill()?;
            }
            self.child.wait()?;
            self.reaped = true;
        }
        Ok(())
    }
}
impl Drop for LabRun {
    fn drop(&mut self) {
        let _ = self.cancel();
    }
}
fn tail(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let len = file.metadata()?.len();
    file.seek(SeekFrom::Start(len.saturating_sub(8192)))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes)
        .lines()
        .rev()
        .take(8)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_summaries_fail_without_replacing_evidence() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("summary.json");
        fs::write(&p, "{\"studies\":[]}").unwrap();
        assert!(Evidence::load(&p).is_err());
        fs::write(&p, "not JSON").unwrap();
        assert!(Evidence::load(&p).is_err());
        assert!(Evidence::load(&d.path().join("missing")).is_err());
        fs::write(
            &p,
            r#"{"studies":{"context":{"H44_pass":false}},"cost":{},"protocol_sha256":"fixture"}"#,
        )
        .unwrap();
        let evidence = Evidence::load(&p).unwrap();
        assert_eq!(evidence.value["studies"]["context"]["H44_pass"], false);
        assert!(evidence.raw["context"].contains("false"));
    }
}
