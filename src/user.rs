use crate::server::Server;
use ed25519_dalek::ed25519::SignatureBytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct Certificate {
    pub components: CertificateComponents,
    #[serde(with = "crate::serde::signature_bytes")]
    pub signature: SignatureBytes,
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct CertificateComponents {
    pub key: [u8; 32],
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct User {
    pub username: String,
    pub server: Server,
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!("{}@{}", self.username, self.server.domain)
    }
}

impl From<&str> for User {
    fn from(string: &str) -> Self {
        let split: Vec<&str> = string.split('@').collect();
        User {
            username: split[0].to_string(),
            server: Server {
                domain: split[1].to_string(),
            },
        }
    }
}