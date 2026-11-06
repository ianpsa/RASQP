pub mod circuit;
pub mod workload;
use crate::{Circuit, Result};
/// Backend port. Seeded sampling is for reproducible experiments, not secrets.
pub trait QuantumExecutor {
    /// Sample ordered circuits for an offline experiment with a reproducible seed.
    fn sample(&mut self, circuits: &[Circuit], shots: u64, seed: u64) -> Result<Vec<Vec<u64>>>;
}
pub trait ProbabilityOracle {
    /// Return the modeled computational-basis outcome probabilities of a circuit.
    fn probabilities(&self, circuit: &Circuit) -> Result<Vec<f64>>;
}
pub trait EquivalenceOracle {
    /// Determine whether two circuits implement the same unitary up to global phase.
    fn equivalent(&self, left: &Circuit, right: &Circuit) -> Result<bool>;
}
