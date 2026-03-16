use {
    super::errors::PipeError,
    std::path::Path,
    tokio::io::{AsyncReadExt, AsyncWriteExt},
};

#[cfg(unix)]
use tokio::net::UnixStream;

struct PipeTransport {
    #[cfg(unix)]
    stream: UnixStream,
}

#[cfg(unix)]
impl PipeTransport {
    pub async fn open(path: impl AsRef<Path>) -> Result<Self, PipeError> {
        let stream = UnixStream::connect(path)
            .await
            .map_err(|err| PipeError::ConnectionFailed(err))?;

        Ok(Self { stream })
    }
}

impl PipeTransport {
    pub async fn write_all(&mut self, buf: &[u8]) -> Result<(), PipeError> {
        Ok(self
            .stream
            .write_all(buf)
            .await
            .map_err(|err| PipeError::TransportError(err))?)
    }

    pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), PipeError> {
        self.stream
            .read_exact(buf)
            .await
            .map_err(|err| PipeError::TransportError(err))?;

        Ok(())
    }
}
