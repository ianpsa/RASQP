//! Application-facing runtime guard. All result consumption flows through this service.
//! A backend adapter owns network transport and its authentication; the guard evaluates
//! the returned measurements but cannot prove that an untrusted provider executed them.
use crate::{
    Circuit, Decision, Error, Request, Result, artifact_digest,
    classical::{
        artifact::ArtifactFingerprint,
        transcript::{NonceStore, payload},
    },
    inspect,
    verification::{CalibratedVerifier, JointVerifier},
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Adapter for a simulator or quantum provider. The caller must pass the exact circuit
/// it submits. Transport authentication belongs to the adapter, not to a local HMAC.
pub trait QuantumBackend {
    /// Stable identifier for the provider, device, and calibration context.
    fn backend_id(&self) -> &str;
    /// Submit the exact circuit and return canonical computational-basis counts.
    /// A remote adapter must independently authenticate its transport and job mapping.
    fn execute(&mut self, request: &Request, circuit: &Circuit) -> Result<Vec<u64>>;
}

/// Optional runtime challenge circuits and their jointly calibrated distribution check.
/// Challenges are separate jobs; they cannot prove that the workload job was honest.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbePlan {
    /// Ordered challenge circuits built from independently approved probe choices.
    pub circuits: Vec<Circuit>,
    /// Number of measurements requested for each challenge.
    pub shots: u64,
    /// References and maximum-TVD threshold calibrated for this exact portfolio.
    pub verifier: JointVerifier,
}

/// Trusted configuration for one guarded workload and its optional challenges.
/// Baselines, artifact fingerprint, and backend identity must be approved outside
/// the potentially hostile quantum provider.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimePolicy {
    /// Trusted workload circuit expected at the execution boundary.
    pub approved: Circuit,
    /// File remeasured around execution; the trusted host controls this path.
    pub artifact_path: PathBuf,
    /// Independently approved fingerprint of that file.
    pub artifact: ArtifactFingerprint,
    /// Stable identity expected from the backend adapter.
    pub backend_id: String,
    /// Requested workload measurements.
    pub shots: u64,
    /// Maximum allowed raw gate increase before rejecting a candidate.
    pub max_extra_gates: usize,
    /// Workload reference and calibrated TVD threshold.
    pub distribution: CalibratedVerifier,
    /// Optional challenge jobs; omitted means no probe check is performed.
    pub probes: Option<ProbePlan>,
}
impl RuntimePolicy {
    /// Check that circuit, distributions, shot counts, and probe shapes are coherent.
    pub fn validate(&self) -> Result<()> {
        self.approved.validate()?;
        self.distribution.validate()?;
        if self.backend_id.is_empty()
            || self.shots == 0
            || self.shots > 1_u64 << 53
            || self.distribution.reference.len() != 1_usize << self.approved.width
            || self.artifact.sha256.len() != 64
            || !self
                .artifact
                .sha256
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(Error::Invalid("runtime policy"));
        }
        if let Some(probes) = &self.probes {
            probes.verifier.validate()?;
            probes
                .shots
                .checked_mul(probes.circuits.len() as u64)
                .and_then(|n| self.shots.checked_add(n))
                .ok_or(Error::Invalid("shot budget overflow"))?;
            if probes.circuits.len() != probes.verifier.references.len()
                || probes.shots == 0
                || probes.shots > 1_u64 << 53
                || probes
                    .circuits
                    .iter()
                    .zip(&probes.verifier.references)
                    .any(|(c, r)| {
                        c.validate().is_err()
                            || c.width != self.approved.width
                            || r.len() != 1_usize << c.width
                    })
            {
                return Err(Error::Invalid("probe plan"));
            }
        }
        Ok(())
    }
    /// Local, versioned commitment to every policy field, including reference data.
    /// Applications must approve this policy independently of the provider.
    pub fn context_digest(&self) -> Result<String> {
        self.validate()?;
        let mut bytes = b"RASQP-RUNTIME-POLICY\0\x01".to_vec();
        bytes.extend(serde_json::to_vec(self)?);
        Ok(artifact_digest(&bytes))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeOutcome {
    /// Accept releases counts; Reject or Inconclusive withholds them.
    pub decision: Decision,
    pub reason: String,
    /// Workload measurements available only after all enabled checks pass.
    pub accepted_counts: Option<Vec<u64>>,
    /// Workload total-variation distance from the trusted reference.
    pub workload_tvd: Option<f64>,
    /// Largest challenge distance from its reference, when probes ran.
    pub probe_max_tvd: Option<f64>,
    /// Requested shots across workload and challenges already submitted.
    pub shots_submitted: u64,
}
impl RuntimeOutcome {
    fn stop(
        decision: Decision,
        reason: &str,
        shots_submitted: u64,
        workload_tvd: Option<f64>,
        probe_max_tvd: Option<f64>,
    ) -> Self {
        Self {
            decision,
            reason: reason.into(),
            accepted_counts: None,
            workload_tvd,
            probe_max_tvd,
            shots_submitted,
        }
    }
}

/// Stateful RASP entry point. The trusted application owns this instance and consumes
/// only `accepted_counts`. Policies and approved circuits never come from backend output.
pub struct GuardedRuntime<B: QuantumBackend, S: NonceStore> {
    backend: B,
    ledger: S,
    policy: RuntimePolicy,
    context_digest: String,
}
impl<B: QuantumBackend, S: NonceStore> GuardedRuntime<B, S> {
    /// Bind a backend and replay ledger to a validated, independently trusted policy.
    pub fn new(policy: RuntimePolicy, backend: B, ledger: S) -> Result<Self> {
        let context_digest = policy.context_digest()?;
        if policy.backend_id != backend.backend_id() {
            return Err(Error::Invalid("backend context"));
        }
        Ok(Self {
            backend,
            ledger,
            policy,
            context_digest,
        })
    }
    /// Inspect the immutable policy currently enforced by this runtime.
    pub fn policy(&self) -> &RuntimePolicy {
        &self.policy
    }
    /// Access the backend adapter for provider-specific maintenance between runs.
    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }
    fn request(&self, circuit: &Circuit, shots: u64) -> Result<Request> {
        let mut nonce = [0_u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut nonce);
        let nonce = hex::encode(nonce);
        let request = Request {
            job_id: format!("rasqp-{nonce}"),
            nonce,
            circuit_digest: circuit.digest()?,
            backend: self.policy.backend_id.clone(),
            width: circuit.width,
            shots,
            artifact_digest: self.policy.artifact.sha256.clone(),
            context_digest: self.context_digest.clone(),
        };
        request.validate()?;
        Ok(request)
    }
    fn artifact_matches(&self) -> Result<bool> {
        Ok(ArtifactFingerprint::measure(&self.policy.artifact_path)? == self.policy.artifact)
    }
    /// Executes one workload plus optional probes. A backend error returns `Err` without
    /// releasing counts; a measured violation returns Reject or Inconclusive.
    pub fn execute(&mut self, candidate: &Circuit) -> Result<RuntimeOutcome> {
        let inspection = inspect(
            &self.policy.approved,
            candidate,
            self.policy.max_extra_gates,
        )?;
        if inspection.decision != Decision::Accept {
            return Ok(RuntimeOutcome::stop(
                inspection.decision,
                "circuit or resource evidence",
                0,
                None,
                None,
            ));
        }
        if !self.artifact_matches()? {
            return Ok(RuntimeOutcome::stop(
                Decision::Reject,
                "classical artifact changed",
                0,
                None,
                None,
            ));
        }
        if self.backend.backend_id() != self.policy.backend_id {
            return Ok(RuntimeOutcome::stop(
                Decision::Inconclusive,
                "backend context changed",
                0,
                None,
                None,
            ));
        }
        let request = self.request(candidate, self.policy.shots)?;
        let counts = self.backend.execute(&request, candidate)?;
        let mut spent = self.policy.shots;
        payload(&request, &counts)?; // Shape, nonnegative counts, and exact shot sum.
        if !self.ledger.claim(&request.nonce)? {
            return Ok(RuntimeOutcome::stop(
                Decision::Reject,
                "duplicate local request",
                spent,
                None,
                None,
            ));
        }
        let score = self.policy.distribution.score(&counts)?;
        if self.policy.distribution.rejects(&counts)? {
            return Ok(RuntimeOutcome::stop(
                Decision::Reject,
                "workload distribution outside tolerance",
                spent,
                Some(score),
                None,
            ));
        }
        let mut probe_score = None;
        if let Some(plan) = self.policy.probes.clone() {
            let mut observations = Vec::with_capacity(plan.circuits.len());
            for probe in &plan.circuits {
                let request = self.request(probe, plan.shots)?;
                let counts = self.backend.execute(&request, probe)?;
                spent = spent
                    .checked_add(plan.shots)
                    .ok_or(Error::Invalid("shot budget overflow"))?;
                payload(&request, &counts)?;
                if !self.ledger.claim(&request.nonce)? {
                    return Ok(RuntimeOutcome::stop(
                        Decision::Reject,
                        "duplicate local probe request",
                        spent,
                        Some(score),
                        probe_score,
                    ));
                }
                observations.push(counts);
            }
            let value = plan.verifier.score(&observations)?;
            probe_score = Some(value);
            if plan.verifier.rejects(&observations)? {
                return Ok(RuntimeOutcome::stop(
                    Decision::Reject,
                    "probe distribution outside tolerance",
                    spent,
                    Some(score),
                    probe_score,
                ));
            }
        }
        if !self.artifact_matches()? {
            return Ok(RuntimeOutcome::stop(
                Decision::Reject,
                "classical artifact changed during execution",
                spent,
                Some(score),
                probe_score,
            ));
        }
        Ok(RuntimeOutcome {
            decision: Decision::Accept,
            reason: "within evaluated runtime policy".into(),
            accepted_counts: Some(counts),
            workload_tvd: Some(score),
            probe_max_tvd: probe_score,
            shots_submitted: spent,
        })
    }
}
