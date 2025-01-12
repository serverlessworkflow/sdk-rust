use serde_derive::{Deserialize, Serialize};
use crate::models::any::*;
use crate::models::resource::*;

/// Provvides the default schema format
fn default_schema_format() -> String{
    SchemaFormat::JSON.to_string()
}

/// Enumerates all supported schema formats
pub struct SchemaFormat;
impl SchemaFormat {
    /// Gets the Avro schema format
    pub const AVRO: &'static str = "avro";
    /// Gets the JSON schema format
    pub const JSON: &'static str = "json";
    /// Gets the XML schema format
    pub const XML: &'static str = "xml";
}

/// Represents the definition of a schema
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaDefinition{

    /// Gets/sets the schema's format. Defaults to 'json'. The (optional) version of the format can be set using `{format}:{version}`.
    #[serde(rename = "format", default = "default_schema_format")]
    pub format : String,

    /// Gets/sets the schema's external resource, if any. Required if <see cref="Document"/> has not been set.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource : Option<ExternalResourceDefinition>,

    /// Gets/sets the inline definition of the schema to use. Required if <see cref="Resource"/> has not been set.
    #[serde(rename = "document", skip_serializing_if = "Option::is_none")]
    pub document : Option<AnyValue>

}