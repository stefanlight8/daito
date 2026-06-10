use std::path::Path;

use crate::{error::RpcError, transport::RpcTransport};

pub struct RpcClient {
    pub client_id: String,
    pub transport: RpcTransport,
}

impl RpcClient {
    pub async fn open(
        client_id: String,
        pipe_path: impl AsRef<Path>,
    ) -> Result<RpcClient, RpcError> {
        let transport = RpcTransport::open(pipe_path).await?;

        Ok(Self {
            client_id,
            transport,
        })
    }
}
