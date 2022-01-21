use serde::{
    Deserialize,
    Serialize,
};

// #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
// pub struct ChangePassword {
//     pub enabled: bool,
//     pub html: String,
// }

// #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
// pub struct GuardianMfaPage {
//     pub enabled: bool,
//     pub html: String,
// }

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct ErrorPage {
    pub html: String,
    pub show_log_link: bool,
    pub url: String,
}

// #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
// pub struct DeviceFlow {
//     pub charset: String,
//     pub mask: String,
// }

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Flags {
    // pub change_pwd_flow_v1: bool,
    // pub enable_apis_section: bool,
    pub disable_impersonation: bool,
    // pub enable_client_connections: bool,
    // pub enable_pipeline2: bool,
    // pub allow_legacy_delegation_grant_types: bool,
    // pub allow_legacy_ro_grant_types: bool,
    // pub allow_legacy_tokeninfo_endpoint: bool,
    // pub enable_legacy_profile: bool,
    // pub enable_idtoken_api2: bool,
    // pub enable_public_signup_user_exists_error: bool,
    pub enable_sso: bool,
    pub allow_changing_enable_sso: bool,
    pub disable_clickjack_protection_headers: bool,
    // pub no_disclose_enterprise_connections: bool,
    // pub enforce_client_authentication_on_passwordless_start: bool,
    // pub enable_adfs_waad_email_verification: bool,
    pub revoke_refresh_token_grant: bool,
    // pub dashboard_log_streams_next: bool,
    // pub dashboard_insights_view: bool,
}
// #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
// pub struct SessionCookie {
//     pub mode: String
// }

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct TenantSettings {
    // pub change_password: ChangePassword,
    // pub guardian_mfa_page: GuardianMfaPage,
    // pub default_audience: String,
    // pub default_directory: String,
    // pub error_page: ErrorPage,
    // pub device_flow: DeviceFlow,
    pub flags: Flags,
    // pub friendly_name: String,
    // pub picture_url: String,
    // pub support_email: String,
    // pub support_url: String,
    // pub allowed_logout_urls: Vec<String>,
    // pub session_lifetime: u64,
    // pub idle_session_lifetime: u64,
    pub sandbox_version: String,
    pub sandbox_versions_available: Vec<String>,
    // pub default_redirection_uri: String,
    pub enabled_locales: Vec<String>,
    // pub session_cookie: SessionCookie,
}


impl TenantSettings {
    pub fn new() -> TenantSettings {
        TenantSettings {
            // change_password: ChangePassword {
            //     enabled: false,
            //     html: String::from(""),
            // },
            // guardian_mfa_page: GuardianMfaPage {
            //     enabled: false,
            //     html: String::from(""),
            // },
            // default_audience: String::from(""),
            // default_directory: String::from(""),
            // error_page: ErrorPage {
            //     html: String::from(""),
            //     show_log_link: false,
            //     url: String::from(""),
            // },
            // device_flow: DeviceFlow {
            //     charset: String::from(""),
            //     mask: String::from(""),
            // },
            flags: Flags {
                // change_pwd_flow_v1: false,
                // enable_apis_section: false,
                disable_impersonation: false,
                // enable_client_connections: false,
                // enable_pipeline2: false,
                // allow_legacy_delegation_grant_types: false,
                // allow_legacy_ro_grant_types: false,
                // allow_legacy_tokeninfo_endpoint: false,
                // enable_legacy_profile: false,
                // enable_idtoken_api2: false,
                // enable_public_signup_user_exists_error: false,
                enable_sso: false,
                allow_changing_enable_sso: false,
                disable_clickjack_protection_headers: false,
                // no_disclose_enterprise_connections: false,
                // enforce_client_authentication_on_passwordless_start: false,
                // enable_adfs_waad_email_verification: false,
                revoke_refresh_token_grant: false,
                // dashboard_log_streams_next: false,
                // dashboard_insights_view: false,
            },
            // friendly_name: String::from(""),
            // picture_url: String::from(""),
            // support_email: String::from(""),
            // support_url: String::from(""),
            // allowed_logout_urls: vec![String::from("")],
            // session_lifetime: 0,
            // idle_session_lifetime: 0,
            sandbox_version: String::from(""),
            sandbox_versions_available: vec![String::from(""),String::from("")],
            // default_redirection_uri: String::from(""),
            enabled_locales: vec![String::from("")],
            // session_cookie: SessionCookie {
            //     mode: String::from(""),
            // },
        }
    }
}







// Tenant Members

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TenantMember {
    pub username: String,
    pub email: String,
    pub connection: String,
    pub roles: String,
    pub is_mfa: bool
}