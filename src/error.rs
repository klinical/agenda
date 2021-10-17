use std::fmt;

pub type AgendaResult<T> = Result<T, AgendaError>;

#[derive(Debug, Clone)]
pub struct AgendaError {
    kind: String,
    message: String,
}

impl fmt::Display for AgendaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} error: {}", self.kind, self.message)
    }
}

impl From<serde_json::Error> for AgendaError {
    fn from(e: serde_json::Error) -> Self {
        AgendaError {
            kind: String::from("serde"),
            message: e.to_string(),
        }
    }
}

impl From<std::io::Error> for AgendaError {
    fn from(e: std::io::Error) -> Self {
        AgendaError {
            kind: String::from("io"),
            message: e.to_string(),
        }
    }
}
