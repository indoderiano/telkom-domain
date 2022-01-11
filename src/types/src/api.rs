use serde::{
    Deserialize,
    Serialize,
};

// #[derive(Deserialize, Debug, Clone)]
// pub struct ApiTitle {
//     pub id: String,
//     pub name: String,
//     pub api_type: String,
//     pub identifier: String,
// }

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Scope {
    pub value: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetResourceServersResponseClient {}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiTitle {
    pub resource_server_id: String,
    pub name: String,
    pub is_system: bool,
    pub identifier: String,
    pub scopes: Vec<Scope>, // array of unidentified objects, for now use String
    pub signing_alg: String,
    pub signing_secret: String,
    pub allow_offline_access: bool,
    pub skip_consent_for_variable_first_party_clients: bool,
    pub token_lifetime: u64,
    pub token_lifetime_for_web: u64,
    pub enforce_policies: bool,
    pub token_dialect: String,
    pub client: GetResourceServersResponseClient, // unidentified data type
    pub tenant_id: u32
}

// #[derive(Deserialize, Debug, Clone)]
// pub struct ResponseApiList {
//     pub message: String,
//     pub data: Vec<ApiTitle>
// }

// #[derive(Serialize, Debug, Clone)]
// pub struct ApiCreate {
//     pub name: String,
//     pub identifier: String,
//     pub sign_algorithm: String,
// }
#[derive(Serialize, Debug, Clone)]
pub struct ApiCreate {
    pub name: String,
    pub identifier: String,
    // pub scopes: Vec<Scope>, // array of unidentified objects, for now use String
    pub signing_alg: String,
    // pub signing_secret: String,
    // pub allow_offline_access: bool,
    // pub token_lifetime: u64,
    // pub token_dialect: String,
    // pub skip_consent_for_variable_first_party_clients: bool,
    // pub enforce_policies: bool,
    // pub client: GetResourceServersResponseClient, // unidentified data type
}
impl ApiCreate {
    pub fn new() -> ApiCreate {
        ApiCreate {
            name: String::from(""),
            identifier: String::from(""),
            // scopes: vec![],
            signing_alg: String::from("RS256"),
            // signing_secret: String::from(""),
            // allow_offline_access: false,
            // token_lifetime: 0,
            // token_dialect: String::from(""),
            // skip_consent_for_variable_first_party_clients: false,
            // enforce_policies: false,
            // client: GetResourceServersResponseClient {},
        }
    }
}

// #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
// pub struct ApiDetails {
//     pub id: u8,
//     pub name: String,
//     pub api_id: String,
//     pub api_type: String,
//     pub identifier: String,
//     pub token_exp: u32,
//     pub token_exp_browser: u32,
//     pub sign_algorithm: String,
//     pub rbac: bool,
//     pub permission_acc_token: bool,
//     pub allow_skip_user: bool,
//     pub allow_off_acc: bool,
//     pub tenant_id: String,
// }
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct GetResourceServersByIdResponseClient {}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct ApiDetails {
    pub resource_server_id: String,
    pub name: String,
    pub is_system: bool,
    pub identifier: String,
    pub scopes: Vec<Scope>, // array of unidentified objects, for now use String
    pub signing_alg: String,
    pub signing_secret: String,
    pub allow_offline_access: bool,
    pub skip_consent_for_variable_first_party_clients: bool,
    pub token_lifetime: u64,
    pub token_lifetime_for_web: u64,
    pub enforce_policies: bool,
    pub token_dialect: String,
    pub client: GetResourceServersByIdResponseClient, // unidentified data type
}
impl ApiDetails {
    pub fn new() -> ApiDetails {
        ApiDetails {
            resource_server_id: String::from(""),
            name: String::from(""),
            is_system: false,
            identifier: String::from(""),
            scopes: vec![],
            signing_alg: String::from(""),
            signing_secret: String::from(""),
            allow_offline_access: false,
            skip_consent_for_variable_first_party_clients: false,
            token_lifetime: 0,
            token_lifetime_for_web: 0,
            enforce_policies: false,
            token_dialect: String::from(""),
            client: GetResourceServersByIdResponseClient {},
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseApiDetails {
    pub message: String,
    pub data: ApiDetails
}


#[derive(Deserialize, Debug, Clone)]
pub struct Application {
    pub name: String,
    pub client_id: String,
    pub status: bool,
}