use crate::{Circuit, Error, Gate, Result};
pub struct WorkloadTransforms;
impl WorkloadTransforms {
    /// Insert inverse rotation pairs around gates to test normalization overhead.
    pub fn identity_padding(c: &Circuit) -> Result<Circuit> {
        Circuit::new(
            c.width,
            c.gates
                .iter()
                .flat_map(|g| {
                    [
                        Gate::rotation("rx", 0, 0.731),
                        Gate::rotation("rx", 0, -0.731),
                        g.clone(),
                    ]
                })
                .collect(),
        )
    }
    /// Flip selected output qubits; the caller must retain the mask to decode results.
    pub fn mask_output(c: &Circuit, mask: usize) -> Result<Circuit> {
        c.validate()?;
        if mask >= 1 << c.width {
            return Err(Error::Invalid("output mask"));
        }
        c.appended((0..c.width).filter(|q| mask & (1 << q) != 0).map(Gate::x))
    }
    /// Undo the corresponding XOR permutation on a probability/count vector.
    pub fn recover<T: Clone>(values: &[T], mask: usize) -> Result<Vec<T>> {
        if !values.len().is_power_of_two() || mask >= values.len() {
            return Err(Error::Invalid("masked distribution"));
        }
        Ok((0..values.len())
            .map(|i| values[i ^ mask].clone())
            .collect())
    }
    /// Construct a Bell-state fixture for a simple correlation check.
    pub fn bell_predicate() -> Circuit {
        Circuit {
            width: 2,
            gates: vec![Gate::h(0), Gate::cx(0, 1)],
        }
    }
    /// Check that impossible Bell outcomes have zero observed counts in the fixture.
    pub fn predicate(counts: &[u64]) -> Result<bool> {
        if counts.len() != 4 {
            return Err(Error::Invalid("Bell counts"));
        }
        Ok(counts[1] == 0 && counts[2] == 0)
    }
}
