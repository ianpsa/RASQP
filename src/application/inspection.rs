use crate::{
    Circuit, Result,
    domain::EquivalenceOracle,
    research::{
        normalization::Normalizer,
        policy::{Assessment, EvidenceFlags},
    },
};
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct CircuitInspection {
    pub assessment: Assessment,
    pub raw_changed: bool,
    pub normalized_equal: bool,
    pub extra_gates: i64,
    pub rewrite_count: usize,
}
/// Offline circuit inspector with an optional trusted equivalence oracle.
pub struct Inspector<'a> {
    pub oracle: Option<&'a dyn EquivalenceOracle>,
    pub max_extra_gates: Option<usize>,
}
impl Inspector<'_> {
    /// Compare approved and candidate circuits and combine semantic/resource evidence.
    pub fn execute(&self, approved: &Circuit, candidate: &Circuit) -> Result<CircuitInspection> {
        let a = Normalizer::default().run(approved)?;
        let b = Normalizer::default().run(candidate)?;
        let equal = a.circuit == b.circuit;
        let mut known = equal;
        let mut violation = approved.width != candidate.width;
        if !equal
            && !violation
            && let Some(oracle) = self.oracle
        {
            known = oracle.equivalent(approved, candidate)?;
            violation = !known;
        }
        let extra = candidate.gates.len() as i64 - approved.gates.len() as i64;
        let flags = EvidenceFlags {
            known,
            semantic_violation: violation,
            resource_violation: self.max_extra_gates.is_some_and(|limit| {
                candidate.gates.len().saturating_sub(approved.gates.len()) > limit
            }),
            ..Default::default()
        };
        Ok(CircuitInspection {
            assessment: flags.assess(),
            raw_changed: approved.digest()? != candidate.digest()?,
            normalized_equal: equal,
            extra_gates: extra,
            rewrite_count: b.trace.len(),
        })
    }
}
