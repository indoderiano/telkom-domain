use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LocalStorage {
    pub email: Option<String>,
    pub username: Option<String>,
    pub token: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseLogin {
    pub email: String,
    pub username: String,
    pub token: String,
}

pub const LOCALSTORAGE_KEY: &str = "telkom-domain";


#[derive(Deserialize, Debug, Clone)]
pub struct ResponseMessage {
    pub message: String,
    pub data: String,
}



pub mod application;
pub mod api;
pub mod login;
pub mod users;
pub mod settings;
pub mod roles;