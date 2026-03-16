use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    io,
};

#[derive(Debug)]
pub enum PipeError {
    ConnectionFailed(io::Error),
    TransportError(io::Error),
}

impl Error for PipeError {}

impl Display for PipeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PipeError::ConnectionFailed(e) => write!(f, "{}", e),
            PipeError::TransportError(e) => write!(f, "{}", e),
        }
    }
}
