use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{io::Read, path::Path};
/// Hashes the bytes read from a file handle. Does not attest loaded memory or
/// eliminate check/use races; use an independently trusted loader/supervisor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactFingerprint {
    pub sha256: String,
    pub bytes: u64,
}
impl ArtifactFingerprint {
    /// Measure a regular file's SHA-256 digest and byte length for a trusted baseline.
    /// This does not attest the code already loaded into a process.
    pub fn measure(path: impl AsRef<Path>) -> Result<Self> {
        let mut file = std::fs::File::open(path)?;
        if !file.metadata()?.is_file() {
            return Err(Error::Invalid("artifact is not a regular file"));
        }
        let mut hasher = Sha256::new();
        let mut bytes = 0;
        let mut buffer = [0_u8; 65536];
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
            bytes += n as u64;
        }
        Ok(Self {
            sha256: hex::encode(hasher.finalize()),
            bytes,
        })
    }
    /// Compare two previously measured file fingerprints exactly.
    pub fn matches(&self, trusted: &Self) -> bool {
        self == trusted
    }
}
