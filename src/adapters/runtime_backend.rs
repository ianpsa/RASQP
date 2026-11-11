//! Local simulator adapter for guarded execution. This is not a remote QPU driver.
use crate::{
    Circuit, Error, Request, Result, application::runtime::QuantumBackend, domain::QuantumExecutor,
};
use rand::RngCore;

use super::simulator::NativeExecutor;

/// Executes guarded jobs in the native noisy simulator using fresh random seeds.
pub struct NativeBackend {
    id: String,
    executor: NativeExecutor,
}

impl NativeBackend {
    /// Create a local backend with a fixed depolarizing probability.
    pub fn new(noise: f64) -> Result<Self> {
        let executor = NativeExecutor::new(noise)?;
        Ok(Self {
            id: format!("native-density-v1:{:016x}", noise.to_bits()),
            executor,
        })
    }
}

impl QuantumBackend for NativeBackend {
    fn backend_id(&self) -> &str {
        &self.id
    }

    fn execute(&mut self, request: &Request, circuit: &Circuit) -> Result<Vec<u64>> {
        request.validate()?;
        if request.backend != self.id
            || request.circuit_digest != circuit.digest()?
            || request.width != circuit.width
        {
            return Err(Error::Invalid("native job binding"));
        }
        let seed = rand::rngs::OsRng.next_u64();
        self.executor
            .sample(std::slice::from_ref(circuit), request.shots, seed)?
            .into_iter()
            .next()
            .ok_or(Error::Invalid("missing native result"))
    }
}
