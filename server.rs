use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Server {
    pub domain: String,
}

#[derive(Serialize, Deserialize)]
pub struct Key {
    pub data: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct Ping {
    pub timestamp: u64,
}