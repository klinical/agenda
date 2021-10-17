use std::{fs::File, io::Read};

use crate::{config, error::AgendaResult, task};
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
}
