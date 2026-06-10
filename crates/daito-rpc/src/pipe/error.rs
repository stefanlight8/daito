use std::{error::Error, fmt::Display};

use tokio::io;

#[derive(Debug)]
pub enum PipeError {
    OpenError(io::Error),
    TransportError(io::Error),
}

impl Error for PipeError {}

impl Display for PipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipeError::OpenError(err) => write!(f, "failed to open pipe: {}", err),
            PipeError::TransportError(err) => write!(f, "transport failed: {}", err),
        }
    }
}
