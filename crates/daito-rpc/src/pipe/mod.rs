pub mod error;

use std::path::Path;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use crate::pipe::error::PipeError;

// TODO(win): windows support via namedpipeclient
pub struct Pipe {
    stream: UnixStream,
}

impl Pipe {
    pub async fn open(path: impl AsRef<Path>) -> Result<Pipe, PipeError> {
        let stream = UnixStream::connect(path)
            .await
            .map_err(PipeError::OpenError)?;

        Ok(Self { stream })
    }

    pub async fn write_all(&mut self, buf: &[u8]) -> Result<(), PipeError> {
        self.stream
            .write_all(buf)
            .await
            .map_err(PipeError::TransportError)
    }

    pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<usize, PipeError> {
        self.stream
            .read_exact(buf)
            .await
            .map_err(PipeError::TransportError)
    }
}
