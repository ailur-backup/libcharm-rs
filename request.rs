use crate::user::Certificate;
use ed25519_dalek::ed25519::SignatureBytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request<T: Serialize> {
    pub data: T,
    #[serde(with = "crate::serde::signature_bytes")]
    pub signature: SignatureBytes,
    pub certificate: Certificate,
}

pub type BlankRequest = u8;
pub const BLANK_REQUEST: BlankRequest = 0;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub status: u16,
}
