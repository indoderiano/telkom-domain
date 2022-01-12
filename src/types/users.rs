use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct GetUsersByIdResponseAppMetadata {}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct GetUsersByIdResponseUserMetadata {}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UserTitle {
    // pub id: u32,
    pub user_id: String,
    pub email: String,
    pub email_verified: bool,
    pub username: String,
    pub phone_number: String,
    pub phone_verified: bool,
    pub created_at: String,
    pub updated_at: String,
    pub identities: Vec<UserIdentity>,
    pub app_metadata: GetUsersByIdResponseAppMetadata,
    pub user_metadata: GetUsersByIdResponseUserMetadata,
    pub picture: String,
    pub name: String,
    pub nickname: String,
    pub multifactor: Vec<String>,
    pub last_ip: String,
    pub last_login: String,
    pub logins_count: u32,
    pub blocked: bool,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseUsersList {
    pub message: String,
    pub data: Vec<UserTitle>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PostUsersByIdResponseAppMetadata {}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PostUsersByIdResponseUserMetadata {}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UserCreate {
    pub email: String,
    // pub phone_number: String,
    // pub user_metadata: PostUsersByIdResponseUserMetadata,
    // pub blocked: bool,
    // pub email_verified: bool,
    // pub phone_verified: bool,
    // pub app_metadata: PostUsersByIdResponseAppMetadata,
    // pub given_name: String,
    // pub family_name: String,
    // pub name: String,
    // pub nickname: String,
    // pub picture: String,
    // pub user_id: String,
    pub connection: String,
    pub password: String,
    // pub verify_email: String,
    // pub username: String,
}

impl UserCreate {
    pub fn new() -> UserCreate {
        UserCreate {
            email: String::from(""),
            // phone_number: String::from(""),
            // user_metadata: PostUsersByIdResponseUserMetadata {},
            // blocked: false,
            // email_verified: false,
            // phone_verified: false,
            // app_metadata: PostUsersByIdResponseAppMetadata {},
            // given_name: String::from(""),
            // family_name: String::from(""),
            // name: String::from(""),
            // nickname: String::from(""),
            // picture: String::from(""),
            // user_id: String::from(""),
            connection: String::from(""),
            password: String::from(""),
            // verify_email: String::from(""),
            // username: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UserDetails {
    // pub id : u32,
    pub user_id: String,
    pub email: String,
    pub email_verified: bool,
    pub username: String,
    pub phone_number: String,
    pub phone_verified: bool,
    pub created_at: String,
    pub updated_at: String,
    pub identities: Vec<UserIdentity>,
    pub app_metadata: GetUsersByIdResponseAppMetadata,
    pub user_metadata: GetUsersByIdResponseUserMetadata,
    pub picture: String,
    pub name: String,
    pub nickname: String,
    pub multifactor: Vec<String>,
    pub last_ip: String,
    pub last_login: String,
    pub logins_count: u32,
    pub blocked: bool,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ResponseUserDetails{
    pub message: String,
    pub data: UserDetails
}

impl UserDetails {
    pub fn new() -> UserDetails {
        UserDetails {
            // id: 0,
            user_id: String::from(""),
            email: String::from(""),
            email_verified: false,
            username: String::from(""),
            phone_number: String::from(""),
            phone_verified: false,
            created_at: String::from(""),
            updated_at: String::from(""),
            identities: vec![],
            app_metadata: GetUsersByIdResponseAppMetadata {},
            user_metadata: GetUsersByIdResponseUserMetadata {},
            picture: String::from(""),
            name: String::from(""),
            nickname: String::from(""),
            multifactor: vec![],
            last_ip: String::from(""),
            last_login: String::from(""),
            logins_count: 0,
            blocked: false,
            given_name: String::from(""),
            family_name: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UserIdentity {
    pub connection: String,
    pub user_id: String,
    pub provider: String,
    pub is_social: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseUserLogLists {
    pub data: Vec<UserLogDetails>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct UserLogDetails {
    pub date: String,
    pub type_detail: String,
    pub description: String,
    pub connection: String,
    pub connection_id: String,
    pub client_id: String,
    pub client_name: String,
    pub ip: String,
    pub hostname: String,
    pub user_id: String,
    pub user_name: String,
    pub audience: String,
    pub scope: String,
    pub strategy: String,
    pub log_id: String,
    pub is_mobile: bool,
    pub details: GetLogsByUserResponseDetails,
    pub user_agent: String,
    pub location_info: GetLogsByUserLocationInfo,
}

impl UserLogDetails {
    pub fn new() -> UserLogDetails {
        UserLogDetails {
            date: String::from(""),
            type_detail: String::from(""),
            description: String::from(""),
            connection: String::from(""),
            connection_id: String::from(""),
            client_id: String::from(""),
            client_name: String::from(""),
            ip: String::from(""),
            hostname: String::from(""),
            user_id: String::from(""),
            user_name: String::from(""),
            audience: String::from(""),
            scope: String::from(""),
            strategy: String::from(""),
            log_id: String::from(""),
            is_mobile: false,
            details: GetLogsByUserResponseDetails {},
            user_agent: String::from(""),
            location_info: GetLogsByUserLocationInfo {
                country_code: String::from(""),
                country_code_3: String::from(""),
                country_name: String::from(""),
                city_name: String::from(""),
                latitude: String::from(""),
                longitude: String::from(""),
                time_zone: String::from(""),
                continent_code: String::from(""),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct GetLogsByUserResponseDetails {}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct GetLogsByUserLocationInfo {
    pub country_code: String,
    pub country_code_3: String,
    pub country_name: String,
    pub city_name: String,
    pub latitude: String,
    pub longitude: String,
    pub time_zone: String,
    pub continent_code: String,
}


#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UserRole{
    pub id: String,
    pub name: String,
    pub description: String,
}

// impl UserRoles {
//     pub fn new() -> UserRoles {
//         UserRoles {
//             id: String::from(""),
//             name: String::from(""),
//             description: String::from(""),
//         }
//     }
// }


#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UserPermissions{
    pub resource_server_identifier: String,
    pub permission_name: String,
    pub resource_server_name: String,
    pub description: String
}

impl UserPermissions{
    pub fn new() -> UserPermissions {
        UserPermissions{
            resource_server_identifier: String::from(""),
            permission_name: String::from(""),
            resource_server_name: String::from(""),
            description: String::from(""),
        }
    }
}
