pub use std::{error::Error, fmt::Display};

use crate::{api::payload::Payload, transport::error::TransportError};

#[derive(Debug)]
pub enum RpcError {
    TransportError(TransportError),
    RpcError {
        code: usize,
        message: Option<String>,
    },
}

impl Error for RpcError {}

impl Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcError::TransportError(err) => write!(f, "{}", err),
            RpcError::RpcError { code, message } => write!(f, "{}: {:?}", code, message),
        }
    }
}

impl From<TransportError> for RpcError {
    fn from(value: TransportError) -> Self {
        RpcError::TransportError(value)
    }
}

#[derive(Debug)]
pub enum HandshakeError {
    TransportError(TransportError),
    ProtocolError {
        expected: &'static str,
        received: String,
    },
    UnexpectedPayload {
        expected: &'static str,
        received: Payload,
    },
    RpcError {
        code: usize,
        message: Option<String>,
    },
}

impl Error for HandshakeError {}

impl Display for HandshakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandshakeError::TransportError(err) => write!(f, "{}", err),
            HandshakeError::ProtocolError { expected, received } => write!(
                f,
                "unexpected event: expected {}, got {}",
                expected, received
            ),
            HandshakeError::UnexpectedPayload { expected, received } => write!(
                f,
                "unexpected payload: expected {}, got {:?}",
                expected, received
            ),
            HandshakeError::RpcError { code, message } => write!(f, "{}: {:?}", code, message),
        }
    }
}

impl From<TransportError> for HandshakeError {
    fn from(value: TransportError) -> Self {
        HandshakeError::TransportError(value)
    }
}
