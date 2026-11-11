use crate::{Circuit, Error, Gate, Result};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rewrite {
    pub rule: String,
    pub offset: usize,
    pub before: Vec<Gate>,
    pub after: Vec<Gate>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Normalized {
    pub circuit: Circuit,
    pub trace: Vec<Rewrite>,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Resources {
    pub gates: usize,
    pub depth: usize,
    pub two_qubit: usize,
}
impl Resources {
    /// Count gates, circuit depth and two-qubit operations in the raw circuit.
    pub fn measure(c: &Circuit) -> Result<Self> {
        c.validate()?;
        let mut depths = vec![0; c.width as usize];
        for g in &c.gates {
            let d = g.qubits.iter().map(|q| depths[*q as usize]).max().unwrap() + 1;
            for q in &g.qubits {
                depths[*q as usize] = d;
            }
        }
        Ok(Self {
            gates: c.gates.len(),
            depth: *depths.iter().max().unwrap(),
            two_qubit: c.gates.iter().filter(|g| g.qubits.len() == 2).count(),
        })
    }
}
pub struct Normalizer {
    pub max_work: u64,
}
impl Default for Normalizer {
    fn default() -> Self {
        Self {
            max_work: 1_000_000,
        }
    }
}
impl Normalizer {
    /// Apply bounded, semantics-preserving local rewrites and record each change.
    /// The result is not a complete semantic normal form.
    pub fn run(&self, c: &Circuit) -> Result<Normalized> {
        c.validate()?;
        let mut gates = c.gates.clone();
        let mut trace = vec![];
        let mut work = 0;
        loop {
            let mut rewrite = None;
            for i in 0..gates.len() {
                work += 1;
                if work > self.max_work {
                    return Err(Error::Invalid("normalization work budget exceeded"));
                }
                let g = &gates[i];
                let mut rule = "";
                let mut length = 0;
                let mut after = vec![];
                if g.name.starts_with('r') && g.angle == 0. {
                    rule = "zero_rotation";
                    length = 1;
                } else if let Some(h) = gates.get(i + 1) {
                    if g == h && !g.name.starts_with('r') {
                        rule = "involution";
                        length = 2;
                    } else if g.name == h.name
                        && g.qubits == h.qubits
                        && g.name.starts_with('r')
                        && g.angle == -h.angle
                    {
                        rule = "inverse_rotation";
                        length = 2;
                    }
                    if rule.is_empty()
                        && let Some(last) = gates.get(i + 2)
                        && g.name == "h"
                        && last.name == "h"
                        && g.qubits == h.qubits
                        && h.qubits == last.qubits
                        && ["x", "z"].contains(&h.name.as_str())
                    {
                        rule = "hadamard_conjugation";
                        length = 3;
                        after.push(Gate::new(
                            if h.name == "x" { "z" } else { "x" },
                            g.qubits.clone(),
                            0.,
                        ));
                    }
                    if rule.is_empty()
                        && g.qubits.iter().all(|q| !h.qubits.contains(q))
                        && (&g.qubits, &g.name) > (&h.qubits, &h.name)
                    {
                        rule = "disjoint_order";
                        length = 2;
                        after = vec![h.clone(), g.clone()];
                    }
                }
                if !rule.is_empty() {
                    rewrite = Some(Rewrite {
                        rule: rule.into(),
                        offset: i,
                        before: gates[i..i + length].to_vec(),
                        after,
                    });
                    break;
                }
            }
            let Some(r) = rewrite else { break };
            gates.splice(r.offset..r.offset + r.before.len(), r.after.clone());
            trace.push(r);
        }
        Ok(Normalized {
            circuit: Circuit::new(c.width, gates)?,
            trace,
        })
    }
    /// Recompute and compare a normalization trace for reproducibility.
    pub fn replay(&self, c: &Circuit, normalized: &Normalized) -> Result<bool> {
        Ok(self.run(c)? == *normalized)
    }
}
