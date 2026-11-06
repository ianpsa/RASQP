use crate::{Error, Result};
use serde::Serialize;
pub fn probabilities(counts: &[u64]) -> Result<Vec<f64>> {
    let sum = counts
        .iter()
        .try_fold(0_u64, |s, n| s.checked_add(*n))
        .ok_or(Error::Invalid("count overflow"))?;
    if sum == 0 || sum > 1 << 53 {
        return Err(Error::Invalid("count total"));
    }
    Ok(counts.iter().map(|n| *n as f64 / sum as f64).collect())
}
pub fn tvd(a: &[f64], b: &[f64]) -> Result<f64> {
    if a.len() != b.len() || a.is_empty() || a.iter().chain(b).any(|x| !x.is_finite() || *x < 0.) {
        return Err(Error::Invalid("distribution shape"));
    }
    Ok(a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum::<f64>() / 2.)
}
pub fn wilson(successes: usize, total: usize) -> Result<[f64; 2]> {
    if successes > total {
        return Err(Error::Invalid("rate"));
    }
    if total == 0 {
        return Ok([0., 1.]);
    }
    let z = 1.959963984540054;
    let n = total as f64;
    let p = successes as f64 / n;
    let den = 1. + z * z / n;
    let center = (p + z * z / (2. * n)) / den;
    let radius = z * ((p * (1. - p) / n + z * z / (4. * n * n)).sqrt()) / den;
    Ok([center - radius, center + radius])
}
#[derive(Clone, Debug, Serialize)]
pub struct Rate {
    pub n: usize,
    pub rejected: usize,
    pub rate: f64,
    pub wilson95: [f64; 2],
}
impl Rate {
    pub fn from_flags(flags: &[bool]) -> Result<Self> {
        if flags.is_empty() {
            return Err(Error::Invalid("empty rate"));
        }
        let n = flags.len();
        let rejected = flags.iter().filter(|x| **x).count();
        Ok(Self {
            n,
            rejected,
            rate: rejected as f64 / n as f64,
            wilson95: wilson(rejected, n)?,
        })
    }
}
/// Stable binomial upper tail computed in log space; no SciPy runtime dependency.
pub fn binomial_tail(k: usize, n: usize, p: f64) -> Result<f64> {
    if k > n || !p.is_finite() || !(0.0..=1.0).contains(&p) || n > 1_000_000 {
        return Err(Error::Invalid("binomial parameters"));
    }
    if k == 0 {
        return Ok(1.);
    }
    if p == 0. {
        return Ok(0.);
    }
    if p == 1. {
        return Ok(1.);
    }
    let mut log_term = n as f64 * (1. - p).ln();
    let mut sum = 0.;
    for i in 0..=n {
        if i >= k {
            sum += log_term.exp();
        }
        if i < n {
            log_term += ((n - i) as f64).ln() - ((i + 1) as f64).ln() + p.ln() - (1. - p).ln();
        }
    }
    Ok(sum.min(1.))
}
pub fn binomial_two_sided(k: usize, n: usize, p: f64) -> Result<f64> {
    binomial_tail(k, n, p)?;
    if p == 0. {
        return Ok(if k == 0 { 1. } else { 0. });
    }
    if p == 1. {
        return Ok(if k == n { 1. } else { 0. });
    }
    let mut log_term = n as f64 * (1. - p).ln();
    let mut terms = Vec::with_capacity(n + 1);
    for i in 0..=n {
        terms.push(log_term);
        if i < n {
            log_term += ((n - i) as f64).ln() - ((i + 1) as f64).ln() + p.ln() - (1. - p).ln();
        }
    }
    Ok(terms
        .iter()
        .filter(|v| **v <= terms[k] + 1e-12)
        .map(|v| v.exp())
        .sum::<f64>()
        .min(1.))
}
#[derive(Clone, Debug, Serialize)]
pub struct PairedComparison {
    pub n: usize,
    pub gains: usize,
    pub losses: usize,
    pub p_one_sided: f64,
    pub delta: f64,
}
impl PairedComparison {
    pub fn evaluate(baseline: &[bool], candidate: &[bool]) -> Result<Self> {
        if baseline.len() != candidate.len() || baseline.is_empty() {
            return Err(Error::Invalid("paired data"));
        }
        let gains = baseline
            .iter()
            .zip(candidate)
            .filter(|(a, b)| !**a && **b)
            .count();
        let losses = baseline
            .iter()
            .zip(candidate)
            .filter(|(a, b)| **a && !**b)
            .count();
        Ok(Self {
            n: baseline.len(),
            gains,
            losses,
            p_one_sided: binomial_tail(gains, gains + losses, 0.5)?,
            delta: (gains as f64 - losses as f64) / baseline.len() as f64,
        })
    }
}
pub fn holm(pvalues: &[f64]) -> Result<Vec<f64>> {
    if pvalues
        .iter()
        .any(|p| !p.is_finite() || !(0.0..=1.0).contains(p))
    {
        return Err(Error::Invalid("p values"));
    }
    let mut order: Vec<_> = (0..pvalues.len()).collect();
    order.sort_by(|a, b| pvalues[*a].total_cmp(&pvalues[*b]));
    let mut adjusted = vec![0.; pvalues.len()];
    let mut running = 0_f64;
    for (k, i) in order.into_iter().enumerate() {
        running = running.max((pvalues.len() - k) as f64 * pvalues[i]);
        adjusted[i] = running.min(1.);
    }
    Ok(adjusted)
}
pub fn conformal_threshold(scores: &[f64], alpha: f64) -> Result<f64> {
    if scores.is_empty()
        || !alpha.is_finite()
        || alpha <= 0.
        || alpha >= 1.
        || scores
            .iter()
            .any(|s| !s.is_finite() || !(0.0..=1.0 + 1e-12).contains(s))
    {
        return Err(Error::Invalid("calibration"));
    }
    let mut scores = scores.to_vec();
    scores.sort_by(f64::total_cmp);
    let rank = ((scores.len() + 1) as f64 * (1. - alpha)).ceil() as usize;
    Ok(scores.get(rank - 1).copied().unwrap_or(1.).min(1.))
}
