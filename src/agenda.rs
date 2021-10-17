use std::{fs::File, io::Read, slice::Iter};

use crate::{
    config,
    error::AgendaResult,
    task::{self, Task},
};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Agenda {
    tasks: Vec<task::Task>,
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
            tasks: Vec::new(),
            cfg: config::Config::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn tasks_iter(&self) -> Iter<Task> {
        self.tasks.iter()
    }
}
