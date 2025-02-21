use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    description: String,
    priority: self::Priority,
}

impl Task {
    pub fn from(description: String, priority: String) -> Result<Self, PriorityError> {
        Ok(Task {
            description,
            priority: Priority::from(priority)?,
        })
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn priority(&self) -> String {
        self.priority.to_string()
    }

    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description
    }

    pub fn set_priority(&mut self, new_priority: String) -> Result<(), PriorityError> {
        self.priority = Priority::from(new_priority)?;

        Ok(())
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Priority {
    ImportantUrgent,
    ImportantNotUrgent,
    NotImportantUrgent,
    NotImportantNotUrgent,
}

#[derive(Debug)]
pub struct PriorityError;

impl fmt::Display for PriorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid/unrecognized priority.")
    }
}

impl Priority {
    fn from(priority: String) -> Result<Self, PriorityError> {
        match priority.to_lowercase().as_str() {
            "important and urgent" | "iu" => Ok(Self::ImportantUrgent),
            "important and not urgent" | "inu" => Ok(Self::ImportantNotUrgent),
            "not important and urgent" | "niu" => Ok(Self::NotImportantUrgent),
            "not important and not urgent" | "ninu" => Ok(Self::NotImportantNotUrgent),
            _ => Err(PriorityError),
        }
    }
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match *self {
            Self::ImportantUrgent => "Important and Urgent".to_owned(),
            Self::ImportantNotUrgent => "Important and Not Urgent".to_owned(),
            Self::NotImportantUrgent => "Not Important and Urgent".to_owned(),
            Self::NotImportantNotUrgent => "Not Important and Not Urgent".to_owned(),
        }
    }
}

mod tests {
    #[test]
    fn create_task_from() {
        use crate::task::*;

        let expected = Task {
            description: "description".to_owned(),
            priority: Priority::ImportantNotUrgent,
        };

        let description = "description";
        let new_task = Task::from(description.to_owned(), "inu".to_owned()).unwrap();

        assert_eq!(expected, new_task);
    }
}
