use crate::{config, constants, error::AgendaResult, task::Task};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{fs, fs::File, io::Read, path};
use crate::error::AppError::InputError;
use crate::task::{Priority};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    task_list: TaskList,
    cfg: config::Config,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<Task>,
}

impl Database {
    fn new(file_path: &str) -> Self {
        Self {
            task_list: TaskList::new(),
            cfg: config::Config::new(file_path)
        }
    }

    pub fn open(file_path: &str) -> AgendaResult<Self> {
        if path::Path::new(file_path).exists() {
            let mut data = String::new();
            let _ = File::open(file_path)?.read_to_string(&mut data)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            // Create directory
            if !path::Path::new(constants::F_DIR).exists() {
                fs::create_dir_all(constants::F_DIR)?;
            }
            // Init empty DB and then write it to file
            let db = Self::new(file_path);
            File::create(file_path)?.write_all(serde_json::to_string_pretty(&db)?.as_bytes())?;
            Ok(db)
        }
    }

    fn save(&self) -> AgendaResult<()> {
        let mut file = File::create(&self.cfg.file_path())?;
        Ok(file.write_all(serde_json::to_string_pretty(&self)?.as_bytes())?)
    }

    pub fn add_task(&mut self, task: Task) -> AgendaResult<()> {
        self.task_list.add(task);
        self.save()
    }

    pub fn tasks(&self) -> &Vec<Task> {
        &self.task_list.tasks
    }

    pub fn remove_task(&mut self, task_id: usize) -> AgendaResult<()> {
        self.task_list.remove(task_id);
        self.save()
    }

    pub fn task_mut(&mut self, task_id: usize) -> AgendaResult<&mut Task>  {
        self.task_list.task_mut(task_id).ok_or_else(|| InputError("Task not found".to_string()))
    }

    pub fn update_task_name(&mut self, task_id: usize, name: impl Into<String>) -> AgendaResult<()> {
        self.task_mut(task_id)?.set_name(name.into());
        self.save()
    }

    pub fn update_task_description(&mut self, task_id: usize, description: impl Into<String>) -> AgendaResult<()> {
        self.task_mut(task_id)?.set_description(description.into());
        self.save()
    }

    pub fn update_task_priority(&mut self, task_id: usize, priority: Priority) -> AgendaResult<()> {
        self.task_mut(task_id)?.set_priority(priority);
        self.save()
    }
}

impl TaskList {
    fn new() -> Self {
        TaskList {
            tasks: Vec::new(),
        }
    }

    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn remove(&mut self, id: usize) {
        let _ = self.tasks.remove(id);
    }

    fn task_mut(&mut self, id: usize) -> Option<&mut Task> {
        self.tasks.get_mut(id)
    }
}