use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::schema::*;

/// Represents the definition of an input data model
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputDataModelDefinition{

    /// Gets/sets the schema, if any, that defines and describes the input data of a workflow or task
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema : Option<SchemaDefinition>,
    
    /// Gets/sets a runtime expression, if any, used to build the workflow or task input data based on both input and scope data
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from : Option<Value>

}