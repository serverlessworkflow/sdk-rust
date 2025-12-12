use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use crate::models::authentication::*;
use crate::models::catalog::*;
use crate::models::duration::*;
use crate::models::error::*;
use crate::models::event::*;
use crate::models::extension::*;
use crate::models::input::*;
use crate::models::map::*;
use crate::models::output::*;
use crate::models::retry::*;
use crate::models::task::*;
use crate::models::timeout::*;

/// Gets the namespace to use by default for workflow definitions
pub const DEFAULT_NAMESPACE: &str = "default";
// Provides the default namespace if not specified during deserialization
fn default_namespace() -> String {
    DEFAULT_NAMESPACE.to_string()
}

/// Gets the latest ServerlessWorkflow DSL version to use by default for workflow definitions
pub const LATEST_DSL_VERSION: &str = "1.0.0";
// Provides the latest ServerlessWorkflow DSL version
fn default_dsl_version() -> String {
    LATEST_DSL_VERSION.to_string()
}

// Provides the default runtime expression language
fn default_runtime_expression_language() -> String{
    RuntimeExpressionLanguage::JQ.to_string()
}

/// Enumerates all supported runtime expression languages
pub struct RuntimeExpressionLanguage;
impl RuntimeExpressionLanguage {
    /// Gets the 'jq' runtime expression language
    pub const JQ: &'static str = "jq";
    /// Gets the 'js' runtime expression language
    pub const JAVASCRIPT: &'static str = "js";
}

/// Represents the definition of a workflow
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDefinition{

    /// Gets/sets an object used to document the defined workflow
    #[serde(rename = "document")]
    pub document: WorkflowDefinitionMetadata,

    /// Gets/sets the workflow's input definition, if any
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<InputDataModelDefinition>,

    /// Gets/sets a collection that contains reusable components for the workflow definition
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub use_: Option<ComponentDefinitionCollection>,

    /// Gets/sets the workflow's timeout, if any
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<OneOfTimeoutDefinitionOrReference>,

    /// Gets/sets the workflow's output definition, if any
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<OutputDataModelDefinition>,

    /// Gets/sets the definition of the workflow's schedule, if any
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<WorkflowScheduleDefinition>,

    /// Gets/sets the configuration of how the runtime expressions
    #[serde(rename = "evaluate", skip_serializing_if = "Option::is_none")]
    pub evaluate: Option<RuntimeExpressionEvaluationConfiguration>,

    /// Gets/sets a name/value mapping of the tasks to perform
    #[serde(rename = "do")]
    pub do_: Map<String, TaskDefinition>,

    /// Gets/sets a key/value mapping, if any, of additional information associated with the workflow
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>

}
impl WorkflowDefinition {

    // Initializes a new workflow definition
    pub fn new(document: WorkflowDefinitionMetadata) -> Self {
        Self { 
            document, 
            input: None,
            use_: None,
            timeout: None, 
            output: None,
            schedule: None,
            evaluate: None,
            do_: Map::new(),
            metadata: None
        }
    }
    
}

/// Represents the metadata of a workflow, including its name, version, and description.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDefinitionMetadata {
    
    /// Gets/sets the version of the DSL used to define the workflow
    #[serde(rename = "dsl")]
    pub dsl: String,

    /// Gets/sets the workflow's namespace
    ///
    /// Defaults to [`DEFAULT_NAMESPACE`] if not specified.
    #[serde(rename = "namespace", default = "default_namespace")]
    pub namespace: String,

    /// Gets/sets the workflow's name
    #[serde(rename = "name")]
    pub name: String,

    /// Gets/sets the workflow's semantic version
    #[serde(rename = "version")]
    pub version: String,

    /// Gets/sets the workflow's title, if any
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Gets/sets the workflow's Markdown summary, if any
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Gets/sets a key/value mapping of the workflow's tags, if any
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>
}
impl WorkflowDefinitionMetadata{

    // Initializes a new workflow definition metadata
    pub fn new(namespace : &str, name: &str, version : &str, title: Option<String>, summary : Option<String>, tags: Option<HashMap<String, String>>) -> Self {
        Self { 
            dsl: default_dsl_version(),
            namespace: namespace.to_owned(),
            name: name.to_owned(),
            version: version.to_owned(),
            title,
            summary,
            tags
        }
    }

}

/// Represents the definition of a workflow's schedule
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowScheduleDefinition{

    /// Gets/sets an object used to document the defined workflow
    #[serde(rename = "every", skip_serializing_if = "Option::is_none")]
    pub every: Option<Duration>,

    /// Gets/sets the schedule using a CRON expression, e.g., '0 0 * * *' for daily at midnight.
    #[serde(rename = "cron", skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,

    /// Gets/sets a delay duration, if any, that the workflow must wait before starting again after it completes. In other words, when this workflow completes, it should run again after the specified amount of time.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<Duration>,

    /// Gets/sets the events that trigger the workflow execution
    #[serde(rename = "on", skip_serializing_if = "Option::is_none")]
    pub on: Option<EventConsumptionStrategyDefinition>

}

/// Represents an object used to configure the workflow's runtime expression evaluation
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeExpressionEvaluationConfiguration{

    /// Gets/sets the language used for writing runtime expressions
    #[serde(rename = "language", default = "default_runtime_expression_language")]
    pub language: String,

    /// Gets/sets the language used for writing runtime expressions
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>

}

/// Represents a collection of workflow components
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentDefinitionCollection{

    /// Gets/sets a name/value mapping of the workflow's reusable authentication policies
    #[serde(rename = "authentications", skip_serializing_if = "Option::is_none")]
    pub authentications: Option<HashMap<String, AuthenticationPolicyDefinition>>,

    /// Gets/sets a name/value mapping of the catalogs, if any, from which to import reusable components used within the workflow
    #[serde(rename = "catalogs", skip_serializing_if = "Option::is_none")]
    pub catalogs: Option<HashMap<String, CatalogDefinition>>,

    /// Gets/sets a name/value mapping of the workflow's errors, if any
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, ErrorDefinition>>,

    /// Gets/sets a list containing the workflow's extensions, if any
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<HashMap<String, ExtensionDefinition>>>,

    /// Gets/sets a name/value mapping of the workflow's reusable functions
    #[serde(rename = "functions", skip_serializing_if = "Option::is_none")]
    pub functions: Option<HashMap<String, TaskDefinition>>,

    /// Gets/sets a name/value mapping of the workflow's reusable retry policies
    #[serde(rename = "retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<HashMap<String, RetryPolicyDefinition>>,

    /// Gets/sets a list containing the workflow's secrets
    #[serde(rename = "secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<String>>,

    /// Gets/sets a name/value mapping of the workflow's reusable timeouts
    #[serde(rename = "timeouts", skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<HashMap<String, TimeoutDefinition>>,

}