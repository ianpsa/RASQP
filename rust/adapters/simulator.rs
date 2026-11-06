//! Bounded native simulators for laboratory fixtures. Density matrices scale as 4^n.
use crate::{
    Circuit, Error, Gate, Result,
    domain::{EquivalenceOracle, ProbabilityOracle, QuantumExecutor},
};
use num_complex::Complex64 as C;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

fn matrix(g: &Gate) -> [C; 4] {
    let zero = C::new(0., 0.);
    let one = C::new(1., 0.);
    let i = C::new(0., 1.);
    let (s, c) = (g.angle / 2.).sin_cos();
    match g.name.as_str() {
        "x" => [zero, one, one, zero],
        "z" => [one, zero, zero, -one],
        "h" => {
            let h = C::new(std::f64::consts::FRAC_1_SQRT_2, 0.);
            [h, h, h, -h]
        }
        "rx" => [C::new(c, 0.), -i * s, -i * s, C::new(c, 0.)],
        "ry" => [C::new(c, 0.), C::new(-s, 0.), C::new(s, 0.), C::new(c, 0.)],
        "rz" => [C::new(c, -s), zero, zero, C::new(c, s)],
        _ => unreachable!("validated gate"),
    }
}
fn apply_vector(state: &mut [C], g: &Gate) {
    if g.name == "cx" {
        let control = 1 << g.qubits[0];
        let target = 1 << g.qubits[1];
        for j in 0..state.len() {
            if j & control != 0 && j & target == 0 {
                state.swap(j, j | target);
            }
        }
    } else {
        let bit = 1 << g.qubits[0];
        let m = matrix(g);
        for j in 0..state.len() {
            if j & bit == 0 {
                let k = j | bit;
                let (a, b) = (state[j], state[k]);
                state[j] = m[0] * a + m[1] * b;
                state[k] = m[2] * a + m[3] * b;
            }
        }
    }
}
#[derive(Default)]
pub struct Statevector;
impl Statevector {
    /// Simulate an ideal circuit from the all-zero input state.
    pub fn state(&self, c: &Circuit) -> Result<Vec<C>> {
        c.validate()?;
        let mut state = vec![C::new(0., 0.); 1 << c.width];
        state[0] = C::new(1., 0.);
        for g in &c.gates {
            apply_vector(&mut state, g);
        }
        Ok(state)
    }
    /// Build the ideal unitary operator for bounded circuit equivalence checks.
    pub fn operator(&self, c: &Circuit) -> Result<Vec<Vec<C>>> {
        c.validate()?;
        if c.width > 8 {
            return Err(Error::Invalid("operator oracle limited to 8 qubits"));
        }
        let n = 1 << c.width;
        let mut columns = Vec::with_capacity(n);
        for j in 0..n {
            let mut v = vec![C::new(0., 0.); n];
            v[j] = C::new(1., 0.);
            for g in &c.gates {
                apply_vector(&mut v, g);
            }
            columns.push(v);
        }
        Ok(columns)
    }
}
impl ProbabilityOracle for Statevector {
    fn probabilities(&self, c: &Circuit) -> Result<Vec<f64>> {
        Ok(self.state(c)?.iter().map(C::norm_sqr).collect())
    }
}
impl EquivalenceOracle for Statevector {
    fn equivalent(&self, a: &Circuit, b: &Circuit) -> Result<bool> {
        if a.width != b.width {
            return Ok(false);
        }
        let a = self.operator(a)?;
        let b = self.operator(b)?;
        let mut phase = None;
        for (a, b) in a.iter().flatten().zip(b.iter().flatten()) {
            if a.norm() > 1e-10 && b.norm() > 1e-10 {
                phase = Some(*a / *b);
                break;
            }
            if (a.norm() - b.norm()).abs() > 1e-10 {
                return Ok(false);
            }
        }
        let Some(phase) = phase else { return Ok(false) };
        Ok(a.iter()
            .flatten()
            .zip(b.iter().flatten())
            .all(|(a, b)| (*a - phase * *b).norm() < 1e-10))
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NoiseModel {
    pub single_qubit: f64,
    pub two_qubit: f64,
    pub readout: f64,
}
impl NoiseModel {
    /// Build the declared local depolarizing/readout fixture from one probability.
    pub fn depolarizing(p: f64) -> Result<Self> {
        let model = Self {
            single_qubit: p,
            two_qubit: (2. * p).min(1.),
            readout: p,
        };
        model.validate()?;
        Ok(model)
    }
    /// Ensure every channel probability is finite and within zero to one.
    pub fn validate(&self) -> Result<()> {
        if [self.single_qubit, self.two_qubit, self.readout]
            .iter()
            .any(|p| !p.is_finite() || !(0.0..=1.0).contains(p))
        {
            return Err(Error::Invalid("noise probability"));
        }
        Ok(())
    }
}
pub struct DensityMatrix {
    pub noise: NoiseModel,
}
impl DensityMatrix {
    /// Create a bounded noisy simulator; the model is not hardware calibration.
    pub fn new(noise: NoiseModel) -> Result<Self> {
        noise.validate()?;
        Ok(Self { noise })
    }
    fn apply_unitary(rho: &mut [C], n: usize, g: &Gate) {
        if g.name == "cx" {
            let control = 1 << g.qubits[0];
            let target = 1 << g.qubits[1];
            for a in 0..n {
                if a & control != 0 && a & target == 0 {
                    for b in 0..n {
                        rho.swap(a * n + b, (a | target) * n + b);
                    }
                }
            }
            for b in 0..n {
                if b & control != 0 && b & target == 0 {
                    for a in 0..n {
                        rho.swap(a * n + b, a * n + (b | target));
                    }
                }
            }
        } else {
            let bit = 1 << g.qubits[0];
            let m = matrix(g);
            for a in 0..n {
                if a & bit == 0 {
                    for b in 0..n {
                        let i = a * n + b;
                        let j = (a | bit) * n + b;
                        let (x, y) = (rho[i], rho[j]);
                        rho[i] = m[0] * x + m[1] * y;
                        rho[j] = m[2] * x + m[3] * y;
                    }
                }
            }
            for b in 0..n {
                if b & bit == 0 {
                    for a in 0..n {
                        let i = a * n + b;
                        let j = a * n + (b | bit);
                        let (x, y) = (rho[i], rho[j]);
                        rho[i] = m[0].conj() * x + m[1].conj() * y;
                        rho[j] = m[2].conj() * x + m[3].conj() * y;
                    }
                }
            }
        }
    }
    fn depolarize(rho: &mut [C], n: usize, qubits: &[u8], p: f64) {
        if p == 0. {
            return;
        }
        let mask = qubits.iter().fold(0_usize, |m, q| m | (1 << q));
        let local: Vec<usize> = (0..1 << qubits.len())
            .map(|t| {
                qubits
                    .iter()
                    .enumerate()
                    .fold(0, |a, (i, q)| a | (((t >> i) & 1) << q))
            })
            .collect();
        let old = rho.to_vec();
        for a in 0..n {
            for b in 0..n {
                let mixed = if a & mask == b & mask {
                    local
                        .iter()
                        .map(|t| old[((a & !mask) | t) * n + ((b & !mask) | t)])
                        .sum::<C>()
                        / local.len() as f64
                } else {
                    C::new(0., 0.)
                };
                rho[a * n + b] = old[a * n + b] * (1. - p) + mixed * p;
            }
        }
    }
}
impl ProbabilityOracle for DensityMatrix {
    fn probabilities(&self, c: &Circuit) -> Result<Vec<f64>> {
        c.validate()?;
        self.noise.validate()?;
        if c.width > 8 {
            return Err(Error::Invalid("density simulator limited to 8 qubits"));
        }
        let n = 1 << c.width;
        let mut rho = vec![C::new(0., 0.); n * n];
        rho[0] = C::new(1., 0.);
        for g in &c.gates {
            Self::apply_unitary(&mut rho, n, g);
            Self::depolarize(
                &mut rho,
                n,
                &g.qubits,
                if g.qubits.len() == 2 {
                    self.noise.two_qubit
                } else {
                    self.noise.single_qubit
                },
            );
        }
        let mut p: Vec<_> = (0..n).map(|i| rho[i * n + i].re.max(0.)).collect();
        for q in 0..c.width {
            for i in 0..n {
                if i & (1 << q) == 0 {
                    let j = i | (1 << q);
                    let (a, b) = (p[i], p[j]);
                    p[i] = a * (1. - self.noise.readout) + b * self.noise.readout;
                    p[j] = b * (1. - self.noise.readout) + a * self.noise.readout;
                }
            }
        }
        let sum = p.iter().sum::<f64>();
        if !sum.is_finite() || sum <= 0. {
            return Err(Error::Invalid("simulated state norm"));
        }
        for v in &mut p {
            *v /= sum;
        }
        Ok(p)
    }
}
#[derive(Default, Clone, Debug, Serialize)]
pub struct ExecutionCost {
    pub jobs: u64,
    pub circuits: u64,
    pub shots: u64,
}
pub struct NativeExecutor {
    engine: DensityMatrix,
    cache: BTreeMap<String, Vec<f64>>,
    pub cost: ExecutionCost,
}
impl NativeExecutor {
    /// Create a seeded experiment executor using the local noisy simulator.
    pub fn new(noise: f64) -> Result<Self> {
        Ok(Self {
            engine: DensityMatrix::new(NoiseModel::depolarizing(noise)?)?,
            cache: BTreeMap::new(),
            cost: ExecutionCost::default(),
        })
    }
    /// Draw counts from a validated probability vector for a fixed shot count.
    pub fn sample_distribution(p: &[f64], shots: u64, rng: &mut ChaCha8Rng) -> Result<Vec<u64>> {
        if shots == 0
            || shots > 100_000_000
            || p.is_empty()
            || p.iter().any(|x| !x.is_finite() || *x < 0.)
            || (p.iter().sum::<f64>() - 1.).abs() > 1e-9
        {
            return Err(Error::Invalid("sampling inputs"));
        }
        let mut counts = vec![0; p.len()];
        let mut cumulative = Vec::with_capacity(p.len());
        let mut sum = 0.;
        for x in p {
            sum += x;
            cumulative.push(sum);
        }
        for _ in 0..shots {
            let v = rng.r#gen::<f64>();
            let index = cumulative.partition_point(|x| *x <= v).min(p.len() - 1);
            counts[index] += 1;
        }
        Ok(counts)
    }
}
impl QuantumExecutor for NativeExecutor {
    fn sample(&mut self, circuits: &[Circuit], shots: u64, seed: u64) -> Result<Vec<Vec<u64>>> {
        if circuits.is_empty() {
            return Err(Error::Invalid("empty execution"));
        }
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut result = vec![];
        for c in circuits {
            let key = c.digest()?;
            if !self.cache.contains_key(&key) {
                if self.cache.len() >= 1024 {
                    self.cache.clear();
                }
                self.cache
                    .insert(key.clone(), self.engine.probabilities(c)?);
            }
            result.push(Self::sample_distribution(
                &self.cache[&key],
                shots,
                &mut rng,
            )?);
        }
        self.cost.jobs += 1;
        self.cost.circuits += circuits.len() as u64;
        self.cost.shots += circuits.len() as u64 * shots;
        Ok(result)
    }
}
