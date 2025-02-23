use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    name: String,
    description: String,
    priority: Priority,
}

impl Task {
    pub fn new(name: impl Into<String>, description: impl Into<String>, priority: Priority) -> Self {
        Self { name: name.into(), description: description.into(), priority }
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn priority(&self) -> String {
        self.priority.to_string()
    }

    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description
    }

    pub fn set_priority(&mut self, new_priority: Priority) {
        self.priority = new_priority;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.name, self.description)
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn values() -> &'static [Priority] {
        &[Priority::Low, Priority::Medium, Priority::High]
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}