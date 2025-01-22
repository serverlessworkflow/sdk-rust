use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::hash::Hash;

/// Represents an ordered key/value map array
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(bound(
    serialize = "TKey: Serialize, TValue: Serialize",
    deserialize = "TKey: for<'d> Deserialize<'d> + Eq + Hash + Clone + PartialEq, TValue: for<'d> Deserialize<'d> + Clone + PartialEq"
))]
#[serde(transparent)]
pub struct Map<TKey, TValue>
where
    TKey: Eq + Hash + Clone + PartialEq,
    TValue: Clone + PartialEq,
{
    #[serde(rename = "entries")]
    pub entries: Vec<HashMap<TKey, TValue>>,
}

// Manual Default implementation for Map
impl<TKey, TValue> Default for Map<TKey, TValue>
where
    TKey: Eq + Hash + Clone + Serialize + for<'d> Deserialize<'d> + PartialEq,
    TValue: Clone + Serialize + for<'d> Deserialize<'d> + PartialEq,
{
    fn default() -> Self {
        Map {
            entries: Vec::new(),
        }
    }
}

impl<TKey, TValue> Map<TKey, TValue>
where
    TKey: Eq + Hash + Clone + Serialize + for<'d> Deserialize<'d> + PartialEq,
    TValue: Clone + Serialize + for<'d> Deserialize<'d> + PartialEq,
{

    /// Initializes a new map
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes a new map with the provided entries
    pub fn from(entries: Vec<(TKey, TValue)>) -> Self {
        let mut vec_entries = Vec::new();
        for (key, value) in entries {
            let mut single_entry = HashMap::new();
            single_entry.insert(key, value);
            vec_entries.push(single_entry);
        }
        Map {
            entries: vec_entries,
        }
    }

    /// Adds the specified entry
    pub fn add(&mut self, key: TKey, value: TValue) {
        let mut single_entry = HashMap::new();
        single_entry.insert(key, value);
        self.entries.push(single_entry);
    }

}