use serde_derive::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

/// Represents any possible value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnyValue {
    /// Variant holding a string
    String(String),
    /// Variant holding a JSON value
    JsonValue(JsonValue),
    /// Variant holding a YAML value 
    YamlValue(YamlValue)
}
impl Default for AnyValue {
    fn default() -> Self {
        // Choose a default variant. For example, default to an empty Uri.
        AnyValue::String(String::new())
    }
}