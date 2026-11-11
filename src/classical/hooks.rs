//! Explicit fixture guard: function identity plus caller-supplied code bytes.
//! Safe Rust cannot introspect Python bytecode/closures. No memory attestation claim.
use crate::artifact_digest;
pub struct FunctionGuard {
    identity: usize,
    code_digest: String,
}
impl FunctionGuard {
    /// Capture a function pointer and caller-supplied code snapshot as a baseline.
    pub fn capture(function: fn(&[u64]) -> usize, code_bytes: &[u8]) -> Self {
        Self {
            identity: function as usize,
            code_digest: artifact_digest(code_bytes),
        }
    }
    /// Detect replacement of the function pointer; does not inspect its machine code.
    pub fn identity_changed(&self, current: fn(&[u64]) -> usize) -> bool {
        current as usize != self.identity
    }
    /// Detect changes in the code bytes explicitly supplied by the caller.
    pub fn code_changed(&self, current_code: &[u8]) -> bool {
        artifact_digest(current_code) != self.code_digest
    }
}
