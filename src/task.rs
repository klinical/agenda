use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Task {
    name: String,
    description: String,
    priority: self::Priority, //TODO!
}

impl Task {
    pub fn from(name: String, description: String) -> Self {
        Task {
            name,
            description,
            priority: Priority::ImportantNotUrgent, //TODO!
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Priority {
    ImportantUrgent,
    ImportantNotUrgent,
    NotImportantUrgent,
    NotImportantNotUrgent,
}

mod tests {
    #[test]
    fn create_task_from() {
        use crate::task::*;

        let expected = Task {
            name: "test".to_owned(),
            description: "description".to_owned(),
            priority: Priority::ImportantNotUrgent, //TODO!
        };

        let taskname = "test";
        let description = "description";
        let new_task = Task::from(taskname.to_owned(), description.to_owned());

        assert_eq!(expected, new_task);
    }
}
