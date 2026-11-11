use crate::{Circuit, Gate, Result};
pub trait CircuitPass {
    fn run(&self, circuit: &Circuit) -> Result<Circuit>;
}
/// Controlled local compiler mutation fixture, not installed into external compilers.
pub struct InjectGatePass {
    pub angle: Option<f64>,
}
impl CircuitPass for InjectGatePass {
    fn run(&self, c: &Circuit) -> Result<Circuit> {
        c.appended([self
            .angle
            .map_or_else(|| Gate::x(0), |angle| Gate::rotation("ry", 0, angle))])
    }
}
pub struct NormalizePass;
impl CircuitPass for NormalizePass {
    fn run(&self, c: &Circuit) -> Result<Circuit> {
        c.normalize()
    }
}
#[derive(Default)]
pub struct PassManager {
    passes: Vec<Box<dyn CircuitPass>>,
}
impl PassManager {
    pub fn add(&mut self, pass: impl CircuitPass + 'static) {
        self.passes.push(Box::new(pass));
    }
    pub fn run(&self, c: &Circuit) -> Result<Circuit> {
        let mut output = c.clone();
        for pass in &self.passes {
            output = pass.run(&output)?;
        }
        Ok(output)
    }
}
