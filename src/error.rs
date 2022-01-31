use std::io;

use thiserror::Error;

#[allow(unused)]
#[derive(Error, Debug)]
pub enum WshError {
    #[error("IO error")]
    IO(#[from] io::Error),

    #[error("Invalid input format: {0}")]
    Format(String),

    #[error("Invalid environment: {0}")]
    Environment(String),

    #[error("Runtime: {0}")]
    Exit(String),
}

impl From<WshError> for exitcode::ExitCode {
    fn from(e: WshError) -> Self {
        match e {
            WshError::IO(_) => exitcode::IOERR,
            WshError::Format(_) => exitcode::USAGE,
            WshError::Environment(_) => exitcode::CONFIG,
            WshError::Exit(_) => exitcode::OK,
        }
    }
}
