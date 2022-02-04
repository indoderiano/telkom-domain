use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum AppRoute {
    // MEMBER PAGES
    #[to = "/getting-started"]
    GettingStarted,
    #[to = "/activity"]
    Activity,
    #[to = "/apis/{resource_server_id}/settings"]
    ApisSettings {
        resource_server_id: String,
    },
    #[to = "/apis"]
    ApisHome,
    #[to = "/{tenant_id}/applications/{app_id}/settings"]
    ApplicationSettings { tenant_id: String, app_id: String },
    #[to = "/{tenant_id}/applications"]
    ApplicationHome { tenant_id: String },
    #[to = "/authentication/database/settings"]
    DatabaseSettings,
    #[to = "/authentication/database/create"]
    DbCreate,
    #[to = "/authentication/database"]
    DatabaseHome,
    #[to = "/authentication/passwordless"]
    AuthPasswordless,
    #[to = "/sso/create-sso"]
    CreateSso,
    #[to = "/sso"]
    SsoHome,
    #[to = "/social/create"]
    SocialCreate,
    #[to = "/social/settings"]
    SocialSettings,
    #[to = "/social"]
    SocialHome,
    #[to = "/user-management/roles/settings/{role_id}"]
    RoleSettings { role_id: String },
    #[to = "/user-management/roles"]
    RolesHome,
    #[to = "/{tenant_id}/users/{user_id}/{id}"]
    UserSettings {
        tenant_id: String,
        user_id: String,
        id: u32,
    },
    #[to = "/{tenant_id}/users"]
    UsersHome { tenant_id: String },
    #[to = "/enterprise/google-app/create"]
    EnterpriseGoogleCreate,
    #[to = "/enterprise/google-app"]
    EnterpriseGoogle,
    #[to = "/enterprise"]
    EnterpriseHome,
    #[to = "/tenant"]
    SettingsHome,
    #[to = "/{tenant_id}/logs/{log_id}"]
    LogDetails { tenant_id: String, log_id: String },
    #[to = "/{tenant_id}/logs"]
    LogsHome { tenant_id: String },
    #[to = "/{tenant_id}/profile"]
    Profile { tenant_id: String },

    // NOT LOGGED IN PAGES
    #[to = "/login/password"]
    RequestPassPage,
    #[to = "/login"]
    LoginPage,
    #[to = "/register"]
    RegisterPage,
    #[to = "/"]
    Home,
}