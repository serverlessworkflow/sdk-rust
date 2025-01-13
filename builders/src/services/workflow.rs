use crate::services::authentication::*;
use crate::services::task::*;
use crate::services::timeout::*;
use serverless_workflow_core::models::timeout::*;
use serverless_workflow_core::models::workflow::*;
use std::collections::HashMap;

/// Represents the service used to build workflows
pub struct WorkflowBuilder{
    workflow: WorkflowDefinition
}
impl WorkflowBuilder{

    /// Initializes a new WorkflowBuilder
    pub fn new() -> Self{
        Self { 
            workflow: WorkflowDefinition::default() 
        }
    }

    /// Sets the semantic version of the Serverless Workflow DSL used to define the workflow
    pub fn use_dsl(mut self, version: &str) -> Self {
        self.workflow.document.dsl = version.to_string();
        self
    }

    /// Sets the workflow's namespace
    pub fn with_namespace(mut self, namespace: &str) -> Self{
        self.workflow.document.namespace = namespace.to_string();
        self
    }

    /// Sets the workflow's name
    pub fn with_name(mut self, name: &str) -> Self{
        self.workflow.document.name = name.to_string();
        self
    }

    /// Sets the workflow's version
    pub fn with_version(mut self, version: &str) -> Self{
        self.workflow.document.version = version.to_string();
        self
    }

    /// Sets the workflow's title
    pub fn with_title(mut self, title: &str) -> Self{
        self.workflow.document.title = Some(title.to_string());
        self
    }

    /// Sets the workflow's summary
    pub fn with_summary(mut self, summary: &str) -> Self{
        self.workflow.document.summary = Some(summary.to_string());
        self
    }

    /// Adds a new tag to the workflow
    pub fn with_tag(mut self, name: &str, value: &str) -> Self{
        if self.workflow.document.tags.is_none(){
            self.workflow.document.tags = Some(HashMap::new());
        }
        if let Some(tags) = &mut self.workflow.document.tags {
            tags.insert(name.to_string(), value.to_string());
        }
        self
    }

    /// Sets the tags of the workflow
    pub fn with_tags(mut self, tags: HashMap<String, String>) -> Self{
        self.workflow.document.tags = Some(tags);
        self
    }

    /// Sets the workflow's timeout
    pub fn with_timeout_reference(mut self, reference: &str) -> Self{
        self.workflow.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the workflow's timeout
    pub fn with_timeout<F>(mut self, setup: F) -> Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.workflow.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Uses the specified authentication policy
    pub fn use_authentication<F>(mut self, name: &str, setup: F) -> Self
    where F: FnOnce(&mut AuthenticationPolicyDefinitionBuilder){
        let mut builder = AuthenticationPolicyDefinitionBuilder::new();
        setup(&mut builder);
        let authentication = builder.build();
        if self.workflow.use_.is_none(){
            self.workflow.use_ = Some(ComponentDefinitionCollection::default());
        }
        if let Some(resources) = &mut self.workflow.use_ {
            if resources.authentications.is_none(){
                resources.authentications = Some(HashMap::new());
            }
            if let Some(authentications) = &mut resources.authentications{
                authentications.insert(name.to_string(), authentication);
            }
        };
        self
    }

    /// Adds a new task with the specified name to the builder
    pub fn do_<F>(mut self, name: &str, setup: F) -> Self
    where F: FnOnce(&mut GenericTaskDefinitionBuilder){
        let mut builder = GenericTaskDefinitionBuilder::new();
        setup(&mut builder);
        let task = builder.build();
        self.workflow.do_.add(name.to_string(), task);
        self
    }

    /// Builds the configure WorkflowDefinition
    pub fn build(self) -> WorkflowDefinition{
        self.workflow
    }

}