use {
    crate::transport::op_code::OpCode,
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload<D> {
    op: OpCode,
    #[serde(alias = "d")]
    data: D,
    #[serde(alias = "t")]
    event: Option<String>,
    #[serde(alias = "s")]
    seq: Option<u64>,
}
