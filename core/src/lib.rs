pub mod models;

#[cfg(test)]
mod unit_tests {

    use crate::models::duration::*;
    use crate::models::workflow::*;
    use crate::models::task::*;
    use crate::models::map::*;
    use serde_json::json;

    #[test]
    fn create_workflow() {
        let namespace = "fake-namespace";
        let name = "fake-workflow";
        let version = "1.0.0";
        let title = Some("fake-title".to_string());
        let summary = Some("fake-summary".to_string());
        let document = WorkflowDefinitionMetadata::new(namespace, name, version, title.clone(), summary.clone(), None);
        let mut call_task = CallTaskDefinition::new("http", None, Some(true));
        call_task.common.then = Some("continue".to_string());
        let do_task = DoTaskDefinition::new(Map::from(vec![("set".to_string(), TaskDefinition::Wait(WaitTaskDefinition::new(OneOfDurationOrIso8601Expression::Duration(Duration::from_milliseconds(200)))))]));
        let mut workflow = WorkflowDefinition::new(document);
        workflow.do_ = Map::new();
        workflow.do_.add("callTask".to_string(), TaskDefinition::Call(call_task));
        workflow.do_.add("doTask".to_string(), TaskDefinition::Do(do_task));
        let json_serialization_result = serde_json::to_string_pretty(&workflow);
        let yaml_serialization_result = serde_yaml::to_string(&workflow);
        assert!(json_serialization_result.is_ok(), "JSON Serialization failed: {:?}", json_serialization_result.err());
        assert!(yaml_serialization_result.is_ok(), "YAML Serialization failed: {:?}", yaml_serialization_result.err());
        if let Result::Ok(yaml) = yaml_serialization_result{
            println!("{}", yaml)
        }
        assert_eq!(workflow.document.namespace, namespace);
        assert_eq!(workflow.document.name, name);
        assert_eq!(workflow.document.version, version);
        assert_eq!(workflow.document.title, title);
        assert_eq!(workflow.document.summary, summary);
    }

    #[test]
    fn test_for_loop_definition_each_field_deserialization() {
        // This test verifies that ForLoopDefinition correctly deserializes "each"
        let for_loop_json = serde_json::json!({
            "each": "item",
            "in": ".items"
        });

        let result: Result<ForLoopDefinition, _> = serde_json::from_value(for_loop_json);

        match result {
            Ok(for_loop) => {
                assert_eq!(for_loop.each, "item", "The 'each' field should be 'item'");
                assert_eq!(for_loop.in_, ".items", "The 'in' field should be '.items'");
            }
            Err(e) => {
                panic!(
                    "Failed to deserialize ForLoopDefinition with 'each' field: {}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_for_task_deserialization() {
        // This is a valid For task - it has a "for" field and a "do" field
        let for_task_json = json!({
            "for": {
                "each": "item",
                "in": ".items"
            },
            "do": [
                {
                    "processItem": {
                        "call": "processFunction",
                        "with": {
                            "item": "${ .item }"
                        }
                    }
                }
            ]
        });

        let result: Result<TaskDefinition, _> = serde_json::from_value(for_task_json.clone());

        match result {
            Ok(TaskDefinition::For(for_def)) => {
                assert_eq!(for_def.for_.each, "item");
                assert_eq!(for_def.for_.in_, ".items");
                assert_eq!(for_def.do_.entries.len(), 1);
                let has_process_item = for_def
                    .do_
                    .entries
                    .iter()
                    .any(|entry| entry.contains_key("processItem"));
                assert!(
                    has_process_item,
                    "For task should contain processItem subtask"
                );
            }
            Ok(TaskDefinition::Do(_)) => {
                panic!("For task incorrectly deserialized as DoTaskDefinition");
            }
            Ok(other) => {
                panic!("For task deserialized as unexpected variant: {:?}", other);
            }
            Err(e) => {
                panic!("Failed to deserialize For task: {}", e);
            }
        }
    }

    #[test]
    fn test_do_task_deserialization() {
        // This is a valid Do task
        let do_task_json = json!({
            "do": [
                {
                    "step1": {
                        "call": "function1"
                    }
                },
                {
                    "step2": {
                        "call": "function2"
                    }
                }
            ]
        });

        let result: Result<TaskDefinition, _> = serde_json::from_value(do_task_json);

        match result {
            Ok(TaskDefinition::Do(do_def)) => {
                assert_eq!(do_def.do_.entries.len(), 2);
                let has_step1 = do_def
                    .do_
                    .entries
                    .iter()
                    .any(|entry| entry.contains_key("step1"));
                let has_step2 = do_def
                    .do_
                    .entries
                    .iter()
                    .any(|entry| entry.contains_key("step2"));
                assert!(has_step1, "Do task should contain step1");
                assert!(has_step2, "Do task should contain step2");
            }
            Ok(other) => {
                panic!("Do task deserialized as unexpected variant: {:?}", other);
            }
            Err(e) => {
                panic!("Failed to deserialize Do task: {}", e);
            }
        }
    }

    #[test]
    fn test_for_task_with_while_condition() {
        // TestFor task with a while condition
        let for_task_json = json!({
            "for": {
                "each": "user",
                "in": ".users",
                "at": "index"
            },
            "while": "${ .index < 10 }",
            "do": [
                {
                    "notifyUser": {
                        "call": "notifyUser",
                        "with": {
                            "user": "${ .user }",
                            "index": "${ .index }"
                        }
                    }
                }
            ]
        });

        let result: Result<TaskDefinition, _> = serde_json::from_value(for_task_json.clone());

        match result {
            Ok(TaskDefinition::For(for_def)) => {
                assert_eq!(for_def.for_.each, "user");
                assert_eq!(for_def.for_.in_, ".users");
                assert_eq!(for_def.for_.at, Some("index".to_string()));
                assert_eq!(for_def.while_, Some("${ .index < 10 }".to_string()));
                assert_eq!(for_def.do_.entries.len(), 1);
            }
            Ok(TaskDefinition::Do(_)) => {
                panic!("For task incorrectly deserialized as DoTaskDefinition");
            }
            Ok(other) => {
                panic!("For task deserialized as unexpected variant: {:?}", other);
            }
            Err(e) => {
                panic!("Failed to deserialize For task with while: {}", e);
            }
        }
    }

    #[test]
    fn test_roundtrip_serialization() {
        // Create a ForTaskDefinition programmatically

        let for_loop = ForLoopDefinition::new("item", ".collection", None, None);
        let mut do_tasks = Map::new();
        do_tasks.add(
            "task1".to_string(),
            TaskDefinition::Call(CallTaskDefinition::new("someFunction", None, None)),
        );

        let for_task = ForTaskDefinition::new(for_loop, do_tasks, None);
        let task_def = TaskDefinition::For(for_task);

        // Serialize to JSON
        let json_str = serde_json::to_string(&task_def).expect("Failed to serialize");
        println!("Serialized: {}", json_str);

        // Deserialize back
        let deserialized: TaskDefinition =
            serde_json::from_str(&json_str).expect("Failed to deserialize");

        // Should still be a For task
        match deserialized {
            TaskDefinition::For(for_def) => {
                assert_eq!(for_def.for_.each, "item");
                assert_eq!(for_def.for_.in_, ".collection");
            }
            TaskDefinition::Do(_) => {
                panic!("After roundtrip serialization, For task became a Do task");
            }
            other => {
                panic!("Unexpected variant after roundtrip: {:?}", other);
            }
        }
    }
}