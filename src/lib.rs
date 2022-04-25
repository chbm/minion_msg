
use num_enum::{IntoPrimitive, TryFromPrimitive, UnsafeFromPrimitive};
use serde::{Serialize, Deserialize};
use postcard;

#[derive(Deserialize, IntoPrimitive, Serialize, TryFromPrimitive, UnsafeFromPrimitive, Debug)]
#[repr(u8)]
pub enum MinionOps {
    Auth,
    Exec,
}

#[derive(Deserialize, Serialize)]
pub struct MinionMsg {
    pub op: MinionOps,
    pub payload: Vec<u8>,
}

pub fn from_bytes(input: &[u8]) -> postcard::Result<MinionMsg> {
    postcard::from_bytes(input)
}

pub fn to_vec(s: &MinionMsg) -> postcard::Result<Vec<u8>> {
    postcard::to_allocvec(s)
}
