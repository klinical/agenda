use std::fmt;
use std::fmt::{Display, Formatter};

pub type AgendaResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error),
    SerdeError(serde_json::Error),
    InputError(String),
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IOError(e) => write!(f, "IO Error: {}", e),
            AppError::SerdeError(e) => write!(f, "(De)Serialization Error: {}", e),
            AppError::InputError(info) => write!(f, "Input Error: {}", info),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::SerdeError(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IOError(e)
    }
}
