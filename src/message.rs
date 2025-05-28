use crate::room::Room;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct Message {
    pub room: Room,
    // Sender should not be sent when creating a new message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<User>,
    pub content: String,
    // ID should not be sent when creating a new message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct Uuid {
    #[serde(with = "crate::serde::uuid")]
    pub uuid: uuid::Uuid,
}

impl Uuid {
    pub fn new_v4() -> Self {
        Uuid {
            uuid: uuid::Uuid::new_v4(),
        }
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, uuid::Error> {
        Ok(Uuid {
            uuid: uuid::Uuid::from_slice(slice)?,
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.uuid.as_bytes()
    }

    pub fn to_string(&self) -> String {
        self.uuid.to_string()
    }

    pub fn parse_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(Uuid {
            uuid: uuid::Uuid::parse_str(s)?,
        })
    }
}
