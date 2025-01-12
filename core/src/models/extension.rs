use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::task::*;

/// Represents the definition of a an extension
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionDefinition{

    /// Gets/sets the type of task to extend
    #[serde(rename = "extend")]
    pub extend: String,

    /// Gets/sets a runtime expression, if any, used to determine whether or not the extension should apply in the specified context
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,

    /// Gets/sets a name/definition map, if any, of the tasks to execute before the extended task
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<HashMap<String, TaskDefinition>>,

    /// Gets/sets a name/definition map, if any, of the tasks to execute after the extended task
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<HashMap<String, TaskDefinition>>

}