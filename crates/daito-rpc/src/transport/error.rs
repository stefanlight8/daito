pub use std::{error::Error, fmt::Display};

use crate::pipe::error::PipeError;

#[derive(Debug)]
pub enum TransportError {
    OpenError(PipeError),
    TransportError(PipeError),
    ParseError(serde_json::Error),
}

impl Error for TransportError {}

impl Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::OpenError(err) => write!(f, "failed to open pipe: {}", err),
            TransportError::TransportError(err) => write!(f, "transport failed: {}", err),
            TransportError::ParseError(err) => write!(f, "failed to parse value: {}", err),
        }
    }
}

impl From<PipeError> for TransportError {
    fn from(value: PipeError) -> Self {
        match value {
            PipeError::OpenError(_) => TransportError::OpenError(value),
            PipeError::TransportError(_) => TransportError::TransportError(value),
        }
    }
}

impl From<serde_json::Error> for TransportError {
    fn from(value: serde_json::Error) -> Self {
        TransportError::ParseError(value)
    }
}
