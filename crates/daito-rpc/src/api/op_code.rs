#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Hello = 0,
    Dispatch = 1,
    Unknown(u32),
}

impl OpCode {
    pub fn to_u32(&self) -> u32 {
        match *self {
            OpCode::Hello => 0,
            OpCode::Dispatch => 1,
            OpCode::Unknown(op) => op,
        }
    }
}

impl From<u32> for OpCode {
    fn from(value: u32) -> Self {
        match value {
            0 => OpCode::Hello,
            1 => OpCode::Dispatch,
            other => OpCode::Unknown(other),
        }
    }
}
