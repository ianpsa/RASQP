use crate::{Circuit, Result};
pub struct OpenQasm;
impl OpenQasm {
    /// Portable IBM-compatible OpenQASM 2 export; no remote job submission or credentials.
    pub fn export(c: &Circuit, measure: bool) -> Result<String> {
        c.validate()?;
        let mut out = format!(
            "OPENQASM 2.0;\ninclude \"qelib1.inc\";\nqreg q[{}];\n",
            c.width
        );
        if measure {
            out.push_str(&format!("creg c[{}];\n", c.width));
        }
        for g in &c.gates {
            let name = if g.name.starts_with('r') {
                format!("{}({:0.17})", g.name, g.angle)
            } else {
                g.name.clone()
            };
            out.push_str(&format!(
                "{} {};\n",
                name,
                g.qubits
                    .iter()
                    .map(|q| format!("q[{q}]"))
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }
        if measure {
            out.push_str("measure q -> c;\n");
        }
        Ok(out)
    }
}
