pub mod services;

#[cfg(test)]
mod unit_tests {

    use serverless_workflow_core::models::any::*;
    use serverless_workflow_core::models::duration::*;
    use serverless_workflow_core::models::error::OneOfErrorDefinitionOrReference;
    use crate::services::workflow::WorkflowBuilder;
    use serverless_workflow_core::models::task::*;
    use serverless_workflow_core::models::timeout::*;
    use std::collections::HashMap;

    #[test]
    fn build_workflow_should_work() {
        //arrange
        let dsl_version = "1.0.0";
        let namespace = "namespace";
        let name = "fake-name";
        let version = "1.0.0";
        let title = "fake-title";
        let summary = "fake-summary";
        let tags: HashMap<String, String> = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())]
            .into_iter()
            .collect();   
        let mut timeout_duration = Duration::default();
        timeout_duration.minutes = Some(69);
        let basic_name = "fake-basic";
        let username = "fake-username";
        let password = "fake-password";
        let call_task_name = "call-task";
        let call_function_name = "fake-function";
        let call_task_with: HashMap<String, AnyValue> = vec![
            ("key1".to_string(), AnyValue::String("value1".to_string())),
            ("key2".to_string(), AnyValue::String("value2".to_string()))]
            .into_iter()
            .collect(); 
        let do_task_name = "do-task"; 
        let emit_task_name = "emit-task";
        let emit_event_attributes: HashMap<String, AnyValue> = vec![
            ("key1".to_string(), AnyValue::String("value1".to_string())),
            ("key2".to_string(), AnyValue::String("value2".to_string()))]
            .into_iter()
            .collect();
        let for_task_name = "for-task";
        let for_each = "item";
        let for_each_in = "items";
        let for_each_at = "index";
        let fork_task_name = "fork-task";
        let listen_task_name = "listen-task";
        let raise_task_name = "raise-task-name";
        let raise_error_type = "error-type";
        let raise_error_status = AnyValue::Int16(400);
        let raise_error_title = "error-title";
        let raise_error_detail = "error-detail";
        let raise_error_instance = "error-instance";
        let run_task_name = "run-task-name";
        let set_task_name = "set-task-name";
        let switch_task_name = "switch-task-name";
        let try_task_name = "try-task-name";
        let wait_task_name = "wait-task";
        let wait_duration = OneOfDurationOrIso8601Expression::Duration(Duration::from_days(3));
        
        //act 
        let workflow = WorkflowBuilder::new()
            .use_dsl(dsl_version)
            .with_namespace(namespace)
            .with_name(name)
            .with_version(version)
            .with_title(title)
            .with_summary(summary)
            .with_tags(tags.clone())
            .with_timeout(|t| { t.after(timeout_duration.clone()); })
            .use_authentication(basic_name, |a| { 
                a.basic()
                    .with_username(username)
                    .with_password(password);})
            .do_(call_task_name, |t| {
                t.call(call_function_name)
                    .with_arguments(call_task_with.clone());
            })
            .do_(do_task_name, |t| {
                t.do_()
                    .do_("fake-wait-task", |st| {
                        st.wait(OneOfDurationOrIso8601Expression::Duration(Duration::from_seconds(25)));
                    });
            })
            .do_(wait_task_name, |t| {
                t.wait(wait_duration.clone());
            })
            .do_(emit_task_name, |t| {
                t.emit(|e|{
                    e.with_attributes(emit_event_attributes.clone());
                });
            })
            .do_(for_task_name, |t| {
                t.for_()
                    .each(for_each)
                    .in_(for_each_in)
                    .at(for_each_at)
                    .do_("fake-wait-task", |st| {
                        st.wait(OneOfDurationOrIso8601Expression::Duration(Duration::from_seconds(25)));
                    });
            })
            .do_(fork_task_name, |t| {
                t.fork()
                    .branch(|b| {
                        b.do_("fake-wait-task", |st| {
                            st.wait(OneOfDurationOrIso8601Expression::Duration(Duration::from_seconds(25)));
                        });
                    });
            })
            .do_(listen_task_name, |t| {
                t.listen()
                    .to(|e|{
                        e.one()
                            .with("key", AnyValue::String("value".to_string()));
                    });
            })
            .do_(raise_task_name, |t| {
                t.raise()
                    .error()
                        .with_type(raise_error_type)
                        .with_status(raise_error_status)
                        .with_title(raise_error_title)
                        .with_detail(raise_error_detail)
                        .with_instance(raise_error_instance);
            })
            .build();

        //assert
        assert_eq!(workflow.document.dsl, dsl_version);
        assert_eq!(workflow.document.namespace, namespace);
        assert_eq!(workflow.document.name, name);
        assert_eq!(workflow.document.version, version);
        assert_eq!(workflow.document.title, Some(title.to_string()));
        assert_eq!(workflow.document.summary, Some(summary.to_string()));
        assert_eq!(workflow.document.tags, Some(tags));
        assert_eq!(workflow.timeout
            .as_ref()
            .and_then(|t| match t {
                OneOfTimeoutDefinitionOrReference::Timeout(definition) => match &definition.after {
                    OneOfDurationOrIso8601Expression::Duration(duration) => Some(duration),
                    OneOfDurationOrIso8601Expression::Iso8601Expression(_) => None,
                },
                OneOfTimeoutDefinitionOrReference::Reference(_) => None}),
            Some(&timeout_duration));
        assert!(
            workflow.use_.as_ref()
                .and_then(|component_collection| component_collection.authentications.as_ref())
                .and_then(|authentications| authentications.get(basic_name))
                .map(|auth_policy| auth_policy.basic.is_some())
                .unwrap_or(false),
            "Expected authentications to contain an entry with the name '{}' and a non-null `basic` property.",
            basic_name);
        assert!(
            workflow.do_
                .entries
                .iter()
                .any(|entry| entry.get(&call_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::Call(call_task) = task {
                        call_task.call == call_function_name && call_task.with == Some(call_task_with.clone())
                    } else {
                        false
                    }
                })),
            "Expected a task with key '{}' and a CallTaskDefinition with 'call'={} and 'with'={:?}",
            call_task_name,
            call_function_name,
            call_task_with);
        assert!(
            workflow.do_
                .entries
                .iter()
                .any(|entry| entry.get(&do_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::Do(_do_task) = task {
                        true
                    } else {
                        false
                    }
                })),
            "Expected a do task with key '{}'",
            do_task_name);
        assert!(
            workflow.do_
                .entries
                .iter()
                .any(|entry| entry.get(&emit_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::Emit(emit_task) = task {
                        emit_task.emit.event.with == emit_event_attributes.clone()
                    } else {
                        false
                    }
                })),
            "Expected a task with key '{}' and a EmitTaskDefinition with 'emit.event.with' matching supplied attributes",
            emit_task_name);  
        assert!(
            workflow.do_
                .entries
                .iter()
                .any(|entry| entry.get(&for_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::For(for_task) = task {
                        for_task.for_.each == for_each && for_task.for_.in_ == for_each_in && for_task.for_.at == Some(for_each_at.to_string())
                    } else {
                        false
                    }
                })),
            "Expected a task with key '{}' and a ForTaskDefinition with 'for.each'={}, 'for.in'={}' and 'for.at'={}'",
            for_task_name,
            for_each,
            for_each_in,
            for_each_at);
        assert!(
            workflow.do_
                .entries
                .iter()
                .any(|entry| entry.get(&fork_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::Fork(_fork_task) = task {
                        true
                    } else {
                        false
                    }
                })),
            "Expected a fork task with key '{}'",
            fork_task_name,);
        assert!(
            workflow.do_
                .entries
                .iter()
                .any(|entry| entry.get(&listen_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::Listen(_listen_task) = task {
                        true
                    } else {
                        false
                    }
                })),
            "Expected a listen task with key '{}'",
            listen_task_name);
        assert!(
            workflow.do_
                .entries
                .iter()
                .any(|entry| entry.get(&raise_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::Raise(raise_task) = task {
                        if let OneOfErrorDefinitionOrReference::Error(error) = &raise_task.raise.error {
                            error.type_ == raise_error_type
                                && error.title == raise_error_title
                                && error.detail == Some(raise_error_detail.to_string())
                                && error.instance == Some(raise_error_instance.to_string())
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })),
            "Expected a task with key '{}' and a RaiseTaskDefinition with 'raise.error.type'={}, 'raise.error.title'={}, 'raise.error.detail'={} and 'raise.error.instance'={}",
            raise_task_name,
            raise_error_type,
            raise_error_title,
            raise_error_detail,
            raise_error_instance);
        assert!(
            workflow.do_
                .entries
                .iter()
                .any(|entry| entry.get(&wait_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::Wait(wait_task) = task {
                        wait_task.duration == wait_duration
                    } else {
                        false
                    }
                })),
            "Expected a task with key '{}' and a WaitTaskDefinition with 'duration'={}",
            wait_task_name,
            wait_duration);
    }

}