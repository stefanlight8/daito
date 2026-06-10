pub use std::{error::Error, fmt::Display};

use crate::transport::error::TransportError;

#[derive(Debug)]
pub enum RpcError {
    TransportError(TransportError),
}

impl Error for RpcError {}

impl Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcError::TransportError(err) => write!(f, "{}", err),
        }
    }
}

impl From<TransportError> for RpcError {
    fn from(value: TransportError) -> Self {
        RpcError::TransportError(value)
    }
}
