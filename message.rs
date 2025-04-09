use crate::room::Room;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub room: Room,
    pub sender: User,
    pub content: String,
    #[serde(with = "crate::serde::uuid")]
    pub id: uuid::Uuid,
}
