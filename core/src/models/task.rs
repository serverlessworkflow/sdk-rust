use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use crate::models::any::*;
use crate::models::duration::*;
use crate::models::event::*;
use crate::models::error::*;
use crate::models::map::*;
use crate::models::input::*;
use crate::models::resource::*;
use crate::models::retry::*;

/// Enumerates all supported task types
pub struct TaskType;
impl TaskType {
    /// Gets the type of a 'call' task
    pub const CALL: &'static str = "call";
    /// Gets the type of a 'do' task
    pub const DO: &'static str = "do";
    /// Gets the type of a 'emit' task
    pub const EMIT: &'static str = "emit";
    /// Gets the type of a 'for' task
    pub const FOR: &'static str = "for";
    /// Gets the type of a 'fork' task
    pub const FORK: &'static str = "fork";
    /// Gets the type of a 'listen' task
    pub const LISTEN: &'static str = "listen";
    /// Gets the type of a 'raise' task
    pub const RAISE: &'static str = "raise";
    /// Gets the type of a 'run' task
    pub const RUN: &'static str = "run";
    /// Gets the type of a 'set' task
    pub const SET: &'static str = "set";
    /// Gets the type of a 'switch' task
    pub const SWITCH: &'static str = "switch";
    /// Gets the type of a 'try' task
    pub const TRY: &'static str = "try";
    /// Gets the type of a 'wait' task
    pub const WAIT: &'static str = "wait";
}

/// Enumerates all supported process types
pub struct ProcessType;
impl ProcessType {
    /// Gets the type of a 'container' process
    pub const CONTAINER: &'static str = "container";
    /// Gets the type of a 'shell' process
    pub const SCRIPT: &'static str = "script";
    /// Gets the type of a 'shell' process
    pub const SHELL: &'static str = "shell";
    /// Gets the type of a 'workflow' process
    pub const WORKFLOW: &'static str = "workflow";
}

/// Represents a value that can be any of the supported task definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskDefinition{
    /// Variant holding the definition of a 'call' task
    Call(CallTaskDefinition),
    /// Variant holding the definition of a 'do' task
    Do(DoTaskDefinition),
    /// Variant holding the definition of an 'emit' task
    Emit(EmitTaskDefinition),
    /// Variant holding the definition of a 'for' task
    For(ForTaskDefinition),
    /// Variant holding the definition of a 'fork' task
    Fork(ForkTaskDefinition),
    /// Variant holding the definition of a 'listen' task
    Listen(ListenTaskDefinition),
    /// Variant holding the definition of a 'raise' task
    Raise(RaiseTaskDefinition),
    /// Variant holding the definition of a 'run' task
    Run(RunTaskDefinition),
    /// Variant holding the definition of a 'set' task
    Set(SetTaskDefinition),
    /// Variant holding the definition of a 'switch' task
    Switch(SwitchTaskDefinition),
    /// Variant holding the definition of a 'try' task
    Try(TryTaskDefinition),
    /// Variant holding the definition of a 'wait' task
    Wait(WaitTaskDefinition)
}

/// A trait that all task definitions must implement
pub trait TypedTaskDefinition {
    /// Gets the task's type
    fn task_type(&self) -> &str;
}

/// Represents the definition of a task used to call a predefined function
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallTaskDefinition{

    /// Gets/sets the reference to the function to call
    #[serde(rename = "call")]
    pub call: String,

    /// Gets/sets a key/value mapping of the call's arguments, if any
    #[serde(rename = "with", skip_serializing_if = "Option::is_none")]
    pub with: Option<HashMap<String, AnyValue>>,

    /// Gets/sets a boolean indicating whether or not to wait for the called function to return. Defaults to true
    #[serde(rename = "await", skip_serializing_if = "Option::is_none")]
    pub await_: Option<bool>

}
impl TypedTaskDefinition for CallTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::CALL
    }
}
impl  CallTaskDefinition {
    pub fn new(call: &str, with: Option<HashMap<String, AnyValue>>, await_: Option<bool>) -> Self{
        Self { 
            call: call.to_string(), 
            with, 
            await_
        }
    }
}

/// Represents the configuration of a task that is composed of multiple subtasks to run sequentially
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoTaskDefinition{

    /// Gets/sets a name/definition mapping of the subtasks to perform sequentially
    #[serde(rename = "do")]
    pub do_: Map<String, TaskDefinition>

}
impl TypedTaskDefinition for DoTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::DO
    }
}
impl DoTaskDefinition {
    pub fn new(do_: Map<String, TaskDefinition>) -> Self{
        Self { 
            do_
        }
    }
}

/// Represents the configuration of a task used to emit an event
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmitTaskDefinition{

    /// Gets/sets the configuration of an event's emission
    #[serde(rename = "emit")]
    pub emit: EventEmissionDefinition

}
impl TypedTaskDefinition for EmitTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::EMIT
    }
}
impl EmitTaskDefinition {
    pub fn new(emit: EventEmissionDefinition) -> Self{
        Self { 
            emit 
        }
    }
}

/// Represents the configuration of an event's emission
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventEmissionDefinition{

    /// Gets/sets the definition of the event to emit
    #[serde(rename = "event")]
    pub event: EventDefinition

}
impl EventEmissionDefinition {
    pub fn new(event: EventDefinition) -> Self{
        Self { 
            event 
        }
    }
}

/// <summary>
/// Represents the definition of a task that executes a set of subtasks iteratively for each element in a collection
/// </summary>
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForTaskDefinition{

    /// Gets/sets the definition of the loop that iterates over a range of values
    #[serde(rename = "for")]
    pub for_: ForLoopDefinition,

    /// Gets/sets a runtime expression that represents the condition, if any, that must be met for the iteration to continue
    #[serde(rename = "while", skip_serializing_if = "Option::is_none")]
    pub while_: Option<String>,

    /// Gets/sets the tasks to perform for each item in the collection
    #[serde(rename = "do")]
    pub do_: Map<String, TaskDefinition>

}
impl TypedTaskDefinition for ForTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::FOR
    }
}
impl ForTaskDefinition {
    pub fn new(for_: ForLoopDefinition, do_: Map<String, TaskDefinition>, while_: Option<String>) -> Self{
        Self { 
            for_, 
            while_, 
            do_
        }
    }
}

/// Represents the definition of a loop that iterates over a range of values
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForLoopDefinition{

    /// Gets/sets the name of the variable that represents each element in the collection during iteration
    #[serde(rename = "emit")]
    pub each: String,

    /// Gets/sets the runtime expression used to get the collection to iterate over
    #[serde(rename = "in")]
    pub in_: String,

    /// Gets/sets the name of the variable used to hold the index of each element in the collection during iteration
    #[serde(rename = "at", skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,

    /// Gets/sets the definition of the data, if any, to pass to iterations to run
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<InputDataModelDefinition>,

}
impl ForLoopDefinition {
    pub fn new(each: &str, in_: &str, at: Option<String>, input: Option<InputDataModelDefinition>) -> Self{
        Self { 
            each: each.to_string(), 
            in_: in_.to_string(), 
            at, 
            input 
        }
    }
}

/// Represents the configuration of a task that is composed of multiple subtasks to run concurrently
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForkTaskDefinition{

    /// Gets/sets the configuration of the branches to perform concurrently
    #[serde(rename = "fork")]
    pub fork: BranchingDefinition

}
impl TypedTaskDefinition for ForkTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::FORK
    }
}
impl ForkTaskDefinition {
    pub fn new(fork: BranchingDefinition) -> Self{
        Self { 
            fork
         }
    }
}

/// Represents an object used to configure branches to perform concurrently
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchingDefinition{

    /// Gets/sets a name/definition mapping of the subtasks to perform concurrently
    #[serde(rename = "branches")]
    pub branches: Map<String, TaskDefinition>,

    /// Gets/sets a boolean indicating whether or not the branches should compete each other. If `true` and if a branch completes, it will cancel all other branches then it will return its output as the task's output
    #[serde(rename = "compete")]
    pub compete: bool

}
impl BranchingDefinition{
    pub fn new(branches:Map<String, TaskDefinition>, compete: bool) -> Self{
        Self { 
            branches, 
            compete 
        }
    }
}

/// Represents the configuration of a task used to listen to specific events
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListenTaskDefinition{

    /// Gets/sets the configuration of the listener to use
    #[serde(rename = "listen")]
    pub listen: ListenerDefinition

}
impl TypedTaskDefinition for ListenTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::LISTEN
    }
}
impl ListenTaskDefinition {
    pub fn new(listen: ListenerDefinition) -> Self{
        Self { 
            listen
        }
    }
}

/// Represents the configuration of an event listener
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListenerDefinition{

    /// Gets/sets the listener's target
    #[serde(rename = "to")]
    pub to_: EventConsumptionStrategyDefinition

}
impl ListenerDefinition {
    pub fn new(to_: EventConsumptionStrategyDefinition) -> Self{
        Self{
            to_
        }
    }
}

/// Represents the configuration of a task used to listen to specific events
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaiseTaskDefinition{

    /// Gets/sets the definition of the error to raise
    #[serde(rename = "raise")]
    pub raise: RaiseErrorDefinition

}
impl TypedTaskDefinition for RaiseTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::RAISE
    }
}
impl RaiseTaskDefinition {
    pub fn new(raise: RaiseErrorDefinition) -> Self{
        Self{
            raise
        }
    }
}

/// Represents the definition of the error to raise
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaiseErrorDefinition{

    /// Gets/sets the error to raise
    #[serde(rename = "error")]
    pub error: OneOfErrorDefinitionOrReference

}

/// Represents the configuration of a task used to run a given process
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunTaskDefinition{

    /// Gets/sets the configuration of the process to execute
    #[serde(rename = "run")]
    pub run: ProcessTypeDefinition

}
impl TypedTaskDefinition for RunTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::RUN
    }
}
impl RunTaskDefinition {
    pub fn new(run: ProcessTypeDefinition) -> Self{
        Self { 
            run
        }
    }
}

/// Represents the configuration of a process execution
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessTypeDefinition{

    /// Gets/sets the configuration of the container to run
    #[serde(rename = "container", skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerProcessDefinition>,

    /// Gets/sets the configuration of the script to run
    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptProcessDefinition>,

    /// Gets/sets the configuration of the shell command to run
    #[serde(rename = "shell", skip_serializing_if = "Option::is_none")]
    pub shell: Option<ShellProcessDefinition>,

    /// Gets/sets the configuration of the workflow to run
    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<WorkflowProcessDefinition>,

    /// Gets/sets a boolean indicating whether or not to await the process completion before continuing. Defaults to 'true'
    #[serde(rename = "await", skip_serializing_if = "Option::is_none")]
    pub await_: Option<bool>

}
impl ProcessTypeDefinition {

    /// Creates a new container process
    pub fn using_container(container: ContainerProcessDefinition, await_: Option<bool>) -> Self{
        Self { 
            container: Some(container),
            await_,
            shell: None,
            script: None,
            workflow: None
        }
    }

    /// Creates a new script process
    pub fn using_script(script: ScriptProcessDefinition, await_: Option<bool>) -> Self{
        Self { 
            script: Some(script),
            await_,
            container: None,
            shell: None,
            workflow: None
        }
    }

    /// Creates a new shell process
    pub fn using_shell(shell: ShellProcessDefinition, await_: Option<bool>) -> Self{
        Self { 
            shell: Some(shell),
            await_,
            container: None,
            script: None,
            workflow: None
        }
    }

    /// Creates a new workflow process
    pub fn using_workflow(workflow: WorkflowProcessDefinition, await_: Option<bool>) -> Self{
        Self { 
            workflow: Some(workflow),
            await_,
            container: None,
            shell: None,
            script: None
        }
    }
    
    /// Gets the type of the defined process
    pub fn get_process_type(&self) -> &str{
        if self.container.is_some(){
            ProcessType::CONTAINER
        }
        else if self.script.is_some(){
            ProcessType::SCRIPT
        }
        else if self.shell.is_some(){
            ProcessType::SHELL
        }
        else{
            ProcessType::WORKFLOW
        }
    }

}

/// Represents the configuration of a container process
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerProcessDefinition{

    /// Gets/sets the name of the container image to run
    #[serde(rename = "image")]
    pub image: String,

    /// Gets/sets the command, if any, to execute on the container
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// Gets/sets a list containing the container's port mappings, if any
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<HashMap<u16, u16>>,

    /// Gets/sets the volume mapping for the container, if any
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<String, String>>,

    /// Gets/sets a key/value mapping of the environment variables, if any, to use when running the configured process
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<HashMap<String, String>>,

}
impl ContainerProcessDefinition {
    pub fn new(image: &str, command: Option<String>, ports: Option<HashMap<u16, u16>>, volumes: Option<HashMap<String, String>>, environment: Option<HashMap<String, String>>) -> Self{
        Self { 
            image: image.to_string(), 
            command, 
            ports, 
            volumes, 
            environment
        }
    }
}

/// Represents the definition of a script evaluation process
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptProcessDefinition{
    
    /// Gets/sets the language of the script to run
    #[serde(rename = "language")]
    pub language: String,

    /// Gets/sets the script's code. Required if 'source' has not been set
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Gets the the script's source. Required if 'code' has not been set
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<ExternalResourceDefinition>,

    /// Gets/sets a key/value mapping of the arguments, if any, to pass to the script to run
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,

    /// Gets/sets a key/value mapping of the environment variables, if any, to use when running the configured process
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<HashMap<String, String>>,

}
impl ScriptProcessDefinition {

    /// Initializes a new script from code
    pub fn from_code(language: &str, code: String, arguments: Option<Vec<String>>, environment: Option<HashMap<String, String>>) -> Self{
        Self { 
            language: language.to_string(), 
            code: Some(code), 
            source: None, 
            arguments, 
            environment
         }
    }

    /// Initializes a new script from an external resource
    pub fn from_source(language: &str, source: ExternalResourceDefinition, arguments: Option<Vec<String>>, environment: Option<HashMap<String, String>>) -> Self{
        Self { 
            language: language.to_string(), 
            code: None, 
            source: Some(source), 
            arguments, 
            environment
         }
    }
}

/// Represents the definition of a shell process
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShellProcessDefinition{
    
    /// Gets/sets the shell command to run
    #[serde(rename = "command")]
    pub command: String,

    /// Gets/sets the arguments of the shell command to run
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,

    /// Gets/sets a key/value mapping of the environment variables, if any, to use when running the configured process
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<HashMap<String, String>>,

}
impl ShellProcessDefinition {
    pub fn new(command: &str, arguments: Option<Vec<String>>, environment: Option<HashMap<String, String>>) -> Self{
        Self { 
            command: command.to_string(), 
            arguments, 
            environment
        }
    }
}

/// Represents the definition of a (sub)workflow process
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowProcessDefinition{
    
    /// Gets/sets the namespace the workflow to run belongs to
    #[serde(rename = "namespace")]
    pub namespace: String,

    /// Gets/sets the name of the workflow to run
    #[serde(rename = "name")]
    pub name: String,

    /// Gets/sets the version of the workflow to run
    #[serde(rename = "version")]
    pub version: String,

    /// Gets/sets the data, if any, to pass as input to the workflow to execute. The value should be validated against the target workflow's input schema, if specified
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<AnyValue>

}
impl WorkflowProcessDefinition {
    pub fn new(namespace: &str, name: &str, version: &str, input: Option<AnyValue>) -> Self{
        Self { 
            namespace: namespace.to_string(), 
            name: name.to_string(), 
            version: version.to_string(), 
            input
        }
    }
}

/// Represents the definition of a task used to set data
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetTaskDefinition{

    /// Gets/sets the data to set
    #[serde(rename = "set")]
    pub set: HashMap<String, AnyValue>

}
impl TypedTaskDefinition for SetTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::SET
    }
}
impl SetTaskDefinition {
    pub fn new(set: HashMap<String, AnyValue>) -> Self{
        Self { 
            set
        }
    }
}

/// Represents the definition of a task that evaluates conditions and executes specific branches based on the result
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchTaskDefinition{

    /// Gets/sets the definition of the switch to use
    #[serde(rename = "switch")]
    pub switch: Map<String, SwitchCaseDefinition>

}
impl TypedTaskDefinition for SwitchTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::SWITCH
    }
}
impl SwitchTaskDefinition {
    pub fn new(switch: Map<String, SwitchCaseDefinition>) -> Self{
        Self { 
            switch
        }
    }
}

/// Represents the definition of a case within a switch task, defining a condition and corresponding tasks to execute if the condition is met
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchCaseDefinition{

    /// Gets/sets the condition that determines whether or not the case should be executed in a switch task
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,

    /// Gets/sets the transition to perform when the case matches
    #[serde(rename = "then", skip_serializing_if = "Option::is_none")]
    pub then: Option<String>

}

/// Represents the definition of a task used to try one or more subtasks, and to catch/handle the errors that can potentially be raised during execution
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TryTaskDefinition{

    /// Gets/sets a name/definition map of the tasks to try running
    #[serde(rename = "try")]
    pub try_: Map<String, TaskDefinition>,

    /// Gets/sets the object used to define the errors to catch
    #[serde(rename = "catch")]
    pub catch: ErrorCatcherDefinition

}
impl TypedTaskDefinition for TryTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::TRY
    }
}
impl TryTaskDefinition {
    pub fn new(try_: Map<String, TaskDefinition>, catch: ErrorCatcherDefinition) -> Self{
        Self { 
            try_,
            catch
        }
    }
}

/// Represents the configuration of a concept used to catch errors
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorCatcherDefinition{

    /// Gets/sets the definition of the errors to catch
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<ErrorFilterDefinition>,

    /// Gets/sets the name of the runtime expression variable to save the error as. Defaults to 'error'.
    #[serde(rename = "as", skip_serializing_if = "Option::is_none")]
    pub as_: Option<String>,

    /// Gets/sets a runtime expression used to determine whether or not to catch the filtered error
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,

    /// Gets/sets a runtime expression used to determine whether or not to catch the filtered error
    #[serde(rename = "exceptWhen", skip_serializing_if = "Option::is_none")]
    pub except_when: Option<String>,

    /// Gets/sets the retry policy to use, if any
    #[serde(rename = "retry", skip_serializing_if = "Option::is_none")]
    pub retry: Option<OneOfRetryPolicyDefinitionOrReference>,

    /// Gets/sets a name/definition map of the tasks, if any, to run when catching an error
    #[serde(rename = "do", skip_serializing_if = "Option::is_none")]
    pub do_: Option<Map<String, TaskDefinition>>

}

/// Represents the definition an an error filter
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorFilterDefinition{

    /// Gets/sets a key/value mapping of the properties errors to filter must define
    #[serde(rename = "with", skip_serializing_if = "Option::is_none")]
    pub with: Option<HashMap<String, AnyValue>>

}

/// Represents the definition of a task used to wait a certain amount of time
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaitTaskDefinition{

    /// Gets/sets the amount of time to wait before resuming workflow
    #[serde(rename = "duration")]
    pub duration: Duration

}
impl TypedTaskDefinition for WaitTaskDefinition {
    fn task_type(&self) -> &str {
        TaskType::WAIT
    }
}
impl WaitTaskDefinition {
    pub fn new(duration: Duration) -> Self{
        Self { 
            duration
        }
    }
}