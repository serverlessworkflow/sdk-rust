use serde_derive::{Deserialize, Serialize};
use crate::models::resource::*;

/// Gets the name of the default catalog
pub const DEFAULT_CATALOG_NAME: &str = "default";

/// Represents the definition of a workflow component catalog
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogDefinition{

    /// Gets/sets the endpoint that defines the root URL at which the catalog is located
    #[serde(rename = "endpoint")]
    pub endpoint: OneOfEndpointDefinitionUri

}