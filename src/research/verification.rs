use crate::{Error, Result};
use serde::{Deserialize, Serialize};

/// Maximum TVD across a fixed, ordered probe portfolio. Calibration batches must
/// use the same probes, raw approved variants, backend context and shot allocation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JointVerifier {
    pub references: Vec<Vec<f64>>,
    pub threshold: f64,
}
impl JointVerifier {
    /// Validate all reference distributions and the common TVD threshold.
    pub fn validate(&self) -> Result<()> {
        if self.references.is_empty() || self.references.len() > 64 {
            return Err(Error::Invalid("probe count"));
        }
        for reference in &self.references {
            CalibratedVerifier {
                reference: reference.clone(),
                threshold: self.threshold,
            }
            .validate()?;
        }
        Ok(())
    }
    /// Return the largest total-variation distance across the ordered probe batch.
    pub fn score(&self, counts: &[Vec<u64>]) -> Result<f64> {
        self.validate()?;
        if counts.len() != self.references.len() {
            return Err(Error::Invalid("probe batch shape"));
        }
        let mut maximum = 0.0_f64;
        for (reference, counts) in self.references.iter().zip(counts) {
            maximum = maximum.max(
                CalibratedVerifier {
                    reference: reference.clone(),
                    threshold: self.threshold,
                }
                .score(counts)?,
            );
        }
        Ok(maximum)
    }
    /// Alarm when any probe exceeds the jointly calibrated threshold.
    pub fn rejects(&self, counts: &[Vec<u64>]) -> Result<bool> {
        Ok(self.score(counts)? > self.threshold + 1e-12)
    }
    /// Fit a maximum-TVD threshold from clean batches with the same probe context.
    /// `alpha` is a target tail fraction, not a guarantee under distribution drift.
    pub fn calibrate(
        references: Vec<Vec<f64>>,
        clean: &[Vec<Vec<u64>>],
        alpha: f64,
    ) -> Result<Self> {
        if clean.is_empty() || !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 {
            return Err(Error::Invalid("joint calibration"));
        }
        let mut verifier = Self {
            references,
            threshold: 1.0,
        };
        let mut scores = clean
            .iter()
            .map(|batch| verifier.score(batch))
            .collect::<Result<Vec<_>>>()?;
        scores.sort_by(f64::total_cmp);
        let rank = ((scores.len() + 1) as f64 * (1.0 - alpha)).ceil() as usize;
        verifier.threshold = scores.get(rank - 1).copied().unwrap_or(1.0);
        Ok(verifier)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// One reference distribution and its permitted total-variation distance.
pub struct CalibratedVerifier {
    pub reference: Vec<f64>,
    pub threshold: f64,
}
impl CalibratedVerifier {
    /// Check reference normalization, size, and threshold range.
    pub fn validate(&self) -> Result<()> {
        if self.reference.len() < 2
            || self.reference.len() > 65536
            || !self.reference.len().is_power_of_two()
            || self.reference.iter().any(|x| !x.is_finite() || *x < 0.0)
            || (self.reference.iter().sum::<f64>() - 1.0).abs() > 1e-12
            || !self.threshold.is_finite()
            || !(0.0..=1.0).contains(&self.threshold)
        {
            return Err(Error::Invalid("distribution or threshold"));
        }
        Ok(())
    }
    /// Measure total-variation distance from counts to the reference.
    pub fn score(&self, counts: &[u64]) -> Result<f64> {
        self.validate()?;
        let sum = counts
            .iter()
            .try_fold(0_u64, |s, x| s.checked_add(*x))
            .ok_or(Error::Invalid("count overflow"))?;
        if counts.len() != self.reference.len() || sum == 0 || sum > 1_u64 << 53 {
            return Err(Error::Invalid("counts"));
        }
        Ok(counts
            .iter()
            .zip(&self.reference)
            .map(|(n, p)| (*n as f64 / sum as f64 - p).abs())
            .sum::<f64>()
            / 2.0)
    }
    /// Alarm when the observed distance exceeds the configured threshold.
    pub fn rejects(&self, counts: &[u64]) -> Result<bool> {
        Ok(self.score(counts)? > self.threshold + 1e-12)
    }
    /// Fit a threshold from clean calibration counts for the same execution context.
    pub fn calibrate(reference: Vec<f64>, clean: &[Vec<u64>], alpha: f64) -> Result<Self> {
        if !alpha.is_finite() || !(0.0..1.0).contains(&alpha) || alpha == 0.0 || clean.is_empty() {
            return Err(Error::Invalid("calibration"));
        }
        let mut verifier = Self {
            reference,
            threshold: 1.0,
        };
        let mut scores = clean
            .iter()
            .map(|c| verifier.score(c))
            .collect::<Result<Vec<_>>>()?;
        scores.sort_by(f64::total_cmp);
        let rank = ((scores.len() + 1) as f64 * (1.0 - alpha)).ceil() as usize;
        verifier.threshold = scores.get(rank - 1).copied().unwrap_or(1.0);
        Ok(verifier)
    }
}
