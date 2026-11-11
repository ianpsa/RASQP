use crate::{Circuit, Error, Gate, Result};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Family {
    Basis,
    Ghz,
    Variational,
}
impl Family {
    /// Rust experiment RNG is ChaCha8, explicitly different from Python MT19937.
    pub fn circuit(self, width: u8, seed: u64) -> Result<Circuit> {
        if !(1..=16).contains(&width) {
            return Err(Error::Invalid("benchmark width"));
        }
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut gates = vec![];
        match self {
            Self::Basis => {
                for q in 0..width {
                    if rng.gen_bool(0.5) {
                        gates.push(Gate::x(q));
                    }
                }
            }
            Self::Ghz => {
                gates.push(Gate::h(0));
                for q in 1..width {
                    gates.push(Gate::cx(q - 1, q));
                }
            }
            Self::Variational => {
                for _ in 0..2 {
                    for q in 0..width {
                        gates.push(Gate::rotation("ry", q, rng.gen_range(-2.8..2.8)));
                    }
                    for q in 0..width - 1 {
                        gates.push(Gate::cx(q, q + 1));
                    }
                    for q in 0..width {
                        gates.push(Gate::rotation("rz", q, rng.gen_range(-2.8..2.8)));
                    }
                }
            }
        }
        Circuit::new(width, gates)
    }
}
pub struct MirrorTrap;
impl MirrorTrap {
    pub fn build(width: u8, seed: u64) -> Result<Circuit> {
        let forward = Family::Variational.circuit(width, seed)?;
        forward.appended(forward.gates.iter().rev().map(Gate::inverse))
    }
    pub fn recognizable(c: &Circuit) -> bool {
        let half = c.gates.len() / 2;
        c.gates.len().is_multiple_of(2)
            && c.gates[half..].iter().eq(c.gates[..half]
                .iter()
                .rev()
                .map(Gate::inverse)
                .collect::<Vec<_>>()
                .iter())
    }
}
