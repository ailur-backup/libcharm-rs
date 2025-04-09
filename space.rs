use crate::server::Server;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub space: Space,
    pub public: bool,
    pub permissions: HashMap<Role, Vec<Permission>>,
    pub hierarchy: Vec<Role>,
}

#[derive(Serialize, Deserialize)]
pub struct Space {
    pub name: String,
    pub server: Server,
}

impl Space {
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.name, self.server.domain)
    }

    pub fn from_string(string: &str) -> Self {
        let split: Vec<&str> = string.split(':').collect();
        Space {
            name: split[0].to_string(),
            server: Server {
                domain: split[1].to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Permission {
    // Permission to modify rooms (add, remove, change role settings for)
    ModifyRooms,
    // Permission to kick users from the space
    Kick,
    // Permission to ban users from the space
    Ban,
    // Permission to invite users to the space
    Invite,
    // Permission to change the space settings (public, role settings, etc.)
    ChangeSettings,
    // Permission to assign roles less than or equal to your current role in the hierarchy
    AssignRoles,
}

type Color = [u8; 3];

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Role {
    pub name: String,
    pub color: Color,
    pub display: bool,
}
