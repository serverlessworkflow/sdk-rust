use serde_derive::{Deserialize, Serialize};
use crate::models::any::*;

/// Represents the definition an error to raise
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition{

    /// Gets/sets an uri that reference the type of the described error
    #[serde(rename = "type")]
    pub type_: String,

    /// Gets/sets a short, human-readable summary of the error type.It SHOULD NOT change from occurrence to occurrence of the error, except for purposes of localization
    #[serde(rename = "title")]
    pub title: String,

    /// Gets/sets the status code produced by the described error
    #[serde(rename = "status")]
    pub status: AnyValue,

    /// Gets/sets a human-readable explanation specific to this occurrence of the error.
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    ///  Gets/sets a <see cref="Uri"/> reference that identifies the specific occurrence of the error.It may or may not yield further information if dereferenced
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>

}
impl ErrorDefinition{
    
    /// Initializes a new ErrorDefinition
    pub fn new(type_: &str, title: &str, status: AnyValue, detail: Option<String>, instance: Option<String>) -> Self{
        Self { 
            type_: type_.to_string(), 
            title: title.to_string(),
            status, 
            detail, 
            instance 
        }
    }

}

/// Represents a value that can be either a ErrorDefinition or a reference to a ErrorDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOfErrorDefinitionOrReference{
    /// Variant holding an error
    Error(ErrorDefinition),
    /// Variant holding a reference to the error to use
    Reference(String)
}
impl Default for OneOfErrorDefinitionOrReference {
    fn default() -> Self {
        // Choose a default variant
        OneOfErrorDefinitionOrReference::Error(ErrorDefinition::default())
    }
}