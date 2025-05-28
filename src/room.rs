use serde::{Deserialize, Serialize};

/*
pub type Permissions = Vec<Permission>;
pub type RolePermissions = HashMap<Role, Permissions>;

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

impl Permission {
    pub fn to_int(&self) -> u8 {
        match self {
            Permission::View => 0,
            Permission::Send => 1,
            Permission::Delete => 2,
            Permission::Embed => 3,
            Permission::React => 4,
            Permission::DeleteOther => 5,
            Permission::RemoveReaction => 6,
            Permission::ChangeRole => 7,
            Permission::ChangeSettings => 8,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Permission::View => "View".to_string(),
            Permission::Send => "Send".to_string(),
            Permission::Delete => "Delete".to_string(),
            Permission::Embed => "Embed".to_string(),
            Permission::React => "React".to_string(),
            Permission::DeleteOther => "DeleteOther".to_string(),
            Permission::RemoveReaction => "RemoveReaction".to_string(),
            Permission::ChangeRole => "ChangeRole".to_string(),
            Permission::ChangeSettings => "ChangeSettings".to_string(),
        }
    }

    pub fn from_int(value: u8) -> Self {
        match value {
            0 => Permission::View,
            1 => Permission::Send,
            2 => Permission::Delete,
            3 => Permission::Embed,
            4 => Permission::React,
            5 => Permission::DeleteOther,
            6 => Permission::RemoveReaction,
            7 => Permission::ChangeRole,
            8 => Permission::ChangeSettings,
            _ => panic!("Invalid permission value"),
        }
    }

    pub fn from_string(value: &str) -> Self {
        match value {
            "View" => Permission::View,
            "Send" => Permission::Send,
            "Delete" => Permission::Delete,
            "Embed" => Permission::Embed,
            "React" => Permission::React,
            "DeleteOther" => Permission::DeleteOther,
            "RemoveReaction" => Permission::RemoveReaction,
            "ChangeRole" => Permission::ChangeRole,
            "ChangeSettings" => Permission::ChangeSettings,
            _ => panic!("Invalid permission value"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub room: Room,
    pub space_config: space::Config,
    pub encrypted: bool,
    pub federated: bool,
    pub roles: Vec<Role>,
}

pub struct Role {
    pub role: space::Role,
    pub permissions: Permissions,
}

const OWNER_PERMISSIONS: Permissions = vec![
    Permission::View,
    Permission::Send,
    Permission::Delete,
    Permission::Embed,
    Permission::React,
    Permission::DeleteOther,
    Permission::RemoveReaction,
    Permission::ChangeRole,
    Permission::ChangeSettings,
];

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
*/


#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct Room {
    pub name: String,
    // DISABLED FOR BETA RELEASE, SEE HISTORY.TXT
    // pub space: Space,
}

impl Room {
    pub fn to_string(&self) -> String {
        format!("#{}", self.name)
    }

    pub fn from_string(string: &str) -> Self {
        Room {
            name: string.to_string(),
            // DISABLED FOR BETA RELEASE, SEE HISTORY.TXT
            // space: Space::from_string(string),
        }
    }
}
