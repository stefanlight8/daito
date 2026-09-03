use std::path::Path;

use daito_api::activity::Activity;

use crate::{
    api::{
        command::{CommandArgs, CommandKind},
        op_code::OpCode,
        payload::Payload,
    },
    error::{HandshakeError, RpcError},
    transport::RpcTransport,
    utils::get_nonce,
};

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

    pub async fn handshake(&mut self) -> Result<(), HandshakeError> {
        self.transport
            .send(
                OpCode::Hello,
                &Payload::Handshake {
                    version: 1,
                    client_id: self.client_id.clone(),
                },
            )
            .await?;

        let (_, payload) = self.transport.receive::<Payload>().await?;
        match payload {
            Payload::Event { evt: Some(evt), .. } => {
                if evt == "READY" {
                    Ok(())
                } else {
                    Err(HandshakeError::ProtocolError {
                        expected: "READY",
                        received: evt,
                    })
                }
            }
            Payload::Error { code, message } => Err(HandshakeError::RpcError { code, message }),
            payload => Err(HandshakeError::UnexpectedPayload {
                expected: "Event",
                received: payload,
            }),
        }
    }

    pub async fn send_command(
        &mut self,
        cmd: CommandKind,
        args: Option<CommandArgs>,
    ) -> Result<(), RpcError> {
        self.transport
            .send(
                OpCode::Dispatch,
                &Payload::Event {
                    cmd,
                    args,
                    evt: None,
                    data: None,
                    nonce: Some(get_nonce()),
                },
            )
            .await?;

        Ok(())
    }

    pub async fn set_activity(&mut self, pid: u32, activity: Activity) -> Result<(), RpcError> {
        self.send_command(
            CommandKind::SetActivity,
            Some(CommandArgs::SetActivity {
                pid,
                activity: Some(activity),
            }),
        )
        .await?;

        let (_, payload) = self.transport.receive::<Payload>().await?;
        match payload {
            Payload::Error { code, message } => Err(RpcError::RpcError { code, message }),
            _ => Ok(()),
        }
    }

    pub async fn clear_activity(&mut self, pid: u32) -> Result<(), RpcError> {
        self.send_command(
            CommandKind::SetActivity,
            Some(CommandArgs::SetActivity {
                pid,
                activity: None,
            }),
        )
        .await?;

        let (_, payload) = self.transport.receive::<Payload>().await?;
        match payload {
            Payload::Error { code, message } => Err(RpcError::RpcError { code, message }),
            _ => Ok(()),
        }
    }
}
