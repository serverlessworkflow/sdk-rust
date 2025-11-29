use crate::services::authentication::*;
use crate::services::timeout::*;
use serde_json::Value;
use serverless_workflow_core::models::duration::*;
use serverless_workflow_core::models::error::*;
use serverless_workflow_core::models::event::*;
use serverless_workflow_core::models::input::*;
use serverless_workflow_core::models::map::*;
use serverless_workflow_core::models::output::*;
use serverless_workflow_core::models::resource::*;
use serverless_workflow_core::models::retry::*;
use serverless_workflow_core::models::schema::*;
use serverless_workflow_core::models::task::*;
use serverless_workflow_core::models::timeout::*;
use std::collections::HashMap;

/// Represents the service used to build TaskDefinitions
pub struct GenericTaskDefinitionBuilder{
    builder: Option<TaskDefinitionBuilder>
}
impl GenericTaskDefinitionBuilder{

    /// Initializes a new GenericTaskDefinitionBuilder
    pub fn new() -> Self{
        Self{
            builder: None
        }
    }

     /// Configures the task to call the specified function
    pub fn call(&mut self, function: &str) -> &mut CalltaskDefinitionBuilder{
        let builder = CalltaskDefinitionBuilder::new(function);
        self.builder = Some(TaskDefinitionBuilder::Call(builder));
        if let Some(TaskDefinitionBuilder::Call(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Call");
        }
    }

    /// Configures the task to perform subtasks sequentially
    pub fn do_(&mut self) -> &mut DoTaskDefinitionBuilder{
        let builder = DoTaskDefinitionBuilder::new();
        self.builder = Some(TaskDefinitionBuilder::Do(builder));
        if let Some(TaskDefinitionBuilder::Do(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Do");
        }
    }

    /// Configures the task to perform subtasks sequentially
    pub fn emit<F>(&mut self, setup: F) -> &mut EmitTaskDefinitionBuilder
    where F: FnOnce(&mut EventDefinitionBuilder){
        let mut event_builder: EventDefinitionBuilder = EventDefinitionBuilder::new();
        setup(&mut event_builder);
        let event = event_builder.build();
        let builder = EmitTaskDefinitionBuilder::new(event);
        self.builder = Some(TaskDefinitionBuilder::Emit(builder));
        if let Some(TaskDefinitionBuilder::Emit(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Emit");
        }
    }

    /// Configures the task to iterate over a collection and perform a task for each of the items it contains
    pub fn for_(&mut self) -> &mut ForTaskDefinitionBuilder{
        let builder = ForTaskDefinitionBuilder::new();
        self.builder = Some(TaskDefinitionBuilder::For(builder));
        if let Some(TaskDefinitionBuilder::For(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to For");
        }
    }

    /// Configures the task to execute branches concurrently
    pub fn fork(&mut self) -> &mut ForkTaskDefinitionBuilder{
        let builder = ForkTaskDefinitionBuilder::new();
        self.builder = Some(TaskDefinitionBuilder::Fork(builder));
        if let Some(TaskDefinitionBuilder::Fork(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Fork");
        }
    }

    /// Configures the task to listen for events
    pub fn listen(&mut self) -> &mut ListenTaskDefinitionBuilder{
        let builder = ListenTaskDefinitionBuilder::new();
        self.builder = Some(TaskDefinitionBuilder::Listen(builder));
        if let Some(TaskDefinitionBuilder::Listen(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Listen");
        }
    }

    /// Configures the task to raise the specified error
    pub fn raise(&mut self) -> &mut RaiseTaskDefinitionBuilder{
        let builder = RaiseTaskDefinitionBuilder::new();
        self.builder = Some(TaskDefinitionBuilder::Raise(builder));
        if let Some(TaskDefinitionBuilder::Raise(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Raise");
        }
    }

    /// Configures the task to run a process
    pub fn run(&mut self) -> &mut RunTaskDefinitionBuilder{
        let builder = RunTaskDefinitionBuilder::new();
        self.builder = Some(TaskDefinitionBuilder::Run(builder));
        if let Some(TaskDefinitionBuilder::Run(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Run");
        }
    }

    /// Configures the task to set variables
    pub fn set(&mut self) -> &mut SetTaskDefinitionBuilder{
        let builder = SetTaskDefinitionBuilder::new();
        self.builder = Some(TaskDefinitionBuilder::Set(builder));
        if let Some(TaskDefinitionBuilder::Set(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Set");
        }
    }

    /// Configures the task to branch the flow based on defined conditions
    pub fn switch(&mut self) -> &mut SwitchTaskDefinitionBuilder{
        let builder = SwitchTaskDefinitionBuilder::new();
        self.builder = Some(TaskDefinitionBuilder::Switch(builder));
        if let Some(TaskDefinitionBuilder::Switch(ref mut builder)) = self.builder{
            builder
        }
        else{
            unreachable!("Builder should always be set to Switch");
        }
    }

    /// Configures the task to try executing a specific task, and handle potential errors
    pub fn try_(&mut self) -> &mut TryTaskDefinitionBuilder{
        let builder = TryTaskDefinitionBuilder::new();
        self.builder = Some(TaskDefinitionBuilder::Try(builder));
        if let Some(TaskDefinitionBuilder::Try(ref mut builder)) = self.builder{
            builder
        }
        else{
            unreachable!("Builder should always be set to Try");
        }
    }

    /// Configures the task to wait a defined amount of time
    pub fn wait(&mut self, duration: OneOfDurationOrIso8601Expression) -> &mut WaitTaskDefinitionBuilder{
        let builder = WaitTaskDefinitionBuilder::new(duration);
        self.builder = Some(TaskDefinitionBuilder::Wait(builder));
        if let Some(TaskDefinitionBuilder::Wait(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Wait");
        }
    }

    /// Builds the configured task
    pub fn build(self) -> TaskDefinition{
        if let Some(builder) = self.builder {
            match builder {
                TaskDefinitionBuilder::Call(builder) => builder.build(),
                TaskDefinitionBuilder::Do(builder) => builder.build(),
                TaskDefinitionBuilder::Emit(builder) => builder.build(),
                TaskDefinitionBuilder::For(builder) => builder.build(),
                TaskDefinitionBuilder::Fork(builder) => builder.build(),
                TaskDefinitionBuilder::Listen(builder) => builder.build(),
                TaskDefinitionBuilder::Raise(builder) => builder.build(),
                TaskDefinitionBuilder::Run(builder) => builder.build(),
                TaskDefinitionBuilder::Set(builder) => builder.build(),
                TaskDefinitionBuilder::Switch(builder) => builder.build(),
                TaskDefinitionBuilder::Try(builder) => builder.build(),
                TaskDefinitionBuilder::Wait(builder) => builder.build()
            } 
        } 
        else {
            panic!("The task must be configured");
        }
    }

}

/// Enumerates all supported task definition builders
pub enum TaskDefinitionBuilder{
    Call(CalltaskDefinitionBuilder),
    Do(DoTaskDefinitionBuilder),
    Emit(EmitTaskDefinitionBuilder),
    For(ForTaskDefinitionBuilder),
    Fork(ForkTaskDefinitionBuilder),
    Listen(ListenTaskDefinitionBuilder),
    Raise(RaiseTaskDefinitionBuilder),
    Run(RunTaskDefinitionBuilder),
    Set(SetTaskDefinitionBuilder),
    Switch(SwitchTaskDefinitionBuilder),
    Try(TryTaskDefinitionBuilder),
    Wait(WaitTaskDefinitionBuilder)
}

/// Defines functionnality common to all TaskDefinitionBuilder implementations
pub trait TaskDefinitionBuilderBase {
    
    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self;

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self;

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder);

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder);

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder);

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder);

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self;

    /// Builds the configured TaskDefinition
    fn build(self) -> TaskDefinition;

}

/// Represents the service used to build CallTaskDefinitions
pub struct CalltaskDefinitionBuilder{
    task: CallTaskDefinition
}
impl CalltaskDefinitionBuilder {
    
    /// Initializes a new CallTaskDefinitionBuilder
    pub fn new(function: &str) -> Self{
        Self { task: CallTaskDefinition::new(function, None, None) }
    }

    /// Adds a new argument to call the function with
    pub fn with(&mut self, name: &str, value: Value) -> &mut Self{
        if self.task.with.is_none(){
            self.task.with = Some(HashMap::new());
        }
        if let Some(with) = &mut self.task.with {
            with.insert(name.to_string(), value);
        }
        self
    }

    /// Sets the arguments to call the function with
    pub fn with_arguments(&mut self, arguments: HashMap<String, Value>) -> &mut Self{
        self.task.with = Some(arguments);
        self
    }
  
}
impl TaskDefinitionBuilderBase for CalltaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.task.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.task.common.then = Some(directive.to_string());
        self
    }

    /// Builds the configures CallTaskDefinition
    fn build(self) -> TaskDefinition{
        TaskDefinition::Call(self.task)
    }

}

/// Represents the service used to build DoTaskDefinitions
pub struct DoTaskDefinitionBuilder{
    task: DoTaskDefinition
}
impl DoTaskDefinitionBuilder {
    
    /// Initializes a new DoTaskDefinitionBuilder
    pub fn new() -> Self{
        Self { task: DoTaskDefinition::default() }
    }

    /// Adds a new task with the specified name to the task
    pub fn do_<F>(&mut self, name: &str, setup: F) -> &mut Self
    where F: FnOnce(&mut GenericTaskDefinitionBuilder){
        let mut builder = GenericTaskDefinitionBuilder::new();
        setup(&mut builder);
        let task = builder.build();
        self.task.do_.add(name.to_string(), task);
        self
    }

}
impl TaskDefinitionBuilderBase for DoTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.task.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.export = Some(builder.build());
        self
    }
    
    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.task.common.then = Some(directive.to_string());
        self
    }

    /// Builds the configures DoTaskDefinition
    fn build(self) -> TaskDefinition{
        TaskDefinition::Do(self.task)
    }

}

/// Represents the service used to build EmitTaskDefinitions
pub struct EmitTaskDefinitionBuilder{
    task: EmitTaskDefinition
}
impl EmitTaskDefinitionBuilder {
    
    /// Initializes a new EmitTaskDefinitionBuilder
    pub fn new(event: EventDefinition) -> Self{
        Self { task: EmitTaskDefinition::new(EventEmissionDefinition::new(event)) }
    }

}
impl TaskDefinitionBuilderBase for EmitTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.task.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.task.common.then = Some(directive.to_string());
        self
    }

    /// Builds the configures DoTaskDefinition
    fn build(self) -> TaskDefinition{
        TaskDefinition::Emit(self.task)
    }

}

/// Represents the service used to build ForTaskDefinitions
pub struct ForTaskDefinitionBuilder{
    task: ForTaskDefinition
}
impl ForTaskDefinitionBuilder{

    /// Initializes a new ForTaskDefinitionBuilder
    pub fn new() -> Self{
        Self { task:ForTaskDefinition::default() }
    }

    /// Sets the name of the variable to store the iteration item to
    pub fn each(&mut self, variable_name: &str) -> &mut Self{
        self.task.for_.each = variable_name.to_string();
        self
    }

    /// Sets the runtime expression used to resolve the collection to iterate
    pub fn in_(&mut self, expression: &str) -> &mut Self{
        self.task.for_.in_ = expression.to_string();
        self
    }

    /// Sets the name of the variable to store the iteration index to
    pub fn at(&mut self, variable_name: &str) -> &mut Self{
        self.task.for_.at = Some(variable_name.to_string());
        self
    }

    /// Configures the task to execute the specified tasks for each item in the specified collection
    pub fn do_<F>(&mut self, name: &str, setup: F) -> &mut Self
    where F: FnOnce(&mut GenericTaskDefinitionBuilder){
        let mut builder = GenericTaskDefinitionBuilder::new();
        setup(&mut builder);
        let task = builder.build();
        self.task.do_.add(name.to_string(), task);
        self
    }
     
}
impl TaskDefinitionBuilderBase for ForTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.task.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.task.common.then = Some(directive.to_string());
        self
    }

    /// Builds the configured ForTaskDefinition
    fn build(self) -> TaskDefinition{
        TaskDefinition::For(self.task)
    }

}

/// Represents the service used to build ForkTaskDefinitions
pub struct ForkTaskDefinitionBuilder{
    task: ForkTaskDefinition
}
impl ForkTaskDefinitionBuilder{

    /// Initializes a new ForkTaskDefinitions
    pub fn new() -> Self{
        Self { task:ForkTaskDefinition::default() }
    }
    
    /// Configures the tasks to perform concurrently
    pub fn branch<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TaskDefinitionMapBuilder){
        let mut builder = TaskDefinitionMapBuilder::new();
        setup(&mut builder);
        let branches = builder.build();
        self.task.fork.branches = branches;
        self
    }

}
impl TaskDefinitionBuilderBase for ForkTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.task.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.task.common.then = Some(directive.to_string());
        self
    }

    /// Builds the configured ForkTaskDefinition
    fn build(self) -> TaskDefinition{
        TaskDefinition::Fork(self.task)
    }

}

/// Represents the service used to build ListenTaskDefinitions
pub struct ListenTaskDefinitionBuilder{
    task: ListenTaskDefinition
}
impl ListenTaskDefinitionBuilder{

    /// Initializes a new ListenTaskDefinitionBuilder
    pub fn new() -> Self{
        Self { task:ListenTaskDefinition::default() }
    }

    /// Configures the task to listen to the specified event(s)
    pub fn to<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut EventConsumptionStrategyDefinitionBuilder){
        let mut builder = EventConsumptionStrategyDefinitionBuilder::new();
        setup(&mut builder);
        let consumption_strategy = builder.build();
        self.task.listen.to = consumption_strategy;
        self
    }

    /// Configures the iterator used to process the consumed events
    pub fn foreach<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut SubscriptionIteratorDefinitionBuilder){
        let mut builder = SubscriptionIteratorDefinitionBuilder::new();
        setup(&mut builder);
        self.task.foreach = Some(builder.build());
        self
    }

}
impl TaskDefinitionBuilderBase for ListenTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.task.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.task.common.then = Some(directive.to_string());
        self
    }

    /// Builds the configured ListenTaskDefinition
    fn build(self) -> TaskDefinition{
        TaskDefinition::Listen(self.task)
    }

}

/// Represents the service used to build RaiseTaskDefinitions
pub struct RaiseTaskDefinitionBuilder{
    common: TaskDefinitionFields,
    builder: Option<ErrorDefinitionBuilder>,
    reference: Option<String>
}
impl RaiseTaskDefinitionBuilder{

    /// Initializes a new RaiseTaskDefinitionBuilder
    pub fn new() -> Self{
        Self { common: TaskDefinitionFields::new(), builder: None, reference: None }
    }

    /// Sets the error to raise
    pub fn error(&mut self) -> &mut ErrorDefinitionBuilder{
        let builder = ErrorDefinitionBuilder::new();
        self.builder = Some(builder);
        if let Some(ref mut error_builder) = self.builder{
            error_builder
        }
        else {
            unreachable!("Builder should always be set to Error");
        }
    }

    /// Sets a reference to the error to raise
    pub fn referenced_error(&mut self, reference: &str){
        self.reference = Some(reference.to_string());
    }

}
impl TaskDefinitionBuilderBase for RaiseTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.common.then = Some(directive.to_string());
        self
    }

    /// Builds the configured RaiseTaskDefinition
    fn build(self) -> TaskDefinition{
        let mut task = RaiseTaskDefinition::default();
        if let Some(builder) = self.builder {
            let error = builder.build();
            task.raise = RaiseErrorDefinition::new(OneOfErrorDefinitionOrReference::Error(error));
        }
        else if let Some(reference) = self.reference{
            task.raise = RaiseErrorDefinition::new(OneOfErrorDefinitionOrReference::Reference(reference));
        }
        else{
            panic!("The error to raise must be configured");
        }
        task.common = self.common;
        TaskDefinition::Raise(task)
    }

}

/// Represents the service used to build RunTaskDefinitions
pub struct RunTaskDefinitionBuilder{
    common: TaskDefinitionFields,
    builder : Option<ProcessDefinitionBuilder>
}
impl RunTaskDefinitionBuilder{

    /// Initializes a new RunTaskDefinitionBuilder
    pub fn new() -> Self{
        Self{ common: TaskDefinitionFields::new(), builder: None }
    }

    /// Configures the task to run the specified container
    pub fn container(&mut self) -> &mut ContainerProcessDefinitionBuilder{
        self.builder = Some(ProcessDefinitionBuilder::Container(ContainerProcessDefinitionBuilder::new()));
        if let Some(ProcessDefinitionBuilder::Container(ref mut container_builder)) = self.builder {
            container_builder
        }
        else {
            unreachable!("Builder should always be set to Container");
        }
    }

    /// Configures the task to run the specified script
    pub fn script(&mut self) -> &mut ScriptProcessDefinitionBuilder{
        self.builder = Some(ProcessDefinitionBuilder::Script(ScriptProcessDefinitionBuilder::new()));
        if let Some(ProcessDefinitionBuilder::Script(ref mut script_builder)) = self.builder {
            script_builder
        }
        else {
            unreachable!("Builder should always be set to Script");
        }
    }

    /// Configures the task to run the specified shell command
    pub fn shell(&mut self) -> &mut ShellProcessDefinitionBuilder{
        self.builder = Some(ProcessDefinitionBuilder::Shell(ShellProcessDefinitionBuilder::new()));
        if let Some(ProcessDefinitionBuilder::Shell(ref mut shell_builder)) = self.builder {
            shell_builder
        }
        else {
            unreachable!("Builder should always be set to Shell");
        }
    }

    /// Configures the task to run the specified workflow
    pub fn workflow(&mut self) -> &mut WorkflowProcessDefinitionBuilder{
        self.builder = Some(ProcessDefinitionBuilder::Workflow(WorkflowProcessDefinitionBuilder::new()));
        if let Some(ProcessDefinitionBuilder::Workflow(ref mut workflow_builder)) = self.builder {
            workflow_builder
        }
        else {
            unreachable!("Builder should always be set to Workflow");
        }
    }

}
impl TaskDefinitionBuilderBase for RunTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.common.then = Some(directive.to_string());
        self
    }

    /// Builds the configured RunTaskDefinition
    fn build(self) -> TaskDefinition{
        if let Some(builder) = self.builder {
            let mut process = match builder {
                ProcessDefinitionBuilder::Container(builder) => builder.build(),
                ProcessDefinitionBuilder::Script(builder) => builder.build(),
                ProcessDefinitionBuilder::Shell(builder) => builder.build(),
                ProcessDefinitionBuilder::Workflow(builder) => builder.build()
            };
            process.common = self.common;
            TaskDefinition::Run(process)
        }
        else{
            panic!("The process to run must be configured");
        }
    } 

}

/// Represents the service used to build SetTaskDefinitions
pub struct SetTaskDefinitionBuilder{
    task: SetTaskDefinition
}
impl SetTaskDefinitionBuilder{

    /// Initializes a new SetTaskDefinition
    pub fn new() -> Self{
        Self { task: SetTaskDefinition::new() }
    }

    /// Sets the specified variable
    pub fn variable(&mut self, name: &str, value: Value) -> &mut Self{
        match &mut self.task.set {
            serverless_workflow_core::models::task::SetValue::Map(map) => {
                map.insert(name.to_string(), value);
            }
            serverless_workflow_core::models::task::SetValue::Expression(_) => {
                // If it was an expression, convert to map
                let mut map = HashMap::new();
                map.insert(name.to_string(), value);
                self.task.set = serverless_workflow_core::models::task::SetValue::Map(map);
            }
        }
        self
    }

    /// Configures the variable as an expression
    pub fn variable_expression(&mut self, expression: String) -> &mut Self{
        self.task.set = serverless_workflow_core::models::task::SetValue::Expression(expression);
        self
    }

    /// Configures the task to set the specified variables
    pub fn variables(&mut self, variables: HashMap<String, Value>) -> &mut Self{
        self.task.set = serverless_workflow_core::models::task::SetValue::Map(variables);
        self
    }

}
impl TaskDefinitionBuilderBase for SetTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.task.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.task.common.then = Some(directive.to_string());
        self
    }

    /// Builds a new SetTaskDefinition
    fn build(self) -> TaskDefinition{
        TaskDefinition::Set(self.task)
    }
    
}

/// Represents the service used to build SwitchTaskDefinitions
pub struct SwitchTaskDefinitionBuilder{
    task: SwitchTaskDefinition
}
impl SwitchTaskDefinitionBuilder{

    /// Initializes a new SwitchTaskDefinition
    pub fn new() -> Self{
        Self { task: SwitchTaskDefinition::new() }
    }

    /// Adds a new casev
    pub fn case<F>(&mut self, name: &str, setup: F) -> &mut Self
    where F: FnOnce(&mut SwitchCaseDefinitionBuilder){
        let mut builder = SwitchCaseDefinitionBuilder::new();
        setup(&mut builder);
        let case = builder.build();
        self.task.switch.add(name.to_string(), case);
        self
    }

}
impl TaskDefinitionBuilderBase for SwitchTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.task.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.task.common.then = Some(directive.to_string());
        self
    }

    /// Builds a new SwitchTaskDefinition
    fn build(self) -> TaskDefinition{
        TaskDefinition::Switch(self.task)
    }

}

/// Represents the service used to build TryTaskDefinitions
pub struct TryTaskDefinitionBuilder{
    task: TryTaskDefinition
}
impl TryTaskDefinitionBuilder{

    /// Initializes a new TryTaskDefinition
    pub fn new() -> Self{
        Self { task: TryTaskDefinition::default() }
    }

    /// Configures the task to try executing the specified tasks 
    pub fn do_<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TaskDefinitionMapBuilder){
        let mut builder = TaskDefinitionMapBuilder::new();
        setup(&mut builder);
        self.task.try_ = builder.build();
        self
    }

    /// Configures the task to catch defined errors
    pub fn catch<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut ErrorCatcherDefinitionBuilder){
        let mut builder = ErrorCatcherDefinitionBuilder::new();
        setup(&mut builder);
        self.task.catch = builder.build();
        self
    }

}
impl TaskDefinitionBuilderBase for TryTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.task.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.task.common.then = Some(directive.to_string());
        self
    }

    /// Builds a new TryTaskDefinition
    fn build(self) -> TaskDefinition{
        TaskDefinition::Try(self.task)
    }

}

/// Represents the service used to build WaitTaskDefinitions
pub struct WaitTaskDefinitionBuilder{
    task: WaitTaskDefinition
}
impl WaitTaskDefinitionBuilder {
    
    /// Initializes a new WaitTaskDefinitionBuilder
    pub fn new(duration: OneOfDurationOrIso8601Expression) -> Self{
        Self { task: WaitTaskDefinition::new(duration) }
    }

}
impl TaskDefinitionBuilderBase for WaitTaskDefinitionBuilder{

    /// Configures the task to build to run only if the specified condition matches
    fn if_(&mut self, condition: &str) -> &mut Self{
        self.task.common.if_ = Some(condition.to_string());
        self
    }

    /// Sets the task's timeout
    fn with_timeout_reference(&mut self, reference: &str) -> &mut Self{
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Sets the task's timeout
    fn with_timeout<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TimeoutDefinitionBuilder){
        let mut builder = TimeoutDefinitionBuilder::new();
        setup(&mut builder);
        let timeout = builder.build();
        self.task.common.timeout = Some(OneOfTimeoutDefinitionOrReference::Timeout(timeout));
        self
    }

    /// Configures the task's input
    fn with_input<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut InputDataModelDefinitionBuilder){
        let mut builder = InputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.input = Some(builder.build());
        self
    }

    /// Configures the task's output
    fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.output = Some(builder.build());
        self
    }

    /// Configures the task's export
    fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.task.common.export = Some(builder.build());
        self
    }

    /// Configures the task to build to then execute the specified flow directive
    fn then(&mut self, directive: &str) -> &mut Self{
        self.task.common.then = Some(directive.to_string());
        self
    }
    
    /// Builds the configures DoTaskDefinition
    fn build(self) -> TaskDefinition{
        TaskDefinition::Wait(self.task)
    }

}

/// Represents the service used to build EventDefinitions
pub struct EventDefinitionBuilder{
    event: EventDefinition
}
impl EventDefinitionBuilder{

    /// Initializes a new EventDefinitionBuilder
    pub fn new() -> Self{
        Self { event: EventDefinition::default() }
    }

    /// Adds a new attribute to the event
    pub fn with(&mut self, name: &str, value: Value) -> &mut Self{
        self.event.with.insert(name.to_string(), value);
        self
    }

    /// Sets the event's attributes
     pub fn with_attributes(&mut self, attributes: HashMap<String, Value>) -> &mut Self{
        self.event.with = attributes;
        self
    }

    /// Builds the configured EventDefinition
    pub fn build(self) -> EventDefinition{
        self.event
    }

}

/// Represents the service used to build TaskDefinitionMaps
pub struct TaskDefinitionMapBuilder{
    map: Map<String, TaskDefinition>
}
impl TaskDefinitionMapBuilder{

    /// Initializes a new TaskDefinitionMapBuilder
    pub fn new() -> Self{
        Self { map: Map::new() }
    }

    /// Adds a new task with the specified name to the task
    pub fn do_<F>(&mut self, name: &str, setup: F) -> &mut Self
    where F: FnOnce(&mut GenericTaskDefinitionBuilder){
        let mut builder = GenericTaskDefinitionBuilder::new();
        setup(&mut builder);
        let task = builder.build();
        self.map.add(name.to_string(), task);
        self
    }
     
    /// Builds the configured TaskDefinitionMap
    pub fn build(self) -> Map<String, TaskDefinition>{
        self.map
    }

}

/// Represents the service used to build EventConsumptionStrategyDefinition
pub struct EventConsumptionStrategyDefinitionBuilder{
    all: Option<EventFilterDefinitionCollectionBuilder>,
    any: Option<EventFilterDefinitionCollectionBuilder>,
    one: Option<EventFilterDefinitionBuilder>,
    until_condition: Option<String>,
    until_events: Option<EventConsumptionStrategyDefinition>
}
impl EventConsumptionStrategyDefinitionBuilder{

    /// Initializes a new EventConsumptionStrategyDefinitionBuilder
    pub fn new() -> Self{
        Self { all: None, any: None, one: None, until_condition: None, until_events: None }
    }

    /// Configures the task to listen for all of the defined events
    pub fn all(&mut self) -> &mut EventFilterDefinitionCollectionBuilder {
        let builder = EventFilterDefinitionCollectionBuilder::new();
        self.all = Some(builder);
        if let Some(ref mut all_builder) = self.all{
            all_builder
        }
        else {
            unreachable!("Builder should always be set to All");
        }
    }

    /// Configures the task to listen for any of the defined events
    pub fn any(&mut self) -> &mut EventFilterDefinitionCollectionBuilder {
        let builder = EventFilterDefinitionCollectionBuilder::new();
        self.any = Some(builder);
        if let Some(ref mut any_builder) = self.any{
            any_builder
        }
        else {
            unreachable!("Builder should always be set to Any");
        }
    }

    /// Configures the task to listen for one single event
    pub fn one(&mut self) -> &mut EventFilterDefinitionBuilder {
        let builder = EventFilterDefinitionBuilder::new();
        self.one = Some(builder);
        if let Some(ref mut one_builder) = self.one{
            one_builder
        }
        else {
            unreachable!("Builder should always be set to One");
        }
    }

    /// Configures the task to listen to any events until the specified events are consumed
    pub fn until<F>(&mut self, setup: F)
    where F: FnOnce(&mut EventConsumptionStrategyDefinitionBuilder){
        let mut builder = EventConsumptionStrategyDefinitionBuilder::new();
        setup(&mut builder);
        self.until_events = Some(builder.build());
    }

    /// Configures the task to listen to any events until the specified condition matches
    pub fn until_condition_matches(&mut self, expression: &str){
        self.until_condition = Some(expression.to_string());
    }

    /// Builds the configured EventConsumptionStrategyDefinition
    pub fn build(self) -> EventConsumptionStrategyDefinition{
        let mut strategy = EventConsumptionStrategyDefinition::default();
        if let Some(all_builder) = self.all {
            strategy.all = Some(all_builder.build());
        }
        else if let Some(any_builder) = self.any {
            strategy.any = Some(any_builder.build());
            if let Some(expression) = self.until_condition{
                strategy.until = Some(Box::new(OneOfEventConsumptionStrategyDefinitionOrExpression::Expression(expression.to_string())));
            }
            else if let Some(until_events) = self.until_events{
                strategy.until = Some(Box::new(OneOfEventConsumptionStrategyDefinitionOrExpression::Strategy(until_events)));
            }
        }
        else if let Some(one_builder) = self.one {
            strategy.one = Some(one_builder.build());
        }
        else{ 
            panic!("No strategy defined!");
        }
        strategy
    }

}

/// Represents the service used to build EventFilterDefinitionCollections
pub struct EventFilterDefinitionCollectionBuilder{
    events: Vec<EventFilterDefinition>
}
impl EventFilterDefinitionCollectionBuilder{

    /// Initializes a new EventFilterDefinitionCollectionBuilder
    pub fn new() -> Self{
        Self { events: Vec::new() }
    }

    /// Adds the specified event filter to the collection
    pub fn event<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut EventFilterDefinitionBuilder){
        let mut builder = EventFilterDefinitionBuilder::new();
        setup(&mut builder);
        let filter = builder.build();
        self.events.push(filter);
        self
    }

    /// Builds the configured EventFilterDefinitionCollection
    pub fn build(self) -> Vec<EventFilterDefinition>{
        self.events
    }

}

/// Represents the service used to build EventFilterDefinitions
pub struct EventFilterDefinitionBuilder{
    filter: EventFilterDefinition
}
impl EventFilterDefinitionBuilder{
    
    /// Initializes a new EventFilterDefinition
    pub fn new() -> Self{
        Self { filter: EventFilterDefinition::default() }
    }

    /// Adds a new attribute to filter events by
    pub fn with(&mut self, name: &str, value: Value) -> &mut Self{
        if self.filter.with.is_none(){
            self.filter.with = Some(HashMap::new());
        }
        if let Some(with) = &mut self.filter.with {
            with.insert(name.to_string(), value);
        }
        self
    }

    /// Sets a name/value mapping of the attributes to filter events by
    pub fn with_attributes(&mut self, attributes: HashMap<String, Value>) -> &mut Self{
        self.filter.with = Some(attributes);
        self
    }

    /// Builds the configured EventFilterDefinition
    pub fn build(self) -> EventFilterDefinition{
        self.filter
    }

}

/// Represents the service used to build SubscriptionIteratorDefinitions
pub struct SubscriptionIteratorDefinitionBuilder{
    iterator: SubscriptionIteratorDefinition
}
impl SubscriptionIteratorDefinitionBuilder{

    /// Initializes a new SubscriptionIteratorDefinitionBuilder
    pub fn new() -> Self{
        Self { iterator: SubscriptionIteratorDefinition::new() }
    }

    /// Sets the name of the variable used to store the item being enumerated
    pub fn with_item(&mut self, variable: &str) -> &mut Self{
        self.iterator.item = Some(variable.to_string());
        self
    }

    /// Sets the name of the variable used to store the index of the item being enumerated
    pub fn at(&mut self, variable: &str) -> &mut Self{
        self.iterator.at = Some(variable.to_string());
        self
    }

    /// Configures the tasks to run for each consumd event/message
    pub fn do_<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TaskDefinitionMapBuilder){
        let mut builder = TaskDefinitionMapBuilder::new();
        setup(& mut builder);
        self.iterator.do_ = Some(builder.build());
        self
    }

    /// Configures the data each iteration outputs
    pub fn with_output<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.iterator.output = Some(builder.build());
        self
    }

    /// Configures the data exported with each iteration
    pub fn with_export<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OutputDataModelDefinitionBuilder){
        let mut builder = OutputDataModelDefinitionBuilder::new();
        setup(&mut builder);
        self.iterator.export = Some(builder.build());
        self
    }

    /// Buils the configures SubscriptionIteratorDefinition
    pub fn build(self) -> SubscriptionIteratorDefinition{
        self.iterator
    }

}

/// Represents the service used to build ErrorDefinition
pub struct ErrorDefinitionBuilder{
    error: ErrorDefinition
}
impl ErrorDefinitionBuilder{

    /// Initializes a new ErrorDefinitionBuilder
    pub fn new() -> Self{
        Self { error: ErrorDefinition::default() }
    }

    /// Sets the error's type
    pub fn with_type(&mut self, type_: &str) -> &mut Self{
        self.error.type_ = type_.to_string();
        self
    }

    /// Sets the error's status
    pub fn with_status(&mut self, status: Value) -> &mut Self{
        self.error.status = status;
        self
    }

    /// Sets the error's title
    pub fn with_title(&mut self, title: &str) -> &mut Self{
        self.error.title = title.to_string();
        self
    }

    /// Sets the error's detail
    pub fn with_detail(&mut self, detail: &str) -> &mut Self{
        self.error.detail = Some(detail.to_string());
        self
    }

    /// Sets a reference to the component the error concerns
    pub fn with_instance(&mut self, instance: &str) -> &mut Self{
        self.error.instance = Some(instance.to_string());
        self
    }

    /// Builds the configured ErrorDefinition
    pub fn build(self) -> ErrorDefinition{
        self.error
    }

}

/// Enumerates all supported process definition builders
pub enum ProcessDefinitionBuilder{
    Container(ContainerProcessDefinitionBuilder),
    Script(ScriptProcessDefinitionBuilder),
    Shell(ShellProcessDefinitionBuilder),
    Workflow(WorkflowProcessDefinitionBuilder)
}

///Represents the service used to build ContainerProcessDefinitions
pub struct ContainerProcessDefinitionBuilder{
    process: ContainerProcessDefinition
}
impl ContainerProcessDefinitionBuilder{

    /// Initializes a new ContainerProcessDefinitionBuilder
    pub fn new() -> Self{
        Self { process: ContainerProcessDefinition::default() }
    }

    /// Configures the name of the container to spawn
    pub fn with_name(&mut self, name: &str) -> &mut Self{
        self.process.name = Some(name.to_string());
        self
    }

    /// Configures the container to use the specified image
    pub fn with_image(&mut self, image: &str) -> &mut Self{
        self.process.image = image.to_string();
        self
    }

    /// Configures the command, if any, to execute on the container
    pub fn with_command(&mut self, command: &str) -> &mut Self{
        self.process.command = Some(command.to_string());
        self
    }

    /// Adds the specified container port mapping
    pub fn with_port(&mut self, host_port: u16, container_port: u16) -> &mut Self{
        if self.process.ports.is_none(){
            self.process.ports = Some(HashMap::new());
        }
        if let Some(ports) = &mut self.process.ports {
            ports.insert(host_port, container_port);
        }
        self
    }

    /// Sets the container's port mapping
    pub fn with_ports(&mut self, ports: HashMap<u16, u16>) -> &mut Self{
        self.process.ports = Some(ports);
        self
    }

    /// Adds the specified volume to the container
    pub fn with_volume(&mut self, key: &str, value: &str) -> &mut Self{
        if self.process.volumes.is_none(){
            self.process.volumes = Some(HashMap::new());
        }
        if let Some(volumes) = &mut self.process.volumes {
            volumes.insert(key.to_string(), value.to_string());
        }
        self
    }

    /// Sets the container's volumes
    pub fn with_volumes(&mut self, volumes: HashMap<String, String>) -> &mut Self{
        self.process.volumes = Some(volumes);
        self
    }

    /// Adds the specified environment variable to the container
    pub fn with_environment(&mut self, name: &str, value: &str) -> &mut Self{
        if self.process.environment.is_none(){
            self.process.environment = Some(HashMap::new());
        }
        if let Some(environment) = &mut self.process.environment {
            environment.insert(name.to_string(), value.to_string());
        }
        self
    }

    /// Sets the container's environment variables
    pub fn with_environment_variables(&mut self, environment: HashMap<String, String>) -> &mut Self{
        self.process.environment = Some(environment);
        self
    }

    /// Builds the configured RunTaskDefinition
    pub fn build(self) -> RunTaskDefinition{
        let mut run_task = RunTaskDefinition::default();
        run_task.run.container = Some(self.process);
        run_task
    }

}

///Represents the service used to build ScriptProcessDefinitions
pub struct ScriptProcessDefinitionBuilder{
    process: ScriptProcessDefinition
}
impl ScriptProcessDefinitionBuilder{

    /// Initializes a new ScriptProcessDefinitionBuilder
    pub fn new() -> Self{
        Self { process: ScriptProcessDefinition::default() }
    }

    /// Sets the language of the script to run
    pub fn with_language(&mut self, language: &str) -> &mut Self{
        self.process.language = language.to_string();
        self
    }

    /// Sets the code of the script to run
    pub fn with_code(&mut self, code: &str) -> &mut Self{
        self.process.code = Some(code.to_string());
        self
    }

    /// Sets the source of the script to run
    pub fn with_source<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut ExternalResourceDefinitionBuilder){
        let mut builder = ExternalResourceDefinitionBuilder::new();
        setup(&mut builder);
        let resource = builder.build();
        self.process.source = Some(resource); 
        self
    }

    /// Adds a new argument to execute the script with
    pub fn with_argument(&mut self, value: &str) -> &mut Self{
        if self.process.arguments.is_none(){
            self.process.arguments = Some(Vec::new());
        }
        if let Some(arguments) = &mut self.process.arguments {
            arguments.push(value.to_string());
        }
        self
    }

    /// Sets the arguments of the script to execute
    pub fn with_arguments(&mut self, arguments: Vec<String>) -> &mut Self{
        self.process.arguments = Some(arguments);
        self
    }

    /// Adds the specified environment variable to the process
    pub fn with_environment(&mut self, name: &str, value: &str) -> &mut Self{
        if self.process.environment.is_none(){
            self.process.environment = Some(HashMap::new());
        }
        if let Some(environment) = &mut self.process.environment {
            environment.insert(name.to_string(), value.to_string());
        }
        self
    }

    /// Sets the process's environment variables
    pub fn with_environment_variables(&mut self, environment: HashMap<String, String>) -> &mut Self{
        self.process.environment = Some(environment);
        self
    }

    pub fn with_stdin(&mut self, stdin: &str) -> &mut Self{
        self.process.stdin = Some(stdin.to_string());
        self
    }

    /// Builds the configured RunTaskDefinition
    pub fn build(self) -> RunTaskDefinition{
        let mut run_task = RunTaskDefinition::default();
        run_task.run.script = Some(self.process);
        run_task
    }

}

///Represents the service used to build ShellProcessDefinitions
pub struct ShellProcessDefinitionBuilder{
    process: ShellProcessDefinition
}
impl ShellProcessDefinitionBuilder{

    /// Initializes a new ShellProcessDefinitions
    pub fn new() -> Self{
        Self { process: ShellProcessDefinition::default() }
    }

    /// Configures the task to execute the specified shell command
    pub fn with_command(&mut self, command: &str) -> &mut Self{
        self.process.command = command.to_string();
        self
    }

    /// Adds a new argument to execute the shell command with
    pub fn with_argument(&mut self, argument: &str) -> &mut Self{
        if self.process.arguments.is_none(){
            self.process.arguments = Some(Vec::new());
        }
        if let Some(arguments) = &mut self.process.arguments {
            arguments.push(argument.to_string());
        }
        self
    }

    /// Sets the arguments of the shell command to execute
    pub fn with_arguments(&mut self, arguments: Vec<String>) -> &mut Self{
        self.process.arguments = Some(arguments);
        self
    }

    /// Adds the specified environment variable to the process
    pub fn with_environment(&mut self, name: &str, value: &str) -> &mut Self{
        if self.process.environment.is_none(){
            self.process.environment = Some(HashMap::new());
        }
        if let Some(environment) = &mut self.process.environment {
            environment.insert(name.to_string(), value.to_string());
        }
        self
    }

    /// Sets the process's environment variables
    pub fn with_environment_variables(&mut self, environment: HashMap<String, String>) -> &mut Self{
        self.process.environment = Some(environment);
        self
    }

    /// Builds the configured RunTaskDefinition
    pub fn build(self) -> RunTaskDefinition{
        let mut run_task = RunTaskDefinition::default();
        run_task.run.shell = Some(self.process);
        run_task
    }

}

///Represents the service used to build WorkflowProcessDefinitions
pub struct WorkflowProcessDefinitionBuilder{
    process: WorkflowProcessDefinition
}
impl WorkflowProcessDefinitionBuilder{

    /// Initializes a new WorkflowProcessDefinitions
    pub fn new() -> Self{
        Self { process: WorkflowProcessDefinition::default() }
    }

    /// Configures the task to run the workflow with the specified namespace
    pub fn with_namespace(&mut self, namespace: &str) -> &mut Self{
        self.process.namespace = namespace.to_string();
        self
    }

    /// Configures the task to run the workflow with the specified name
    pub fn with_name(&mut self, name: &str) -> &mut Self{
        self.process.name = name.to_string();
        self
    }

    /// Configures the task to run the workflow with the specified version
    pub fn with_version(&mut self, version: &str) -> &mut Self{
        self.process.version = version.to_string();
        self
    }

    /// Sets the input of the workflow to run
    pub fn with_input(&mut self, input: Value) -> &mut Self{
        self.process.input = Some(input);
        self
    }

    /// Builds the configured RunTaskDefinition
    pub fn build(self) -> RunTaskDefinition{
        let mut run_task = RunTaskDefinition::default();
        run_task.run.workflow = Some(self.process);
        run_task
    }

}

/// Represents the service used to build ExternalResourceDefinitions
pub struct ExternalResourceDefinitionBuilder{
    resource: ExternalResourceDefinition
}
impl ExternalResourceDefinitionBuilder{

    /// Initializes a new ExternalResourceDefinitionBuilder
    pub fn new() -> Self{
        Self { resource:ExternalResourceDefinition::default() }
    }

    /// Configures the name of the referenced external resource
    pub fn with_name(&mut self, name: &str) -> &mut Self{
        self.resource.name = Some(name.to_string());
        self
    }

    /// Configures the endpoint at which to get the defined resource
    pub fn with_endpoint<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut EndpointDefinitionBuilder){
        let mut builder = EndpointDefinitionBuilder::new();
        setup(&mut builder);
        let endpoint = builder.build();
        self.resource.endpoint = OneOfEndpointDefinitionOrUri::Endpoint(endpoint);
        self
    }

    /// Configures the endpoint at which to get the defined resource
    pub fn with_endpoint_uri(&mut self, uri: &str) -> &mut Self{
        self.resource.endpoint = OneOfEndpointDefinitionOrUri::Uri(uri.to_string());
        self
    }

    /// Builds the configured ExternalResourceDefinition
    pub fn build(self) -> ExternalResourceDefinition{
        self.resource
    }

}

/// Represents the service used to build EndpointDefinitions
pub struct EndpointDefinitionBuilder{
    endpoint: EndpointDefinition
}
impl EndpointDefinitionBuilder{

    /// Initializes a new EndpointDefinitionBuilder
    pub fn new() -> Self{
        Self { endpoint: EndpointDefinition::default() }
    }

    /// Sets the endpoint's <see cref="Uri"/>
    pub fn with_uri(&mut self, uri: &str) -> &mut Self{
        self.endpoint.uri = uri.to_string();
        self
    }

    /// Configures the authentication policy used to access the configured endpoint
    pub fn with_authentication<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut AuthenticationPolicyDefinitionBuilder){
        let mut builder = AuthenticationPolicyDefinitionBuilder::new();
        setup(&mut builder);
        let authentication = builder.build();
        self.endpoint.authentication = Some(authentication);
        self
    }

    /// Builds the configured EndpointDefinition
    pub fn build(self) -> EndpointDefinition{
        self.endpoint
    }

}

/// Represents the service used to build SwitchCaseDefinitions
pub struct SwitchCaseDefinitionBuilder{
    case: SwitchCaseDefinition
}
impl SwitchCaseDefinitionBuilder{

    /// Initializes a new SwitchCaseDefinitionBuilder
    pub fn new() -> Self{
        Self { case: SwitchCaseDefinition::default() }
    }

    /// Sets a runtime expression that defines whether or not the case applies
    pub fn when(&mut self, expression: &str) -> &mut Self{
        self.case.when = Some(expression.to_string());
        self
    }

    /// Sets the flow directive to execute when the case is matched
    pub fn then(&mut self, directive: &str) -> &mut Self{
        self.case.then = Some(directive.to_string());
        self
    }

    /// Builds the configured SwitchCaseDefinition
    pub fn build(self) -> SwitchCaseDefinition{
        self.case
    }

}

/// Represents the service used to build ErrorCatcherDefinitions
pub struct ErrorCatcherDefinitionBuilder{
    catch: ErrorCatcherDefinition
}
impl ErrorCatcherDefinitionBuilder{
    
    /// Initializes a new ErrorCatcherDefinitionBuilder
    pub fn new() -> Self{
        Self { catch: ErrorCatcherDefinition::default() }
    }

    /// Catches errors matching the specified filter
    pub fn errors<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut ErrroFilterDefinitionBuilder){
        let mut builder = ErrroFilterDefinitionBuilder::new();
        setup(&mut builder);
        self.catch.errors = Some(builder.build());
        self
    }

    /// Sets the name of the variable that contains caught errors
    pub fn as_(&mut self, variable: &str) -> &mut Self{
        self.catch.as_ = Some(variable.to_string());
        self
    }

    /// Sets the runtime expression used to determine whether to catch the filtered error
    pub fn when(&mut self, expression: &str) -> &mut Self{
        self.catch.when = Some(expression.to_string());
        self
    }

    /// Sets the runtime expression used to determine whether not to catch the filtered error
    pub fn except_when(&mut self, expression: &str) -> &mut Self{
        self.catch.except_when = Some(expression.to_string());
        self
    }

    /// Sets the reference to the retry policy to use
    pub fn retry<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut RetryPolicyDefinitionBuilder){
        let mut builder = RetryPolicyDefinitionBuilder::new();
        setup(&mut builder);
        self.catch.retry = Some(OneOfRetryPolicyDefinitionOrReference::Retry(builder.build()));
        self
    }

    /// Sets the reference to the retry policy to use
    pub fn retry_using(&mut self, reference: &str) -> &mut Self{
        self.catch.retry = Some(OneOfRetryPolicyDefinitionOrReference::Reference(reference.to_string()));
        self
    }

    /// Configures the tasks to execute the specified task after catching or after retry exhaustion
    pub fn do_<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut TaskDefinitionMapBuilder){
        let mut builder = TaskDefinitionMapBuilder::new();
        setup(&mut builder);
        self.catch.do_ = Some(builder.build());
        self
    }

    /// Builds the configured ErrorCatcherDefinition
    pub fn build(self) -> ErrorCatcherDefinition{
        self.catch
    }

}

/// Represents the service used to build ErrroFilterDefinitions
pub struct ErrroFilterDefinitionBuilder{
    filter: ErrorFilterDefinition
}
impl ErrroFilterDefinitionBuilder{

    /// Initializes a new ErrroFilterDefinitionBuilder
    pub fn new() -> Self{
        Self { filter: ErrorFilterDefinition::default() }
    }

     /// Adds a new attribute filter
     pub fn with(&mut self, name: &str, value: Value) -> &mut Self{
        if self.filter.with.is_none(){
            self.filter.with = Some(HashMap::new());
        }
        if let Some(with) = &mut self.filter.with {
            with.insert(name.to_string(), value);
        }
        self
    }

    /// Sets a name/value mapping of the attributes to filter errors by
    pub fn with_attributes(&mut self, attributes: HashMap<String, Value>) -> &mut Self{
        self.filter.with = Some(attributes);
        self
    }

    /// Builds the configured ErrorFilterDefinition
    pub fn build(self) -> ErrorFilterDefinition{
        self.filter
    }

}

/// Represents the service used to build RetryPolicyDefinitions
pub struct RetryPolicyDefinitionBuilder{
    retry: RetryPolicyDefinition
}
impl RetryPolicyDefinitionBuilder{

    /// Initializes a new RetryPolicyDefinitionBuilder
    pub fn new() -> Self{
        Self { retry: RetryPolicyDefinition::default() }
    }

    /// Sets the runtime expression used to determine whether to retry the filtered error
    pub fn when(&mut self, expression: &str) -> &mut Self{
        self.retry.when = Some(expression.to_string());
        self
    }

    /// Sets the runtime expression used to determine whether not to retry the filtered error
    pub fn except_when(&mut self, expression: &str) -> &mut Self{
        self.retry.except_when = Some(expression.to_string());
        self
    }

    /// Sets the limits of the retry policy to build
    pub fn limit<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut RetryPolicyLimitDefinitionBuilder){
        let mut builder = RetryPolicyLimitDefinitionBuilder::new();
        setup(&mut builder);
        self.retry.limit = Some(builder.build());
        self
    }

    /// Sets the delay duration between retry attempts
    pub fn delay(&mut self, duration: Duration) -> &mut Self{
        self.retry.delay = Some(duration);
        self
    }

    /// Sets the backoff strategy of the retry policy to build
    pub fn backoff<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut GenericBackoffStrategyDefinitionBuilder){
        let mut builder = GenericBackoffStrategyDefinitionBuilder::new();
        setup(&mut builder);
        self.retry.backoff = Some(builder.build());
        self
    }

    /// Sets the jitter to apply to the retry policy to build
    pub fn jitter<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut JitterDefinitionBuilder){
        let mut builder = JitterDefinitionBuilder::new();
        setup(&mut builder);
        self.retry.jitter = Some(builder.build());
        self
    }

    /// Builds the configured RetryPolicyDefinition
    pub fn build(self) -> RetryPolicyDefinition{
        self.retry
    }
    
}

/// Represents the service used to build RetryPolicyLimitDefinitions
pub struct RetryPolicyLimitDefinitionBuilder{
    limit: RetryPolicyLimitDefinition
}
impl RetryPolicyLimitDefinitionBuilder{

    /// Initializes a new RetryPolicyLimitDefinitionBuilder
    pub fn new() -> Self{
        Self { limit: RetryPolicyLimitDefinition::default() }
    }
    
    /// Configures retry attempts limits
    pub fn attempt<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut RetryAttemptLimitDefinitionBuilder){
        let mut builder = RetryAttemptLimitDefinitionBuilder::new();
        setup(&mut builder);
        self.limit.attempt = Some(builder.build());
        self
    }

    /// Configures the maximum duration during which retrying is allowed
    pub fn duration(&mut self, duration: Duration) -> &mut Self{
        self.limit.duration = Some(duration);
        self
    }

    /// Builds the configured RetryPolicyLimitDefinition
    pub fn build(self) -> RetryPolicyLimitDefinition{
        self.limit
    } 

}

/// Represents the service used to build RetryAttemptLimitDefinitions
pub struct RetryAttemptLimitDefinitionBuilder{
    attempt: RetryAttemptLimitDefinition
}
impl RetryAttemptLimitDefinitionBuilder{

    /// Initializes a new RetryAttemptLimitDefinitionBuilder
    pub fn new() -> Self{
        Self { attempt: RetryAttemptLimitDefinition::default() }
    }
    
    /// Sets the maximum attempts count
    pub fn count(&mut self, count: u16) -> &mut Self{
        self.attempt.count = Some(count);
        self
    }

    /// Sets the maximum duration per attempt
    pub fn duration(&mut self, duration: Duration) -> &mut Self{
        self.attempt.duration = Some(duration);
        self
    }

    /// Builds the configured RetryAttemptLimitDefinition
    pub fn build(self) -> RetryAttemptLimitDefinition{
        self.attempt
    } 

}

/// Represents the service used to build BackoffStrategyDefinitions
pub struct GenericBackoffStrategyDefinitionBuilder{
    builder: Option<BackoffStrategyDefinitionBuilder>
}
impl GenericBackoffStrategyDefinitionBuilder{

    /// Initializes a new BackoffStrategyDefinitionBuilder
    pub fn new() -> Self{
        Self { builder: None }
    }
    
    /// Configures a constant backoff strategy
    pub fn constant(&mut self) -> &mut ConstantBackoffDefinitionBuilder{
        let builder = ConstantBackoffDefinitionBuilder::new();
        self.builder = Some(BackoffStrategyDefinitionBuilder::Constant(builder));
        if let Some(BackoffStrategyDefinitionBuilder::Constant(ref mut builder)) = self.builder{
            builder
        }
        else{
            unreachable!("Builder should always be set to Constant");
        }
    }

    /// Configures an exponential backoff strategy
    pub fn exponential(&mut self) -> &mut ExponentialBackoffDefinitionBuilder{
        let builder = ExponentialBackoffDefinitionBuilder::new();
        self.builder = Some(BackoffStrategyDefinitionBuilder::Exponential(builder));
        if let Some(BackoffStrategyDefinitionBuilder::Exponential(ref mut builder)) = self.builder{
            builder
        }
        else{
            unreachable!("Builder should always be set to Exponential");
        }
    }

    /// Configures a linear backoff strategy
    pub fn linear(&mut self) -> &mut LinearBackoffDefinitionBuilder{
        let builder = LinearBackoffDefinitionBuilder::new();
        self.builder = Some(BackoffStrategyDefinitionBuilder::Linear(builder));
        if let Some(BackoffStrategyDefinitionBuilder::Linear(ref mut builder)) = self.builder{
            builder
        }
        else{
            unreachable!("Builder should always be set to Linear");
        }
    }

    /// Builds the configured BackoffStrategyDefinitions
    pub fn build(self) -> BackoffStrategyDefinition{
        if let Some(builder) = self.builder{
            match builder{
                BackoffStrategyDefinitionBuilder::Constant(builder) => builder.build(),
                BackoffStrategyDefinitionBuilder::Exponential(builder) => builder.build(),
                BackoffStrategyDefinitionBuilder::Linear(builder) => builder.build(),
            }
        }
        else{
            unreachable!("The backoff strategy must be configured")
        }
    } 

}

/// Enumerates all supported BackoffStrategyDefinition builders
pub enum BackoffStrategyDefinitionBuilder{
    Constant(ConstantBackoffDefinitionBuilder),
    Exponential(ExponentialBackoffDefinitionBuilder),
    Linear(LinearBackoffDefinitionBuilder)
}

/// Represents the service used to build ConstantBackoffDefinitions
pub struct ConstantBackoffDefinitionBuilder;
impl ConstantBackoffDefinitionBuilder{

    /// Initializes a new ConstantBackoffDefinitionBuilder
    pub fn new() -> Self{
        Self{}
    }

    /// Builds the configures ConstantBackoffDefinition
    pub fn build(self) -> BackoffStrategyDefinition{
        let mut strategy = BackoffStrategyDefinition::new();
        strategy.constant = Some(ConstantBackoffDefinition::new());
        strategy
    }

}

/// Represents the service used to build ExponentialBackoffDefinitionBuilder
pub struct ExponentialBackoffDefinitionBuilder;
impl ExponentialBackoffDefinitionBuilder{

    /// Initializes a new ExponentialBackoffDefinitionBuilder
    pub fn new() -> Self{
        Self{}
    }

    /// Builds the configures ExponentialBackoffDefinition
    pub fn build(self) -> BackoffStrategyDefinition{
        let mut strategy = BackoffStrategyDefinition::new();
        strategy.exponential = Some(ExponentialBackoffDefinition::new());
        strategy
    }

}

/// Represents the service used to build LinearBackoffDefinition
pub struct LinearBackoffDefinitionBuilder{
    increment: Option<Duration>
}
impl LinearBackoffDefinitionBuilder{

    /// Initializes a new LinearBackoffDefinitionBuilder
    pub fn new() -> Self{
        Self{ increment: None }
    }

    /// Sets the linear incrementation to the delay between retry attempts
    pub fn with_increment(&mut self, increment: Duration) -> &mut Self{
        self.increment = Some(increment);
        self
    }

    /// Builds the configures LinearBackoffDefinition
    pub fn build(self) -> BackoffStrategyDefinition{
        let mut strategy = BackoffStrategyDefinition::new();
        let mut linear = LinearBackoffDefinition::new();
        linear.increment = self.increment;
        strategy.linear = Some(linear);
        strategy
    }

}

/// Represents the service used to build JitterDefinitions
pub struct JitterDefinitionBuilder{
    jitter: JitterDefinition
}
impl JitterDefinitionBuilder{

    /// Initializes a new JitterDefinitionBuilder
    pub fn new() -> Self{
        Self { jitter: JitterDefinition::default() }
    }
    
    /// Sets the jitter range's minimum duration
    pub fn from(&mut self, duration: Duration) -> &mut Self{
        self.jitter.from = duration;
        self
    }

    /// Sets the jitter range's maximum duration
    pub fn to(&mut self, duration: Duration) -> &mut Self{
        self.jitter.to = duration;
        self
    }

    /// Builds the configured JitterDefinition
    pub fn build(self) -> JitterDefinition{
        self.jitter
    } 

}

/// Represents the service used to build InputDataModelDefinitions
pub struct InputDataModelDefinitionBuilder{
    input : InputDataModelDefinition
}
impl InputDataModelDefinitionBuilder{

    /// Initializes a new InputDataModelDefinitionBuilder
    pub fn new() -> Self{
        Self{ input: InputDataModelDefinition::default() }
    }

    /// Configures the input schema
    pub fn with_schema<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut SchemaDefinitionBuilder){
        let mut builder = SchemaDefinitionBuilder::new();
        setup(&mut builder);
        self.input.schema = Some(builder.build());
        self
    }

    /// Configures the expression used to filter the input
    pub fn from(&mut self, expression: Value) -> &mut Self{
        self.input.from = Some(expression);
        self
    }

    /// Builds the configured InputDataModelDefinition
    pub fn build(self) -> InputDataModelDefinition{
        self.input
    }

}

/// Represents the service used to build OutputDataModelDefinitions
pub struct OutputDataModelDefinitionBuilder{
    output : OutputDataModelDefinition
}
impl OutputDataModelDefinitionBuilder{

    /// Initializes a new OutputDataModelDefinitionBuilder
    pub fn new() -> Self{
        Self{ output: OutputDataModelDefinition::default() }
    }

    /// Sets a runtime expression, if any, used to output specific data to the scope data
    pub fn as_(&mut self, expression: Value) -> &mut Self{
        self.output.as_ = Some(expression);
        self
    }

    /// Builds the configured OutputDataModelDefinition
    pub fn build(self) -> OutputDataModelDefinition{
        self.output
    }

}

/// Represents the service used to build SchemaDefinitions
pub struct SchemaDefinitionBuilder{
    schema: SchemaDefinition
}
impl SchemaDefinitionBuilder{

    /// Initializes a new SchemaDefinitionBuilder
    pub fn new() -> Self{
        Self { schema: SchemaDefinition::default() }
    }

    /// Sets the schema format
    pub fn with_format(&mut self, format: &str) -> &mut Self{
        self.schema.format = format.to_string();
        self
    }

    /// Sets the schema resource
    pub fn with_resource<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut ExternalResourceDefinitionBuilder){
        let mut builder = ExternalResourceDefinitionBuilder::new();
        setup(&mut builder);
        self.schema.resource = Some(builder.build());
        self
    }

    /// Sets the schema document
    pub fn with_document(&mut self, document: Value) -> &mut Self{
        self.schema.document = Some(document);
        self
    }

    /// Builds the configured SchemaDefinition
    pub fn build(self) -> SchemaDefinition{
        self.schema
    }

}