pub mod models;

#[cfg(test)]
mod unit_tests {

    use crate::models::duration::*;
    use crate::models::workflow::*;
    use crate::models::task::*;
    use crate::models::map::*;

    #[test]
    fn create_workflow() {
        let namespace = "fake-namespace";
        let name = "fake-workflow";
        let version = "1.0.0";
        let title = Some("fake-title".to_string());
        let summary = Some("fake-summary".to_string());
        let document = WorkflowDefinitionMetadata::new(namespace, name, version, title.clone(), summary.clone(), None);
        let mut workflow = WorkflowDefinition::new(document);
        workflow.do_ = Map::new();
        workflow.do_.add("callTask".to_string(), TaskDefinition::Call(CallTaskDefinition::new("http", None, Some(true))));
        workflow.do_.add("doTask".to_string(), TaskDefinition::Do(DoTaskDefinition::new(Map::from(vec![("set".to_string(), TaskDefinition::Wait(WaitTaskDefinition::new(Duration::default())))]))));
        let json_serialization_result = serde_json::to_string_pretty(&workflow);
        let yaml_serialization_result = serde_yaml::to_string(&workflow);
        assert!(json_serialization_result.is_ok(), "JSON Serialization failed: {:?}", json_serialization_result.err());
        assert!(yaml_serialization_result.is_ok(), "YAML Serialization failed: {:?}", yaml_serialization_result.err());
        assert_eq!(workflow.document.namespace, namespace);
        assert_eq!(workflow.document.name, name);
        assert_eq!(workflow.document.version, version);
        assert_eq!(workflow.document.title, title);
        assert_eq!(workflow.document.summary, summary);
    }

}