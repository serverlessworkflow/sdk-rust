use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::any::*;

/// Represents the configuration of an event consumption strategy
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventConsumptionStrategyDefinition{

    /// Gets/sets a list containing all the events that must be consumed, if any
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<EventFilterDefinition>>,

    /// Gets/sets a list containing any of the events to consume, if any
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<EventFilterDefinition>>,

    /// Gets/sets the single event to consume
    #[serde(rename = "one", skip_serializing_if = "Option::is_none")]
    pub one: Option<EventFilterDefinition>

}

/// Represents the configuration of an event filter
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventFilterDefinition{

    /// Gets/sets a name/value mapping of the attributes filtered events must define. Supports both regular expressions and runtime expressions
    #[serde(rename = "with", skip_serializing_if = "Option::is_none")]
    pub with : Option<HashMap<String, AnyValue>>,

    /// Gets/sets a name/definition mapping of the correlation to attempt when filtering events.
    #[serde(rename = "correlate", skip_serializing_if = "Option::is_none")]
    pub correlate: Option<HashMap<String, CorrelationKeyDefinition>>

}

/// Represents the definition of an event correlation key
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrelationKeyDefinition{

    /// Gets/sets a runtime expression used to extract the correlation key value from events
    #[serde(rename = "from")]
    pub from: String,

    /// Gets/sets a constant or a runtime expression, if any, used to determine whether or not the extracted correlation key value matches expectations and should be correlated. If not set, the first extracted value will be used as the correlation key's expectation
    #[serde(rename = "expect", skip_serializing_if = "Option::is_none")]
    pub expect: Option<String>

}
impl CorrelationKeyDefinition {
    pub fn new(from: &str, expect: Option<String>) -> Self{
        Self { 
            from: from.to_string(), 
            expect 
        }
    }
}

/// Represents the definition of an event
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventDefinition{

    /// Gets/sets a key/value mapping of the attributes of the configured event
    #[serde(rename = "with")]
    pub with: HashMap<String, AnyValue>

}
impl EventDefinition {
    pub fn new(with: HashMap<String, AnyValue>) -> Self{
        Self{
            with
        }
    }
}