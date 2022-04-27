
use num_enum::{IntoPrimitive, TryFromPrimitive, UnsafeFromPrimitive};
use serde::{Serialize, Deserialize};
use postcard;
use uuid::{uuid, Uuid};

pub struct MinionId(Uuid);

impl MinionId {
    pub fn new() -> MinionId {
        MinionId(Uuid::new_v4())
    }

    pub fn nil() -> MinionId {
        MinionId(Uuid::nil())
    }
}

impl From<Vec<u8>> for MinionId {
    fn from(v: Vec<u8>) -> MinionId {
        MinionId(Uuid::from_slice(&v).unwrap())
    }
}

impl From<MinionId> for Vec<u8> {
    fn from(i: MinionId) -> Vec<u8> {
        match i { MinionId(u) => u }.as_bytes().to_vec()
    }
}



#[derive(Deserialize, IntoPrimitive, Serialize, TryFromPrimitive, UnsafeFromPrimitive, Debug)]
#[repr(u8)]
pub enum MinionOps {
    Auth,
    Exec,
    Ret,
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
