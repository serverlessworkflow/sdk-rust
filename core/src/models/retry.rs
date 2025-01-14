use serde_derive::{Deserialize, Serialize};
use crate::models::duration::*;

/// Represents the definition of a retry policy
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryPolicyDefinition{

    /// Gets/sets a runtime expression used to determine whether or not to retry running the task, in a given context
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,

    /// Gets/sets a runtime expression used to determine whether or not to retry running the task, in a given context
    #[serde(rename = "exceptWhen", skip_serializing_if = "Option::is_none")]
    pub except_when: Option<String>,

    /// Gets/sets the limits, if any, of the retry policy
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<RetryPolicyLimitDefinition>,

    /// Gets/sets the delay duration between retry attempts
    #[serde(rename = "delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<Duration>,

    /// Gets/sets the backoff strategy to use, if any
    #[serde(rename = "backoff", skip_serializing_if = "Option::is_none")]
    pub backoff: Option<BackoffStrategyDefinition>,

    /// Gets/sets the parameters, if any, that control the randomness or variability of the delay between retry attempts
    #[serde(rename = "jitter", skip_serializing_if = "Option::is_none")]
    pub jitter: Option<JitterDefinition>

}

/// Represents the configuration of the limits of a retry policy
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryPolicyLimitDefinition{

    /// Gets/sets the definition of the limits for all retry attempts of a given policy
    #[serde(rename = "attempt", skip_serializing_if = "Option::is_none")]
    pub attempt: Option<RetryAttemptLimitDefinition>,

    /// Gets/sets the maximum duration, if any, during which to retry a given task
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,

}

/// Represents the definition of the limits for all retry attempts of a given policy
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryAttemptLimitDefinition{

    /// Gets/sets the maximum attempts count
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u16>,

    /// Gets/sets the duration limit, if any, for all retry attempts
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>

}

/// Represents the definition of a retry backoff strategy
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackoffStrategyDefinition{

    /// Gets/sets the definition of the constant backoff to use, if any
    #[serde(rename = "constant", skip_serializing_if = "Option::is_none")]
    pub constant: Option<ConstantBackoffDefinition>,

    /// Gets/sets the definition of the exponential backoff to use, if any
    #[serde(rename = "exponential", skip_serializing_if = "Option::is_none")]
    pub exponential: Option<ExponentialBackoffDefinition>,

    /// Gets/sets the definition of the linear backoff to use, if any
    #[serde(rename = "linear", skip_serializing_if = "Option::is_none")]
    pub linear: Option<LinearBackoffDefinition>

}
impl BackoffStrategyDefinition{

    /// Initializes a new BackoffStrategyDefinition
    pub fn new() -> Self{
        Self{
            constant: None,
            exponential: None,
            linear: None
        }
    }

}

/// Represents the definition of a constant backoff
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstantBackoffDefinition{

}
impl ConstantBackoffDefinition{

    /// Initializes a new ConstantBackoffDefinition
    pub fn new() -> Self{
        Self{}
    }

}

/// Represents the definition of an exponential backoff
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExponentialBackoffDefinition{

}
impl ExponentialBackoffDefinition{

    /// Initializes a new ExponentialBackoffDefinition
    pub fn new() -> Self{
        Self{}
    }

}

/// Represents the definition of a linear backoff
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinearBackoffDefinition{

    /// Gets/sets the linear incrementation to the delay between retry attempts
    #[serde(rename = "increment", skip_serializing_if = "Option::is_none")]
    pub increment: Option<Duration>

}
impl LinearBackoffDefinition{

    /// Initializes a new LinearBackoffDefinition
    pub fn new() -> Self{
        Self{ increment: None }
    }

}

/// Represents the definition of the parameters that control the randomness or variability of a delay, typically between retry attempts
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct JitterDefinition{

    /// Gets/sets the minimum duration of the jitter range
    #[serde(rename = "from")]
    pub from: Duration,

    /// Gets/sets the maximum duration of the jitter range
    #[serde(rename = "to")]
    pub to: Duration

}

/// Represents a value that can be either a RetryPolicyDefinition or a reference to a RetryPolicyDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOfRetryPolicyDefinitionOrReference{
    /// Variant holding an retry policy definition
    Retry(RetryPolicyDefinition),
    /// Variant holding a reference to the retry policy definition to use
    Reference(String)
}
impl Default for OneOfRetryPolicyDefinitionOrReference {
    fn default() -> Self {
        // Choose a default variant
        OneOfRetryPolicyDefinitionOrReference::Retry(RetryPolicyDefinition::default())
    }
}