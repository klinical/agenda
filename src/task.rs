use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Task {
    description: String,
    name: String,
}

impl Task {
    pub fn from(name: String, description: String) -> Self {
        Task { name, description }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

mod tests {
    #[test]
    fn create_task_from() {
        use crate::task::*;

        let expected = Task {
            name: "test".to_owned(),
            description: "description".to_owned(),
        };

        let taskname = "test";
        let description = "description";
        let new_task = Task::from(taskname.to_owned(), description.to_owned());

        assert_eq!(expected, new_task);
    }
}
