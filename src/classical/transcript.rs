use crate::{Error, Result, artifact_digest, put_text, put_u64};
use hmac::{Hmac, Mac};
use rusqlite::{Connection, OpenFlags, params};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{path::Path, time::Duration};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub job_id: String,
    pub nonce: String,
    pub circuit_digest: String,
    pub backend: String,
    pub width: u8,
    pub shots: u64,
    /// Digest of the independently approved classical artifact manifest.
    pub artifact_digest: String,
    /// Immutable calibration / probe / policy configuration identifier.
    pub context_digest: String,
}
impl Request {
    /// Validate identifier, digest, width and shot fields before encoding or submission.
    pub fn validate(&self) -> Result<()> {
        if !(1..=16).contains(&self.width) || self.shots == 0 || self.shots > (1_u64 << 53) {
            return Err(Error::Invalid("width or shots"));
        }
        for s in [&self.job_id, &self.nonce, &self.backend] {
            if s.is_empty() || s.len() > 4096 {
                return Err(Error::Invalid("request identifier"));
            }
        }
        for s in [
            &self.circuit_digest,
            &self.artifact_digest,
            &self.context_digest,
        ] {
            if s.len() != 64
                || !s
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err(Error::Invalid("SHA256 digest"));
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Response {
    pub request: Request,
    pub counts: Vec<u64>,
    pub tag: String,
}
/// Validate count shape and exact shot total, then encode the versioned response bytes.
pub fn payload(request: &Request, counts: &[u64]) -> Result<Vec<u8>> {
    request.validate()?;
    if counts.len() != 1_usize << request.width
        || counts.iter().try_fold(0_u64, |n, x| n.checked_add(*x)) != Some(request.shots)
    {
        return Err(Error::Invalid("counts shape or sum"));
    }
    let mut out = b"RASQP-RESPONSE\0\x01".to_vec();
    for s in [
        &request.job_id,
        &request.nonce,
        &request.circuit_digest,
        &request.backend,
        &request.artifact_digest,
        &request.context_digest,
    ] {
        put_text(&mut out, s);
    }
    out.push(request.width);
    put_u64(&mut out, request.shots);
    put_u64(&mut out, counts.len() as u64);
    for x in counts {
        put_u64(&mut out, *x);
    }
    Ok(out)
}
fn mac(key: &[u8]) -> Result<Hmac<Sha256>> {
    if key.len() < 32 {
        return Err(Error::Invalid("key must contain at least 32 bytes"));
    }
    Hmac::<Sha256>::new_from_slice(key).map_err(|_| Error::Invalid("HMAC key"))
}
/// Provider-side laboratory transport surrogate, not an IBM signature API.
/// Sign a response for a trusted local fixture or a service that really owns the key.
/// This does not authenticate a provider that does not issue the MAC itself.
pub fn sign(request: Request, counts: Vec<u64>, key: &[u8]) -> Result<Response> {
    let mut m = mac(key)?;
    m.update(&payload(&request, &counts)?);
    Ok(Response {
        request,
        counts,
        tag: hex::encode(m.finalize().into_bytes()),
    })
}
/// Implementations MUST atomically persist a first-use claim before returning true.
pub trait NonceStore {
    /// Atomically record a nonce; return false if it was already claimed.
    fn claim(&mut self, nonce: &str) -> Result<bool>;
}
/// Deterministic lab baseline; intentionally loses replay history on recreation.
#[derive(Default)]
pub struct MemoryLedger {
    used: std::collections::BTreeSet<String>,
}
impl NonceStore for MemoryLedger {
    fn claim(&mut self, nonce: &str) -> Result<bool> {
        Ok(self.used.insert(nonce.to_owned()))
    }
}
pub struct TranscriptVerifier<S: NonceStore> {
    key: Vec<u8>,
    store: S,
}
impl<S: NonceStore> TranscriptVerifier<S> {
    /// Initialize HMAC verification with an independently protected key and ledger.
    pub fn new(key: Vec<u8>, store: S) -> Result<Self> {
        mac(&key)?;
        Ok(Self { key, store })
    }
    /// Authenticate and replay-check a signed response for the expected request.
    pub fn verify(&mut self, expected: &Request, response: &Response) -> Result<bool> {
        verify(expected, response, &self.key, &mut self.store)
    }
}
pub struct SqliteLedger {
    connection: Connection,
}
impl SqliteLedger {
    /// Open a durable local SQLite nonce ledger for cross-process replay detection.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        // Disable SQLite URI interpretation: a file:...mode=memory URI must never
        // silently turn a caller's persistent ledger into an ephemeral one.
        let connection = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        connection.busy_timeout(Duration::from_secs(10))?;
        connection.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; CREATE TABLE IF NOT EXISTS nonces (digest TEXT PRIMARY KEY NOT NULL);")?;
        Ok(Self { connection })
    }
}
impl NonceStore for SqliteLedger {
    fn claim(&mut self, nonce: &str) -> Result<bool> {
        Ok(self.connection.execute(
            "INSERT INTO nonces (digest) VALUES (?1) ON CONFLICT(digest) DO NOTHING",
            params![artifact_digest(nonce.as_bytes())],
        )? == 1)
    }
}
/// Verify the MAC and request binding, then claim the nonce on an authentic response.
pub fn verify(
    expected: &Request,
    response: &Response,
    key: &[u8],
    ledger: &mut impl NonceStore,
) -> Result<bool> {
    if expected != &response.request {
        return Ok(false);
    }
    let mut m = mac(key)?;
    m.update(&payload(expected, &response.counts)?);
    let Ok(tag) = hex::decode(&response.tag) else {
        return Ok(false);
    };
    if m.verify_slice(&tag).is_err() {
        return Ok(false);
    }
    ledger.claim(&expected.nonce)
}
