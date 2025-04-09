use crate::server::Server;
use crate::space;
use crate::space::{Role, Space};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub enum Permission {
    // Permission to view the room
    View,
    // Permission to send messages in the room
    Send,
    // Permission to delete your own messages
    Delete,
    // Permission to embed links in the room
    Embed,
    // Permission to react to messages in the room
    React,
    // Permission to delete other users messages
    DeleteOther,
    // Permission to remove the reactions of other users from messages
    RemoveReaction,
    // Permission to change the role settings of the room
    ChangeRole,
    // Permission to change the room settings (federation, encryption, etc.)
    ChangeSettings,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub room: Room,
    pub space_config: space::Config,
    pub encrypted: bool,
    pub federated: bool,
    pub permissions: HashMap<Role, Vec<Permission>>,
}

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub space: Space,
}

impl Room {
    pub fn to_string(&self) -> String {
        format!(
            "#{}:{}:{}",
            self.name, self.space.name, self.space.server.domain
        )
    }

    pub fn from_string(string: &str) -> Self {
        let split: Vec<&str> = string.split(':').collect();
        Room {
            name: split[0].to_string(),
            space: Space {
                name: split[1].to_string(),
                server: Server {
                    domain: split[2].to_string(),
                },
            },
        }
    }
}
