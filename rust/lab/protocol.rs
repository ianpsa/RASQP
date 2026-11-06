use crate::Result;
use serde_json::Value;
pub struct ProtocolRegistry;
impl ProtocolRegistry {
    pub fn native() -> Result<Value> {
        Ok(serde_json::from_str(include_str!(
            "../../docs/NATIVE_PROTOCOL_V1.json"
        ))?)
    }
    pub fn hypotheses() -> Result<Value> {
        Ok(serde_json::from_str(include_str!(
            "../../docs/HYPOTHESIS_PROTOCOL.json"
        ))?)
    }
    pub fn extension() -> Result<Value> {
        Ok(serde_json::from_str(include_str!(
            "../../docs/EXTENSION_PROTOCOL_V03.json"
        ))?)
    }
    pub fn context() -> Result<Value> {
        Ok(serde_json::from_str(include_str!(
            "../../docs/EXTENSION_PROTOCOL_V04.json"
        ))?)
    }
}
