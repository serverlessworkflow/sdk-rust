use serde_derive::{Deserialize, Serialize};
use crate::models::any::*;
use crate::models::schema::*;

/// Represents the definition of an output data model
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputDataModelDefinition{

        /// Gets/sets the schema, if any, that defines and describes the output data of a workflow or task
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<SchemaDefinition>,
    
       /// Gets/sets a runtime expression, if any, used to output specific data to the scope data
    #[serde(rename = "as", skip_serializing_if = "Option::is_none")]
    pub as_: Option<AnyValue>

}