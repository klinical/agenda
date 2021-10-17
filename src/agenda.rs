use std::{collections::hash_map::Iter, collections::HashMap, fs::File, io::Read};

use crate::{
    config,
    error::AgendaResult,
    task::{self, Task},
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Agenda {
    tasks: HashMap<String, task::Task>,
    cfg: config::Config,
}

impl Agenda {
    pub fn read_from_file(file: &mut File) -> AgendaResult<Self> {
        let mut data = String::new();
        let _ = file.read_to_string(&mut data)?;

        let agenda: Agenda = serde_json::from_str(&data)?;
        Ok(agenda)
    }

    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            cfg: config::Config::new(),
        }
    }

    pub fn add_task(&mut self, name: String, task: Task) {
        self.tasks.insert(name, task);
    }

    pub fn tasks_iter(&self) -> Iter<String, Task> {
        self.tasks.iter()
    }

    pub fn remove_task(&mut self, taskname: &str) -> Option<(String, Task)> {
        self.tasks.remove_entry(taskname)
    }
}
