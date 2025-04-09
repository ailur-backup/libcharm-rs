use crate::server::Server;
use ed25519_dalek::ed25519::SignatureBytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Certificate {
    pub components: CertificateComponents,
    #[serde(with = "crate::serde::signature_bytes")]
    pub signature: SignatureBytes,
}

#[derive(Serialize, Deserialize)]
pub struct CertificateComponents {
    pub key: [u8; 32],
    pub user: User,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub server: Server,
}

impl User {
    pub fn to_string(&self) -> String {
        format!("{}@{}", self.username, self.server.domain)
    }

    pub fn from_string(string: &str) -> Self {
        let split: Vec<&str> = string.split('@').collect();
        User {
            username: split[0].to_string(),
            server: Server {
                domain: split[1].to_string(),
            },
        }
    }
}
