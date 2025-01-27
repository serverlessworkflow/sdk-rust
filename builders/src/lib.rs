pub mod services;

#[cfg(test)]
mod unit_tests {

    use serde_json::Value;
    use serverless_workflow_core::models::duration::*;
    use serverless_workflow_core::models::error::OneOfErrorDefinitionOrReference;
    use crate::services::workflow::WorkflowBuilder;
    use serverless_workflow_core::models::task::*;
    use serverless_workflow_core::models::timeout::*;
    use std::collections::HashMap;
    use serde_json::json;

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
        let call_task_with: HashMap<String, Value> = vec![
            ("key1".to_string(), Value::String("value1".to_string())),
            ("key2".to_string(), Value::String("value2".to_string()))]
            .into_iter()
            .collect(); 
        let do_task_name = "do-task"; 
        let emit_task_name = "emit-task";
        let emit_event_attributes: HashMap<String, Value> = vec![
            ("key1".to_string(), Value::String("value1".to_string())),
            ("key2".to_string(), Value::String("value2".to_string()))]
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
        let raise_error_status = json!(400);
        let raise_error_title = "error-title";
        let raise_error_detail = "error-detail";
        let raise_error_instance = "error-instance";
        let run_container_task_name = "run-container-task-name";
        let container_image = "container-image-name";
        let container_command = "container-command";
        let container_ports: HashMap<u16, u16> = vec![
            (8080, 8081),
            (8082, 8083)]
            .into_iter()
            .collect();
        let container_volumes: HashMap<String, String> = vec![
            ("volume-1".to_string(), "/some/fake/path".to_string())]
            .into_iter()
            .collect();
        let container_environment: HashMap<String, String> = vec![
            ("env1-name".to_string(), "env1-value".to_string()),
            ("env2-name".to_string(), "env2-value".to_string())]
            .into_iter()
            .collect();
        let run_script_task_name = "run-script-task-name";
        let script_code = "script-code";
        let run_shell_task_name = "run-shell-task-name";
        let shell_command_name = "run-shell-command";
        let run_workflow_task_name = "run-workflow-task-name";
        let workflow_namespace = "workflow-namespace";
        let workflow_name = "workflow-name";
        let workflow_version = "workflow-version";
        let workflow_input = json!({"hello": "world"});
        let set_task_name = "set-task-name";
        let set_task_variables : HashMap<String, Value> = vec![
            ("var1-name".to_string(), json!("var1-value".to_string())),
            ("var2-name".to_string(), json!(69))]
            .into_iter()
            .collect();
        let switch_task_name = "switch-task-name";
        let switch_case_name = "switch-case-name";
        let switch_case_when = "true";
        let switch_case_then = "continue";
        let try_task_name = "try-task-name";
        let catch_when = "catch-when";
        let catch_errors_attributes: HashMap<String, Value> = vec![
            ("var1-name".to_string(), json!("var1-value".to_string())),
            ("var2-name".to_string(), json!(69))]
            .into_iter()
            .collect();
        let retry_except_when = "retry-except-when";
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
            .do_(call_task_name, |task| {
                task.call(call_function_name)
                    .with_arguments(call_task_with.clone());
            })
            .do_(do_task_name, |task| {
                task.do_()
                    .do_("fake-wait-task", |st| {
                        st.wait(OneOfDurationOrIso8601Expression::Duration(Duration::from_seconds(25)));
                    });
            })
            .do_(emit_task_name, |task| {
                task.emit(|e|{
                    e.with_attributes(emit_event_attributes.clone());
                });
            })
            .do_(for_task_name, |task| {
                task.for_()
                    .each(for_each)
                    .in_(for_each_in)
                    .at(for_each_at)
                    .do_("fake-wait-task", |st| {
                        st.wait(OneOfDurationOrIso8601Expression::Duration(Duration::from_seconds(25)));
                    });
            })
            .do_(fork_task_name, |task| {
                task.fork()
                    .branch(|b| {
                        b.do_("fake-wait-task", |st| {
                            st.wait(OneOfDurationOrIso8601Expression::Duration(Duration::from_seconds(25)));
                        });
                    });
            })
            .do_(listen_task_name, |task| {
                task.listen()
                    .to(|e|{
                        e.one()
                            .with("key", Value::String("value".to_string()));
                    });
            })
            .do_(raise_task_name, |task| {
                task.raise()
                    .error()
                        .with_type(raise_error_type)
                        .with_status(raise_error_status)
                        .with_title(raise_error_title)
                        .with_detail(raise_error_detail)
                        .with_instance(raise_error_instance);
            })
            .do_(run_container_task_name, |task|{
                task.run()
                    .container()
                        .with_image(container_image)
                        .with_command(container_command)
                        .with_ports(container_ports.clone())
                        .with_volumes(container_volumes.clone())
                        .with_environment_variables(container_environment.clone());
            })
            .do_(run_script_task_name, |task|{
                task.run()
                    .script()
                        .with_code(script_code);
            })
            .do_(run_shell_task_name, |task|{
                task.run()
                    .shell()
                        .with_command(shell_command_name);
            })
            .do_(run_workflow_task_name, |task|{
                task.run()
                    .workflow()
                        .with_namespace(workflow_namespace)
                        .with_name(workflow_name)
                        .with_version(workflow_version)
                        .with_input(workflow_input.clone());
            })
            .do_(set_task_name, |task|{
                task.set()
                    .variables(set_task_variables.clone());
            })
            .do_(switch_task_name, |task|{
                task.switch()
                    .case(switch_case_name, |case|{
                        case.when(switch_case_when)
                            .then(switch_case_then);
                    });

            })
            .do_(try_task_name, |task|{
                task.try_()
                    .do_(|tasks|{
                        tasks
                            .do_("fake-wait-task", |subtask|{
                                subtask.wait(OneOfDurationOrIso8601Expression::Duration(Duration::from_seconds(5)));
                            });
                    })
                    .catch(|catch| {
                        catch
                            .errors(|errors|{
                                errors
                                    .with_attributes(catch_errors_attributes.clone());
                        })
                            .when(catch_when)
                            .retry(|retry|{
                                retry
                                    .except_when(retry_except_when)
                                    .delay(Duration::from_seconds(1))
                                    .backoff(|backoff|{
                                        backoff
                                            .linear()
                                                .with_increment(Duration::from_milliseconds(500));
                                    })
                                    .jitter(|jitter|{
                                        jitter
                                            .from(Duration::from_seconds(1))
                                            .to(Duration::from_seconds(3));
                                    });
                                });
                    });
            })
            .do_(wait_task_name, |task| {
                task.wait(wait_duration.clone());
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
            workflow
                .do_
                .entries
                .iter()
                .any(|entry| entry.get(&run_container_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::Run(run_task) = task {
                        if let Some(container) = &run_task.run.container {
                            container.image == container_image
                                && container.command == Some(container_command.to_string())
                                && container.ports == Some(container_ports.clone())
                                && container.volumes == Some(container_volumes.clone())
                                && container.environment == Some(container_environment.clone())
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })),
            "Expected a task with key '{}' and a RunTaskDefinition with 'container.image'={}, 'container.command'={}, 'container.ports'={:?}, 'container.volumes'={:?}, and 'container.environment'={:?}",
            run_container_task_name,
            container_image,
            container_command,
            container_ports,
            container_volumes,
            container_environment);
        assert!(
            workflow
                .do_
                .entries
                .iter()
                .any(|entry| entry.get(&run_workflow_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::Run(run_task) = task {
                        if let Some(subflow) = &run_task.run.workflow{
                            subflow.namespace == workflow_namespace.to_string()
                                && subflow.name == workflow_name.to_string()
                                && subflow.version == workflow_version.to_string()
                                && subflow.input == Some(workflow_input.clone())
                        }
                        else{
                            false
                        }
                    } else {
                        false
                    }
                })),
            "Expected a task with key '{}' and a RunTaskDefinition with 'workflow.namespace'={}, 'workflow.name'={}, 'workflow.version'={}, and 'workflow.input'={:?}",
            run_container_task_name,
            workflow_namespace,
            workflow_name,
            workflow_version,
            workflow_input);
        assert!(
            workflow
                .do_
                .entries
                .iter()
                .any(|entry| entry.get(&set_task_name.to_string()).map_or(false, |task|{
                    if let TaskDefinition::Set(set_task) = task {
                        set_task.set == set_task_variables.clone()
                    }
                    else{
                        false
                    }
                })),
            "Expected a task with key '{}' and a SetTaskDefinition with specified variables",
            set_task_name);
        assert!(
            workflow
                .do_
                .entries
                .iter()
                .any(|entry| entry.get(&switch_task_name.to_string()).map_or(false, |task|{
                    if let TaskDefinition::Switch(switch_task) = task{
                        switch_task
                            .switch
                            .entries
                            .iter()
                            .any(|case| case.contains_key(switch_case_name))
                    }
                    else{
                        false
                    }
                })),
            "Expected a task with key '{}' and a SwitchTaskDefinition with a case named '{}'",
            set_task_name,
            switch_case_name);
        assert!(
            workflow.do_
                .entries
                .iter()
                .any(|entry| entry.get(&try_task_name.to_string()).map_or(false, |task| {
                    if let TaskDefinition::Try(try_task) = task {
                        try_task.catch.when == Some(catch_when.to_string())
                    } else {
                        false
                    }
                })),
            "Expected a task with key '{}' and a TryTaskDefinition with 'catch.when'={}",
            try_task_name,
            catch_when);
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