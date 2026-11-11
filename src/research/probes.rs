use crate::{Circuit, Error, Gate, Result, domain::ProbabilityOracle, research::statistics::tvd};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Preparation state and measurement basis used to expose circuit differences.
pub struct Probe {
    pub input_state: String,
    pub basis: String,
}
impl Probe {
    /// Build a uniform preparation/basis choice for every qubit.
    pub fn global(width: u8, state: char, basis: char) -> Self {
        Self {
            input_state: state.to_string().repeat(width as usize),
            basis: basis.to_string().repeat(width as usize),
        }
    }
    /// Wrap a circuit with preparation gates before and basis rotation after it.
    pub fn apply(&self, c: &Circuit) -> Result<Circuit> {
        c.validate()?;
        if self.input_state.len() != c.width as usize || self.basis.len() != c.width as usize {
            return Err(Error::Invalid("probe width"));
        }
        let mut before = vec![];
        let mut after = vec![];
        for (q, state) in self.input_state.chars().enumerate() {
            let q = q as u8;
            match state {
                '0' => {}
                '1' => before.push(Gate::x(q)),
                '+' | 'i' => {
                    before.push(Gate::h(q));
                    if state == 'i' {
                        before.push(Gate::rotation("rz", q, std::f64::consts::FRAC_PI_2));
                    }
                }
                _ => return Err(Error::Invalid("probe input")),
            }
        }
        for (q, basis) in self.basis.chars().enumerate() {
            let q = q as u8;
            match basis {
                'Z' => {}
                'X' => after.push(Gate::h(q)),
                'Y' => after.extend([
                    Gate::rotation("rz", q, -std::f64::consts::FRAC_PI_2),
                    Gate::h(q),
                ]),
                _ => return Err(Error::Invalid("probe basis")),
            }
        }
        before.extend(c.gates.clone());
        before.extend(after);
        Circuit::new(c.width, before)
    }
    /// Enumerate the fixed research candidate set of global states and bases.
    pub fn catalogue(width: u8) -> Vec<Self> {
        ['0', '1', '+', 'i']
            .into_iter()
            .flat_map(|s| {
                ['Z', 'X', 'Y']
                    .into_iter()
                    .map(move |b| Self::global(width, s, b))
            })
            .collect()
    }
}
#[derive(Debug, Serialize)]
pub struct ProbeSelection {
    pub probes: Vec<Probe>,
    pub training_tvd_matrix: Vec<Vec<f64>>,
    pub objective: f64,
}
/// Offline selector that compares approved and suspect circuits using an oracle.
/// Selection and calibration must precede guarded runtime execution.
pub struct ProbeSelector<'a> {
    pub oracle: &'a dyn ProbabilityOracle,
}
impl ProbeSelector<'_> {
    /// Score each candidate probe against each supplied suspect using TVD.
    pub fn matrix(
        &self,
        reference: &Circuit,
        suspects: &[Circuit],
    ) -> Result<(Vec<Probe>, Vec<Vec<f64>>)> {
        if suspects.is_empty() {
            return Err(Error::Invalid("empty suspects"));
        }
        let probes = Probe::catalogue(reference.width);
        let mut matrix = vec![];
        for p in &probes {
            let a = self.oracle.probabilities(&p.apply(reference)?)?;
            matrix.push(
                suspects
                    .iter()
                    .map(|c| tvd(&a, &self.oracle.probabilities(&p.apply(c)?)?))
                    .collect::<Result<_>>()?,
            );
        }
        Ok((probes, matrix))
    }
    /// Select one probe with the largest average separation over supplied suspects.
    pub fn single(&self, reference: &Circuit, suspects: &[Circuit]) -> Result<ProbeSelection> {
        let (probes, matrix) = self.matrix(reference, suspects)?;
        let mut best = 0;
        let mut objective = -1.;
        for (i, row) in matrix.iter().enumerate() {
            let score = row.iter().sum::<f64>() / row.len() as f64;
            if score > objective {
                best = i;
                objective = score;
            }
        }
        Ok(ProbeSelection {
            probes: vec![probes[best].clone()],
            training_tvd_matrix: matrix,
            objective,
        })
    }
    /// Select a fixed-size portfolio maximizing the worst suspect's best probe score.
    pub fn portfolio(
        &self,
        reference: &Circuit,
        suspects: &[Circuit],
        size: usize,
    ) -> Result<ProbeSelection> {
        let (probes, matrix) = self.matrix(reference, suspects)?;
        if size == 0 || size > probes.len() {
            return Err(Error::Invalid("portfolio size"));
        }
        let mut selected: Vec<usize> = (0..size).collect();
        let mut best = selected.clone();
        let mut objective = -1.;
        loop {
            let score = (0..suspects.len())
                .map(|j| selected.iter().map(|i| matrix[*i][j]).fold(0_f64, f64::max))
                .fold(1_f64, f64::min);
            if score > objective {
                objective = score;
                best = selected.clone();
            }
            let pivot = (0..size)
                .rev()
                .find(|i| selected[*i] < probes.len() - size + *i);
            let Some(i) = pivot else { break };
            selected[i] += 1;
            for j in i + 1..size {
                selected[j] = selected[j - 1] + 1;
            }
        }
        Ok(ProbeSelection {
            probes: best.iter().map(|i| probes[*i].clone()).collect(),
            training_tvd_matrix: matrix,
            objective,
        })
    }
}
