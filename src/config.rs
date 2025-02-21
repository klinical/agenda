use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    file_path: String,
}

impl Config {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: String::from(file_path),
        }
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }
}
