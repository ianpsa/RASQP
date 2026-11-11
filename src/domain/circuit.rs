use crate::{Error, Result, artifact_digest, put_u64};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gate {
    pub name: String,
    pub qubits: Vec<u8>,
    #[serde(default)]
    pub angle: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Circuit {
    pub width: u8,
    pub gates: Vec<Gate>,
}
impl Gate {
    pub fn new(name: impl Into<String>, qubits: impl Into<Vec<u8>>, angle: f64) -> Self {
        Self {
            name: name.into(),
            qubits: qubits.into(),
            angle,
        }
    }
    pub fn x(q: u8) -> Self {
        Self::new("x", vec![q], 0.)
    }
    pub fn h(q: u8) -> Self {
        Self::new("h", vec![q], 0.)
    }
    pub fn z(q: u8) -> Self {
        Self::new("z", vec![q], 0.)
    }
    pub fn cx(control: u8, target: u8) -> Self {
        Self::new("cx", vec![control, target], 0.)
    }
    pub fn rotation(axis: &str, q: u8, angle: f64) -> Self {
        Self::new(axis, vec![q], angle)
    }
    pub fn inverse(&self) -> Self {
        let mut g = self.clone();
        if g.name.starts_with('r') {
            g.angle = -g.angle;
        }
        g
    }
}
fn opcode(name: &str) -> Option<u8> {
    ["x", "h", "z", "rx", "ry", "rz", "cx"]
        .iter()
        .position(|x| *x == name)
        .map(|i| i as u8)
}
impl Circuit {
    pub fn new(width: u8, gates: Vec<Gate>) -> Result<Self> {
        let c = Self { width, gates };
        c.validate()?;
        Ok(c)
    }
    pub fn identity(width: u8) -> Result<Self> {
        Self::new(width, vec![])
    }
    pub fn appended(&self, gates: impl IntoIterator<Item = Gate>) -> Result<Self> {
        let mut result = self.clone();
        result.gates.extend(gates);
        result.validate()?;
        Ok(result)
    }
    pub fn validate(&self) -> Result<()> {
        if !(1..=16).contains(&self.width) || self.gates.len() > 4096 {
            return Err(Error::Invalid("circuit size"));
        }
        for g in &self.gates {
            if opcode(&g.name).is_none()
                || g.qubits.len() != if g.name == "cx" { 2 } else { 1 }
                || g.qubits.iter().any(|q| *q >= self.width)
                || (g.qubits.len() == 2 && g.qubits[0] == g.qubits[1])
                || !g.angle.is_finite()
                || (!g.name.starts_with('r') && g.angle != 0.0)
            {
                return Err(Error::Invalid("gate"));
            }
        }
        Ok(())
    }
    /// Version 1 binary encoding; intentionally incompatible with legacy JSON hashes.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = b"RASQP-CIRCUIT\0\x01".to_vec();
        out.push(self.width);
        put_u64(&mut out, self.gates.len() as u64);
        for g in &self.gates {
            out.push(opcode(&g.name).unwrap());
            out.push(g.qubits.len() as u8);
            out.extend(&g.qubits);
            let angle = if g.angle == 0.0 { 0.0_f64 } else { g.angle };
            out.extend(angle.to_le_bytes());
        }
        Ok(out)
    }
    pub fn digest(&self) -> Result<String> {
        Ok(artifact_digest(&self.canonical_bytes()?))
    }
    /// Conservative local rewrite system, not a complete semantic canonical form.
    pub fn normalize(&self) -> Result<Self> {
        Ok(crate::research::normalization::Normalizer::default()
            .run(self)?
            .circuit)
    }
}
#[derive(Debug, Serialize, Deserialize)]
/// Result of the conservative circuit and resource preflight.
pub struct Inspection {
    pub decision: crate::Decision,
    pub normalized_equal: bool,
    pub extra_gates: i64,
}
/// Compare a submitted circuit with an approved one after local normalization.
/// A width or gate-budget violation rejects; an unproven semantic difference is
/// inconclusive, so normalized inequality is never treated as an attack proof.
pub fn inspect(
    approved: &Circuit,
    candidate: &Circuit,
    max_extra_gates: usize,
) -> Result<Inspection> {
    let equal = approved.normalize()? == candidate.normalize()?;
    let extra = candidate.gates.len() as i64 - approved.gates.len() as i64;
    let decision = if approved.width != candidate.width
        || candidate.gates.len().saturating_sub(approved.gates.len()) > max_extra_gates
    {
        crate::Decision::Reject
    } else if equal {
        crate::Decision::Accept
    } else {
        crate::Decision::Inconclusive
    };
    Ok(Inspection {
        decision,
        normalized_equal: equal,
        extra_gates: extra,
    })
}
