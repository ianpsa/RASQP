use crate::{Circuit, Gate, Result};
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mutation {
    AppendX,
    AppendZ,
    SmallRotation,
    DropGate,
    ParameterShift,
    EquivalentRewrite,
}
impl Mutation {
    /// Generate a specified synthetic circuit mutation for testing a detector.
    pub fn apply(self, c: &Circuit) -> Result<Circuit> {
        c.validate()?;
        let mut gates = c.gates.clone();
        match self {
            Self::AppendX => gates.push(Gate::x(0)),
            Self::AppendZ => gates.push(Gate::z(0)),
            Self::SmallRotation => gates.push(Gate::rotation("ry", 0, 0.04)),
            Self::DropGate => {
                if !gates.is_empty() {
                    gates.remove(gates.len() / 2);
                }
            }
            Self::ParameterShift => {
                if let Some(g) = gates.iter_mut().find(|g| g.name.starts_with('r')) {
                    g.angle += 0.4;
                }
            }
            Self::EquivalentRewrite => gates.extend([Gate::x(0), Gate::x(0)]),
        }
        Circuit::new(c.width, gates)
    }
}
