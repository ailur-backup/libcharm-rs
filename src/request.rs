use crate::user::Certificate;
use ed25519_dalek::ed25519::SignatureBytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct Request<T: Serialize> {
    pub data: T,
    #[serde(with = "crate::serde::signature_bytes")]
    pub signature: SignatureBytes,
    pub certificate: Certificate,
}

pub type BlankRequest = u8;
pub const BLANK_REQUEST: BlankRequest = 0;

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct Response<T: Serialize> {
    pub data: T,
    pub status: u16,
}