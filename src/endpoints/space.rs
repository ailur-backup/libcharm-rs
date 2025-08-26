use serde::Serialize;
use serde::Deserialize;
use crate::space::Space;

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct CreateInvite {
    pub space: Space,
    pub expiry_time: u64,
}
