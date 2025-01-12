use serde_derive::{Deserialize, Serialize};

/// Represents a duration
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Duration{

    /// Gets/sets the numbers of days, if any
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days : Option<u64>,

    /// Gets/sets the numbers of hours, if any
    #[serde(rename = "hours", skip_serializing_if = "Option::is_none")]
    pub hours : Option<u64>,

    /// Gets/sets the numbers of minutes, if any
    #[serde(rename = "minutes", skip_serializing_if = "Option::is_none")]
    pub minutes : Option<u64>,

    /// Gets/sets the seconds of days, if any
    #[serde(rename = "seconds", skip_serializing_if = "Option::is_none")]
    pub seconds : Option<u64>,

    /// Gets/sets the numbers of milliseconds, if any
    #[serde(rename = "milliseconds", skip_serializing_if = "Option::is_none")]
    pub milliseconds : Option<u64>

}
impl Duration{
    
    /// Gets the the duration's total amount of days
    pub fn total_days(&self) -> f64{
        self.total_hours() / 24.0
    }

    /// Gets the the duration's total amount of hours
    pub fn total_hours(&self) -> f64{
        self.total_minutes() / 60.0
    }

    /// Gets the the duration's total amount of minutes
    pub fn total_minutes(&self) -> f64{
        self.total_seconds() / 60.0
    }

    /// Gets the the duration's total amount of seconds
    pub fn total_seconds(&self) -> f64{
        (self.total_milliseconds() as f64) / 1000.0
    }

    /// Gets the the duration's total amount of milliseconds
    pub fn total_milliseconds(&self) -> u64{
        let days_ms = self.days.unwrap_or(0) * 24 * 60 * 60 * 1000;
        let hours_ms = self.hours.unwrap_or(0) * 60 * 60 * 1000;
        let minutes_ms = self.minutes.unwrap_or(0) * 60 * 1000;
        let seconds_ms = self.seconds.unwrap_or(0) * 1000;
        let millis = self.milliseconds.unwrap_or(0);
        days_ms + hours_ms + minutes_ms + seconds_ms + millis
    }

}

/// Represents a value that can be either a Duration or an ISO 8601 duration expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOfDurationOrIso8601Expression{
    /// Variant holding a duration
    Duration(Duration),
    /// Variant holding an ISO 8601 duration expression
    Iso8601Expression(String)
}
impl Default for OneOfDurationOrIso8601Expression {
    fn default() -> Self {
        // Choose a default variant
        OneOfDurationOrIso8601Expression::Duration(Duration::default())
    }
}