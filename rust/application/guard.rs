use crate::{
    Circuit, Request, Response, Result, inspect,
    transcript::{NonceStore, verify},
    verification::CalibratedVerifier,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    Accept,
    Reject,
    Inconclusive,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub artifact_digest: String,
    pub context_digest: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub max_extra_gates: usize,
    pub context: Context,
    pub distribution: CalibratedVerifier,
}
/// Stateful application facade; references and policy are fixed by the trusted caller.
pub struct Supervisor<S: NonceStore> {
    policy: Policy,
    key: Vec<u8>,
    ledger: S,
}
impl<S: NonceStore> Supervisor<S> {
    /// Create a result verifier with a trusted policy, signing key and replay ledger.
    /// The HMAC key is meaningful only when a separate trusted signer exists.
    pub fn new(policy: Policy, key: Vec<u8>, ledger: S) -> Result<Self> {
        policy.distribution.validate()?;
        if key.len() < 32 {
            return Err(crate::Error::Invalid("key must contain at least 32 bytes"));
        }
        Ok(Self {
            policy,
            key,
            ledger,
        })
    }
    /// Verify one already obtained signed response before exposing its counts.
    pub fn execute(
        &mut self,
        approved: &Circuit,
        submitted: &Circuit,
        request: &Request,
        response: &Response,
    ) -> Result<Outcome> {
        protect(
            approved,
            submitted,
            request,
            response,
            &self.policy,
            &self.key,
            &mut self.ledger,
        )
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Outcome {
    pub decision: Decision,
    pub reason: String,
    pub accepted_counts: Option<Vec<u64>>,
}
impl Outcome {
    fn stop(decision: Decision, reason: &str) -> Self {
        Self {
            decision,
            reason: reason.into(),
            accepted_counts: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchEntry {
    pub approved: Circuit,
    pub submitted: Circuit,
    pub request: Request,
    pub response: Response,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchPolicy {
    pub context: Context,
    pub max_extra_gates: usize,
    pub verifier: crate::verification::JointVerifier,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOutcome {
    pub decision: Decision,
    pub reason: String,
    pub accepted_counts: Option<Vec<Vec<u64>>>,
}
/// Verify an ordered, already obtained signed batch. Release counts only after every
/// transcript, circuit, context and the
/// joint behavioral check succeeds. A failed batch may consume some nonces; retry
/// requires fresh trusted requests. This prevents partial result release.
pub fn protect_batch(
    entries: &[BatchEntry],
    policy: &BatchPolicy,
    key: &[u8],
    ledger: &mut impl NonceStore,
) -> Result<BatchOutcome> {
    policy.verifier.validate()?;
    if entries.len() != policy.verifier.references.len() {
        return Err(crate::Error::Invalid("batch size"));
    }
    let mut counts = Vec::new();
    for (entry, reference) in entries.iter().zip(&policy.verifier.references) {
        let individual = Policy {
            context: policy.context.clone(),
            max_extra_gates: policy.max_extra_gates,
            distribution: CalibratedVerifier {
                reference: reference.clone(),
                threshold: 1.0,
            },
        };
        let out = protect(
            &entry.approved,
            &entry.submitted,
            &entry.request,
            &entry.response,
            &individual,
            key,
            ledger,
        )?;
        if out.decision != Decision::Accept {
            return Ok(BatchOutcome {
                decision: out.decision,
                reason: out.reason,
                accepted_counts: None,
            });
        }
        counts.push(
            out.accepted_counts
                .ok_or(crate::Error::Invalid("accepted response missing counts"))?,
        );
    }
    if policy.verifier.rejects(&counts)? {
        return Ok(BatchOutcome {
            decision: Decision::Reject,
            reason: "joint probe deviation".into(),
            accepted_counts: None,
        });
    }
    Ok(BatchOutcome {
        decision: Decision::Accept,
        reason: "batch within evaluated scope".into(),
        accepted_counts: Some(counts),
    })
}
/// Verify one already obtained signed response against a trusted request, circuit,
/// artifact/context identifiers and calibrated measurement distribution.
/// All policy/reference inputs are supplied by the trusted supervisor, never the provider.
/// A consumed authentic response stays consumed even when later checks reject it.
pub fn protect(
    approved: &Circuit,
    submitted: &Circuit,
    request: &Request,
    response: &Response,
    policy: &Policy,
    key: &[u8],
    ledger: &mut impl NonceStore,
) -> Result<Outcome> {
    policy.distribution.validate()?;
    if request.width != submitted.width || request.circuit_digest != submitted.digest()? {
        return Ok(Outcome::stop(Decision::Reject, "circuit binding mismatch"));
    }
    if !verify(request, response, key, ledger)? {
        return Ok(Outcome::stop(
            Decision::Reject,
            "invalid or replayed transcript",
        ));
    }
    if request.artifact_digest != policy.context.artifact_digest {
        return Ok(Outcome::stop(
            Decision::Reject,
            "classical artifact manifest mismatch",
        ));
    }
    if request.context_digest != policy.context.context_digest {
        return Ok(Outcome::stop(
            Decision::Inconclusive,
            "calibration context changed",
        ));
    }
    let inspection = inspect(approved, submitted, policy.max_extra_gates)?;
    if inspection.decision != Decision::Accept {
        return Ok(Outcome::stop(
            inspection.decision,
            "circuit or resource evidence",
        ));
    }
    if policy.distribution.rejects(&response.counts)? {
        return Ok(Outcome::stop(
            Decision::Reject,
            "distribution outside calibrated tolerance",
        ));
    }
    Ok(Outcome {
        decision: Decision::Accept,
        reason: "within evaluated scope".into(),
        accepted_counts: Some(response.counts.clone()),
    })
}
