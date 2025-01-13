use serde_derive::{Deserialize, Serialize};
use std::fmt;

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
    
    /// Initializes a new Duration from the specified amount of days
    pub fn from_days(days: u64) -> Self{
        Self { 
            days: Some(days), 
            hours: None, 
            minutes: None, 
            seconds: None, 
            milliseconds: None 
        }
    }

    /// Initializes a new Duration from the specified amount of hours
    pub fn from_hours(hours: u64) -> Self{
        Self { 
            days: None, 
            hours: Some(hours), 
            minutes: None, 
            seconds: None, 
            milliseconds: None 
        }
    }

    /// Initializes a new Duration from the specified amount of minutes
    pub fn from_minutes(minutes: u64) -> Self{
        Self { 
            days: None, 
            hours: None, 
            minutes: Some(minutes), 
            seconds: None, 
            milliseconds: None 
        }
    }

    /// Initializes a new Duration from the specified amount of seconds
    pub fn from_seconds(seconds: u64) -> Self{
        Self { 
            days: None, 
            hours: None, 
            minutes: None, 
            seconds: Some(seconds), 
            milliseconds: None 
        }
    }

    /// Initializes a new Duration from the specified amount of milliseconds
    pub fn from_milliseconds(milliseconds: u64) -> Self{
        Self { 
            days: None, 
            hours: None, 
            minutes: None, 
            seconds: None, 
            milliseconds: Some(milliseconds) 
        }
    }

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
impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if let Some(days) = self.days {
            parts.push(format!("{} days", days));
        }
        if let Some(hours) = self.hours {
            parts.push(format!("{} hours", hours));
        }
        if let Some(minutes) = self.minutes {
            parts.push(format!("{} minutes", minutes));
        }
        if let Some(seconds) = self.seconds {
            parts.push(format!("{} seconds", seconds));
        }
        if let Some(milliseconds) = self.milliseconds {
            parts.push(format!("{} milliseconds", milliseconds));
        }
        write!(f, "{}", parts.join(" "))
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
impl fmt::Display for OneOfDurationOrIso8601Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OneOfDurationOrIso8601Expression::Duration(duration) => write!(f, "{}", duration),
            OneOfDurationOrIso8601Expression::Iso8601Expression(expr) => write!(f, "{}", expr),
        }
    }
}