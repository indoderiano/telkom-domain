use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct ResponseRoleDelete {
}

impl Role {
    pub fn new() -> Role {
        Role {
            id: String::from(""),
            name: String::from(""),
            description: String::from(""),
        }
    }
}




#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct RolePermission{
    pub resource_server_identifier: String,
    pub permission_name: String,
    pub resource_server_name: String,
    pub desciption: String
}

impl RolePermission{
    pub fn new() -> RolePermission {
        RolePermission{
            resource_server_identifier: String::from(""),
            permission_name: String::from(""),
            resource_server_name: String::from(""),
            desciption: String::from(""),
        }
    }
}