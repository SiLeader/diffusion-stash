use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    NotFound,
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "Not found"),
            Error::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}
