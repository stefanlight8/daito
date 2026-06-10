use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::command::Command;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Payload {
    Handshake {
        #[serde(alias = "v")]
        version: u32,
        client_id: String,
    },
    Event {
        cmd: Command,
        evt: Option<String>,
        data: Option<Value>,
        args: Option<Value>,
        nonce: Option<String>,
    },
    Error {
        code: usize,
        message: Option<String>,
    },
}
