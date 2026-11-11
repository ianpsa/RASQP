pub mod artifact;
pub mod hooks;
pub mod transcript;
use crate::artifact_digest;
pub fn artifact_matches(bytes: &[u8], trusted_sha256: &str) -> bool {
    artifact_digest(bytes) == trusted_sha256
}
