use serverless_workflow_core::models::any::*;
use serverless_workflow_core::models::duration::*;
use serverless_workflow_core::models::error::ErrorDefinition;
use serverless_workflow_core::models::error::OneOfErrorDefinitionOrReference;
use serverless_workflow_core::models::event::*;
use serverless_workflow_core::models::map::*;
use serverless_workflow_core::models::task::*;
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
    Wait(WaitTaskDefinitionBuilder)
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
    pub fn with(&mut self, name: &str, value: AnyValue) -> &mut Self{
        if self.task.with.is_none(){
            self.task.with = Some(HashMap::new());
        }
        if let Some(with) = &mut self.task.with {
            with.insert(name.to_string(), value);
        }
        self
    }

    /// Sets the arguments to call the function with
    pub fn with_arguments(&mut self, arguments: HashMap<String, AnyValue>) -> &mut Self{
        self.task.with = Some(arguments);
        self
    }

    /// Builds the configures CallTaskDefinition
    pub fn build(self) -> TaskDefinition{
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
    
    /// Builds the configures DoTaskDefinition
    pub fn build(self) -> TaskDefinition{
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

    /// Builds the configures DoTaskDefinition
    pub fn build(self) -> TaskDefinition{
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
     
    /// Builds the configured ForTaskDefinition
    pub fn build(self) -> TaskDefinition{
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

    /// Builds the configured ForkTaskDefinition
    pub fn build(self) -> TaskDefinition{
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

    /// Builds the configured ListenTaskDefinition
    pub fn build(self) -> TaskDefinition{
        TaskDefinition::Listen(self.task)
    }

}

/// Represents the service used to build RaiseTaskDefinitions
pub struct RaiseTaskDefinitionBuilder{
    builder: Option<ErrorDefinitionBuilder>,
    reference: Option<String>
}
impl RaiseTaskDefinitionBuilder{

    /// Initializes a new RaiseTaskDefinitionBuilder
    pub fn new() -> Self{
        Self { builder: None, reference: None }
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

    /// Builds the configured RaiseTaskDefinition
    pub fn build(self) -> TaskDefinition{
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
        TaskDefinition::Raise(task)
    }

}

/// Represents the service used to build RunTaskDefinitions
pub struct RunTaskDefinitionBuilder{
    builder : Option<ProcessDefinitionBuilder>
}
impl RunTaskDefinitionBuilder{

    /// Initializes a new RunTaskDefinitionBuilder
    pub fn new() -> Self{
        Self{ builder: None }
    }

    /// Configures the task to run the specified container
    pub fn container(&mut self) -> &mut ContainerProcessDefinitionBuilder{
        let &mut builder = ContainerProcessDefinitionBuilder::new();
        if let Some(ProcessDefinitionBuilder::Container(ref mut container_builder)) = self.builder{
            container_builder
        }
        else {
            unreachable!("Builder should always be set to Container");
        }
    }

    /// Configures the task to run the specified script
    pub fn script(&mut self) -> &mut ScriptProcessDefinitionBuilder{
        let &mut builder = ScriptProcessDefinitionBuilder::new();
        if let Some(ProcessDefinitionBuilder::Script(ref mut script_builder)) = self.builder{
            script_builder
        }
        else {
            unreachable!("Builder should always be set to Script");
        }
    }

    /// Configures the task to run the specified shell command
    pub fn shell(&mut self) -> &mut ShellProcessDefinitionBuilder{
        let &mut builder = ShellProcessDefinitionBuilder::new();
        if let Some(ProcessDefinitionBuilder::Shell(ref mut shell_builder)) = self.builder{
            shell_builder
        }
        else {
            unreachable!("Builder should always be set to Shell");
        }
    }

    /// Configures the task to run the specified workflow
    pub fn workflow(&mut self) -> &mut WorkflowProcessDefinitionBuilder{
        let &mut builder = WorkflowProcessDefinitionBuilder::new();
        if let Some(ProcessDefinitionBuilder::Workflow(ref mut workflow_builder)) = self.builder{
            workflow_builder
        }
        else {
            unreachable!("Builder should always be set to Workflow");
        }
    }

    /// Builds the configured RunTaskDefinition
    pub fn build(self) -> TaskDefinition{
        if let builder = self.builder{
            let process = builder.build();
            TaskDefinition::Run(process)
        }
        else{
            panic!("The process to run must be configured");
        }
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
    
    /// Builds the configures DoTaskDefinition
    pub fn build(self) -> TaskDefinition{
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
    pub fn with(&mut self, name: &str, value: AnyValue) -> &mut Self{
        self.event.with.insert(name.to_string(), value);
        self
    }

    /// Sets the event's attributes
     pub fn with_attributes(&mut self, attributes: HashMap<String, AnyValue>) -> &mut Self{
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
    one: Option<EventFilterDefinitionBuilder>
}
impl EventConsumptionStrategyDefinitionBuilder{

    /// Initializes a new EventConsumptionStrategyDefinitionBuilder
    pub fn new() -> Self{
        Self { all: None, any: None, one: None }
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

    /// Builds the configured EventConsumptionStrategyDefinition
    pub fn build(self) -> EventConsumptionStrategyDefinition{
        let mut strategy = EventConsumptionStrategyDefinition::default();
        if let Some(all_builder) = self.all {
            strategy.all = Some(all_builder.build());
        }
        else if let Some(any_builder) = self.any {
            strategy.any = Some(any_builder.build());
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
    pub fn with(&mut self, name: &str, value: AnyValue) -> &mut Self{
        if self.filter.with.is_none(){
            self.filter.with = Some(HashMap::new());
        }
        if let Some(with) = &mut self.filter.with {
            with.insert(name.to_string(), value);
        }
        self
    }

    /// Sets a name/value mapping of the attributes to filter events by
    pub fn with_attributes(&mut self, attributes: HashMap<String, AnyValue>) -> &mut Self{
        self.filter.with = Some(attributes);
        self
    }

    /// Builds the configured EventFilterDefinition
    pub fn build(self) -> EventFilterDefinition{
        self.filter
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
    pub fn with_status(&mut self, status: AnyValue) -> &mut Self{
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

}
impl ContainerProcessDefinitionBuilder{

    /// Builds the configured ContainerProcessDefinition
    pub fn build() -> ContainerProcessDefinition{

    }

}

///Represents the service used to build ScriptProcessDefinitions
pub struct ScriptProcessDefinitionBuilder{

}
impl ScriptProcessDefinitionBuilder{

    /// Builds the configured ScriptProcessDefinition
    pub fn build() -> ScriptProcessDefinition{

    }

}

///Represents the service used to build ShellProcessDefinitions
pub struct ShellProcessDefinitionBuilder{

}
impl ShellProcessDefinitionBuilder{

    /// Builds the configured ShellProcessDefinition
    pub fn build() -> ShellProcessDefinition{

    }

}

///Represents the service used to build WorkflowProcessDefinitions
pub struct WorkflowProcessDefinitionBuilder{

}
impl WorkflowProcessDefinitionBuilder{

    /// Builds the configured WorkflowProcessDefinitionBuilder
    pub fn build() -> WorkflowProcessDefinitionBuilder{

    }

}