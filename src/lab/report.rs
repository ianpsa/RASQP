//! Report generation from versioned native experiment summaries.
use crate::Result;
use serde_json::Value;
pub struct ResearchReport;
impl ResearchReport {
    pub fn render(summary: &Value) -> Result<String> {
        let mut text = format!(
            "# RASQP native Rust research run\n\nProtocol SHA256: `{}`. All quantum results are local simulation.\n\nNative RNG changes sampled counts and generated benchmarks; this is not an exact replay of Python random streams. No scientific novelty or quantum advantage is established.\n\n",
            summary["protocol_sha256"].as_str().unwrap_or("unknown")
        );
        if let Some(studies) = summary["studies"].as_object() {
            for (name, value) in studies {
                text.push_str(&format!(
                    "## {name}\n\n```json\n{}\n```\n\n",
                    serde_json::to_string_pretty(value)?
                ));
            }
        }
        text.push_str(&format!(
            "## Measured simulation cost\n\n```json\n{}\n```\n",
            serde_json::to_string_pretty(&summary["cost"])?
        ));
        Ok(text)
    }
}
