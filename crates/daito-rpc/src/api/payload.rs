use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::command::{CommandArgs, CommandKind};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Payload {
    Handshake {
        #[serde(alias = "v")]
        version: u32,
        client_id: String,
    },
    Event {
        cmd: CommandKind,
        evt: Option<String>,
        data: Option<Value>,
        args: Option<CommandArgs>,
        nonce: Option<String>,
    },
    Error {
        code: usize,
        message: Option<String>,
    },
}
