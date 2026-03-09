use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum OpCode {
    Dispatch = 0,
    Reconnect = 7,
    Hello = 10,
    Unknown(u8),
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Dispatch,
            7 => Self::Reconnect,
            10 => Self::Hello,
            other => Self::Unknown(other),
        }
    }
}
