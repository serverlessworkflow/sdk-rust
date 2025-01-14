use serverless_workflow_core::models::duration::*;
use serverless_workflow_core::models::timeout::*;

/// Represents a service used to build TimeoutDefinitions
pub struct TimeoutDefinitionBuilder {
    timeout: TimeoutDefinition
}
impl TimeoutDefinitionBuilder {

    /// Initializes a new TimeoutDefinitionBuilder
    pub fn new() -> Self {
        Self {
            timeout: TimeoutDefinition::default()
        }
    }

    /// Sets the duration after which to timeout
    pub fn after(&mut self, duration: Duration) -> &mut Self{
        self.timeout.after = OneOfDurationOrIso8601Expression::Duration(duration);
        self
    }

    /// Sets the ISO 8601 expression of the duration after which to timeout
    pub fn after_expression(&mut self, duration: String) -> &mut Self{
        self.timeout.after = OneOfDurationOrIso8601Expression::Iso8601Expression(duration);
        self
    }

    /// Builds the configured TimeoutDefinition
    pub fn build(self) -> TimeoutDefinition {
        self.timeout
    }

}