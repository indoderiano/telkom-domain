use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Debug, Clone)]
pub struct AppList {
    pub name: String,
    pub client_id: String,
    pub app_type: String,
    // pub logo_uri: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct RefreshToken {
    pub expiration_type: String,
    pub leeway: i32,
    pub infinite_token_lifetime: bool,
    pub infinite_idle_token_lifetime: bool,
    pub token_lifetime: i32,
    pub idle_token_lifetime: i32,
    pub rotation_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct SigningKeys {
    pub cert: String,
    pub pkcs7: String,
    pub subject: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct JwtConfiguration {
    pub lifetime_in_seconds: i32,
    pub secret_encoded: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct AppDetails {
    pub tenant: String,
    pub global: bool,
    pub description: String,
    pub is_token_endpoint_ip_header_trusted: bool,
    pub name: String,
    pub is_first_party: bool,
    pub oidc_conformant: bool,
    pub sso_disabled: bool,
    pub cross_origin_auth: bool,
    pub refresh_token: RefreshToken,
    pub encrypted: bool,
    pub allowed_clients: Vec<String>,
    pub callbacks: Vec<String>,
    pub allowed_origins: Vec<String>,
    pub web_origins: Vec<String>,
    pub client_aliases: Vec<String>,
    pub allowed_logout_urls: Vec<String>,
    pub signing_keys: Vec<SigningKeys>,
    pub client_id: String,
    pub callback_url_template: bool,
    pub client_secret: String,
    pub jwt_configuration: JwtConfiguration,
    pub token_endpoint_auth_method: String,
    pub app_type: String,
    pub grant_types: Vec<String>,
    pub custom_login_page_on: bool,
    pub logo_uri: String,
    pub sso: bool,
    pub cross_origin_loc: String,
    pub custom_login_page: String,
    pub custom_login_page_preview: String,
    pub form_template: String,
    pub initiate_login_uri: String,
    pub organization_usage: String,
    pub organization_require_behavior: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct AppCreate {
    pub name: String,
    pub app_type: String,
}

impl AppCreate {
    pub fn new() -> AppCreate {
        AppCreate {
            name: String::from(""),
            app_type: String::from("spa"),
            // signing_alg: String::from("RS256"),
        }
    }
}