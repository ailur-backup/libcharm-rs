// DISABLED FOR BETA RELEASE, SEE HISTORY.TXT
/*
#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct Config {
    pub space: Space,
    pub public: bool,
    pub hierarchy: Vec<Role>,
    pub default: Role,
    pub members: Vec<Member>,
    pub rooms: Vec<Room>,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
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
    // Permission to delete the space
    Delete,
}

impl Permission {
    pub fn to_int(&self) -> u8 {
        match self {
            Permission::ModifyRooms => 0,
            Permission::Kick => 1,
            Permission::Ban => 2,
            Permission::Invite => 3,
            Permission::ChangeSettings => 4,
            Permission::AssignRoles => 5,
            Permission::Delete => 6,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Permission::ModifyRooms => "ModifyRooms".to_string(),
            Permission::Kick => "Kick".to_string(),
            Permission::Ban => "Ban".to_string(),
            Permission::Invite => "Invite".to_string(),
            Permission::ChangeSettings => "ChangeSettings".to_string(),
            Permission::AssignRoles => "AssignRoles".to_string(),
            Permission::Delete => "Delete".to_string(),
        }
    }

    pub fn from_int(value: u8) -> Self {
        match value {
            0 => Permission::ModifyRooms,
            1 => Permission::Kick,
            2 => Permission::Ban,
            3 => Permission::Invite,
            4 => Permission::ChangeSettings,
            5 => Permission::AssignRoles,
            6 => Permission::Delete,
            _ => panic!("Invalid permission value"),
        }
    }

    pub fn from_string(value: &str) -> Self {
        match value {
            "ModifyRooms" => Permission::ModifyRooms,
            "Kick" => Permission::Kick,
            "Ban" => Permission::Ban,
            "Invite" => Permission::Invite,
            "ChangeSettings" => Permission::ChangeSettings,
            "AssignRoles" => Permission::AssignRoles,
            "Delete" => Permission::Delete,
            _ => panic!("Invalid permission value"),
        }
    }
}

type Color = [u8; 3];

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct Role {
    pub space: Space,
    pub permissions: Vec<Permission>,
    pub name: String,
    pub color: Color,
    pub display: bool,
}

impl Role {
    pub fn to_string(&self) -> String {
        format!("%{}:{}:{}", self.name, self.space.name, self.space.server.domain)
    }
}

impl ToOwned for Role {
    type Owned = Role;

    fn to_owned(&self) -> Self::Owned {
        Role {
            space: self.space.clone(),
            permissions: self.permissions.clone(),
            name: self.name.clone(),
            color: self.color,
            display: self.display,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub struct Member {
    pub user: User,
    pub roles: Vec<Role>,
}

pub const OWNER_PERMISSIONS: Vec<Permission> = vec![
    Permission::ModifyRooms,
    Permission::Kick,
    Permission::Ban,
    Permission::Invite,
    Permission::ChangeSettings,
    Permission::AssignRoles,
    Permission::Delete,
];
*/