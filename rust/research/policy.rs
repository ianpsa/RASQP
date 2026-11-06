use crate::Decision;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EvidenceFlags {
    pub known: bool,
    pub semantic_violation: bool,
    pub behavioral_violation: bool,
    pub resource_violation: bool,
    pub drift: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Assessment {
    pub decision: Decision,
    pub reason: String,
}
impl EvidenceFlags {
    pub fn assess(self) -> Assessment {
        let (decision, reason) =
            if self.semantic_violation || self.behavioral_violation || self.resource_violation {
                (
                    Decision::Reject,
                    "specified property violated; intent undetermined",
                )
            } else if self.drift || !self.known {
                (
                    Decision::Inconclusive,
                    "unknown variant or calibration context changed",
                )
            } else {
                (Decision::Accept, "within explicitly evaluated scope")
            };
        Assessment {
            decision,
            reason: reason.into(),
        }
    }
}
impl Assessment {
    pub fn release(&self, counts: Vec<u64>) -> Option<Vec<u64>> {
        (self.decision == Decision::Accept).then_some(counts)
    }
}
