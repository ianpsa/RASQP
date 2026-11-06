use crate::{Circuit, Error, Gate, Result, domain::QuantumExecutor};
/// Restricted U=H nested Pauli gadget. Terminal flag measurement and conditional
/// data reset are represented explicitly by the classical output mapping below.
pub struct PauliSandwich {
    pub circuit: Circuit,
    pub dynamic_reset: bool,
}
impl PauliSandwich {
    /// Build the restricted nested Pauli-check research fixture.
    pub fn new(payload: &str, layers: u8, dynamic_reset: bool) -> Result<Self> {
        if !(1..=2).contains(&layers) || !["i", "x", "y", "z"].contains(&payload) {
            return Err(Error::Invalid("Pauli check fixture"));
        }
        let mut g = vec![Gate::rotation("ry", 0, 0.7)];
        for a in 1..=layers {
            g.push(Gate::h(a));
            if a == 1 {
                g.extend([Gate::h(0), Gate::cx(a, 0), Gate::h(0)]);
            } else {
                g.push(Gate::cx(a, 0));
            }
        }
        g.push(Gate::h(0));
        match payload {
            "x" => g.push(Gate::x(0)),
            "z" => g.push(Gate::z(0)),
            "y" => g.extend([Gate::x(0), Gate::z(0)]),
            _ => {}
        }
        for a in (1..=layers).rev() {
            if a == 1 {
                g.push(Gate::cx(a, 0));
            } else {
                g.extend([Gate::h(0), Gate::cx(a, 0), Gate::h(0)]);
            }
            g.push(Gate::h(a));
        }
        Ok(Self {
            circuit: Circuit::new(1 + layers, g)?,
            dynamic_reset,
        })
    }
    /// Sample fixture repetitions; optional reset is an explicit classical count map.
    pub fn sample(
        &self,
        executor: &mut impl QuantumExecutor,
        repetitions: usize,
        shots: u64,
        seed: u64,
    ) -> Result<Vec<Vec<u64>>> {
        let mut rows = executor.sample(&vec![self.circuit.clone(); repetitions], shots, seed)?;
        if self.dynamic_reset {
            for row in &mut rows {
                for index in 0..row.len() {
                    if index >> 1 != 0 && index & 1 == 1 {
                        row[index & !1] += row[index];
                        row[index] = 0;
                    }
                }
            }
        }
        Ok(rows)
    }
    /// Estimate probability that any ancilla flag was measured as one.
    pub fn flag_probability(counts: &[u64]) -> Result<f64> {
        let p = crate::research::statistics::probabilities(counts)?;
        Ok(p.iter()
            .enumerate()
            .filter(|(i, _)| i >> 1 != 0)
            .map(|(_, p)| p)
            .sum())
    }
}
