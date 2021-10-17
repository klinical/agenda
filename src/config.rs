use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }
}
