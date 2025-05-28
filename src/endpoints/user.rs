use crate::request::BlankRequest;
use crate::user::User;
use ed25519_dalek::ed25519::SignatureBytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Create {
    pub user: User,
    pub key: [u8; 32],
}

impl Clone for Create {
    fn clone(&self) -> Self {
        Create {
            user: self.user.clone(),
            key: self.key,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub user: User,
    pub data: BlankRequest,
    #[serde(with = "crate::serde::signature_bytes")]
    pub signature: SignatureBytes,
}

impl Clone for Login {
    fn clone(&self) -> Self {
        Login {
            user: self.user.clone(),
            data: self.data,
            signature: self.signature,
        }
    }
}