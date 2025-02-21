use crate::{config, error::AgendaResult, task::Task, F_DIR};
use serde::{Deserialize, Deserializer, Serialize};
use std::io::Write;
use std::{collections::hash_map::Iter, collections::HashMap, fs, fs::File, io::Read, path};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    file_path: String,
    task_list: TaskList,
    cfg: config::Config,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskList {
    tasks: HashMap<String, Task>,
}

impl Database {
    pub fn open(file_path: &str) -> AgendaResult<Self> {
        if path::Path::new(file_path).exists() {
            let mut data = String::new();
            let _ = File::open(file_path)?.read_to_string(&mut data)?;
            Ok(Database {
                file_path: String::from(file_path),
                task_list: serde_json::from_str(&data)?,
                cfg: config::Config::new()
            })
        } else {
            if !path::Path::new(F_DIR).exists() {
                fs::create_dir_all(F_DIR)?;
            }
            File::create(file_path)?;
            Ok(Database {
                file_path: String::from(file_path),
                task_list: TaskList::new(),
                cfg: config::Config::new()
            })
        }
    }

    pub fn save(&self) -> AgendaResult<()> {
        let mut file = File::create(&self.file_path)?;
        Ok(file.write_all(serde_json::to_string_pretty(&self)?.as_bytes())?)
    }

    pub fn add_task(&mut self, name: String, task: Task) {
        self.task_list.insert(name, task);
    }

    pub fn tasks(&self) -> &HashMap<String, Task> {
        &self.task_list.tasks
    }

    pub fn remove_task(&mut self, taskname: &str) -> Option<(String, Task)> {
        self.task_list.remove_entry(taskname)
    }

    pub fn task(&mut self, key: &str) -> Option<Task> {
        self.task_list.task(key)
    }
}

impl TaskList {
    fn new() -> Self {
        TaskList {
            tasks: HashMap::new(),
        }
    }

    fn insert(&mut self, name: String, task: Task) {
        self.tasks.insert(name, task);
    }

    fn tasks(&self) -> &HashMap<String, Task> {
        &self.tasks
    }

    fn remove_entry(&mut self, taskname: &str) -> Option<(String, Task)> {
        self.tasks.remove_entry(taskname)
    }

    fn task(&self, key: &str) -> Option<Task> {
        self.tasks.get(key).cloned()
    }
}