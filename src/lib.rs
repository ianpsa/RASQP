//! Experimental runtime assurance. Trusted inputs, keys and ledger must be protected
//! independently from the workload. No protection against a compromised verifier host.
#![forbid(unsafe_code)]
pub mod adapters;
pub mod application;
pub mod classical;
pub mod domain;
pub mod lab;
pub mod research;
pub use application::guard;
pub use circuit::{Circuit, Gate, Inspection, inspect};
pub use classical::transcript;
pub use domain::circuit;
pub use guard::{Context, Decision, Outcome, Policy, protect};
pub use research::verification;
pub use transcript::{Request, Response, SqliteLedger, sign};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid input: {0}")]
    Invalid(&'static str),
    #[error("ledger failure: {0}")]
    Ledger(#[from] rusqlite::Error),
    #[error("I/O failure: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON failure: {0}")]
    Json(#[from] serde_json::Error),
    #[error("operation failed: {0}")]
    Operation(String),
}
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn put_u64(out: &mut Vec<u8>, n: u64) {
    out.extend(n.to_le_bytes());
}
pub(crate) fn put_text(out: &mut Vec<u8>, s: &str) {
    put_u64(out, s.len() as u64);
    out.extend(s.as_bytes());
}
pub fn artifact_digest(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(bytes))
}
