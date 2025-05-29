use std::fmt;
use std::process::Child;

// Command
pub struct CommandHandle {
    pub child: Option<Child>,
    pub ret: Option<i32>,
}

// Errors
#[derive(Debug)]
pub enum CheckerError {
    Io(std::io::Error),
    NotFound,
    Other(&'static str),
}

impl From<std::io::Error> for CheckerError {
    fn from(err: std::io::Error) -> CheckerError {
        CheckerError::Io(err)
    }
}

impl fmt::Display for CheckerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckerError::Io(e) => write!(f, "IO error: {}", e),
            CheckerError::NotFound => write!(f, "Not found"),
            CheckerError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for CheckerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CheckerError::Io(ref err) => Some(err),
            CheckerError::NotFound => None,
            CheckerError::Other(_) => None,
        }
    }
}
