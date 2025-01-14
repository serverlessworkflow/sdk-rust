use serde_derive::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

/// Represents any possible value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnyValue {
    String(String),
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    Vec(Vec<AnyValue>),
    HashMap(std::collections::HashMap<String, AnyValue>),
    Json(JsonValue),
    Yaml(YamlValue)
}
impl Default for AnyValue {
    fn default() -> Self {
        // Choose a default variant. For example, default to an empty Uri.
        AnyValue::String(String::new())
    }
}