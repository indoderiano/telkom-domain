use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
    }
};
use serde::{
    Serialize,
};
use types::{
    settings::{ TenantSettings, Flags },
    ResponseMessage,
};
use yew_router::service::RouteService;
use crate::configs::server::API_URL;
use loading::Loading;



pub enum StateError {
    GetSettings,
    UpdateLoginLogout, 
    UpdateLoginSession,
    UpdateDeviceFlow,
    UpdateSettings,
    DeleteTenant,
}

pub enum Data {
    // AllowedLogoutUrls,
    // DefaultRedirectionUri,
    // SessionCookieMode,
    // IdleSessionLifetime,
    // SessionLifetime,
    // DeviceFlowCharset,
    // DeviceFlowMask,
    // Global client id
    // Global client secret
    // FlagsChangePwdFlowV1,
    // FlagsEnableApisSection,
    // FlagsEnableClientConnection,
    // FlagsEnablePublicSignupUserExistsError,
    // FlagsEnableAdfsWaadEmailVerification,
    FlagsRevokeRefreshTokenGrant,
    // Extensibility
    FlagsDisableClickjackProtectionHeaders,
    // Fixed length token
}

pub struct SettingsAdvanced {
    tenant_settings: TenantSettings,
    link: ComponentLink<Self>,
    loading_request_settings: bool,
    loading_update_login_logout: bool,
    loading_update_login_session: bool,
    loading_update_device_flow: bool,
    loading_update_settings: bool,
    loading_update_extensibility: bool,
    loading_delete: bool,
    error_request_settings: Option<String>,
    error_update_login_logout: Option<String>,
    error_update_login_session: Option<String>,
    error_update_device_flow: Option<String>,
    error_update_settings: Option<String>,
    error_update_extensibility: Option<String>,
    error_delete: Option<String>,
    fetch_task: Option<FetchTask>,
    route_service: RouteService,
}

pub enum Msg {
    RequestSettingsDetails,
    GetSettingsDetails(TenantSettings),
    SetDefaultState,
    InputString(String, Data),
    // UpdateLoginLogout,
    // UpdateLoginSession,
    // UpdateDeviceFlow,
    // // UpdateGlobalClientInfo,
    UpdateSettings,
    // UpdateExtensibility,
    // // UpdateMigrations,
    DeleteTenant,
    ResponseError(String, StateError),
    RedirectToGettingStarted,
}

impl Component for SettingsAdvanced {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SettingsAdvanced {
            tenant_settings: TenantSettings::new(),
            link,
            loading_request_settings: false,
            loading_update_login_logout: false,
            loading_update_login_session: false,
            loading_update_device_flow: false,
            loading_update_settings: false,
            loading_update_extensibility: false,
            loading_delete: false,
            error_request_settings: None,
            error_update_login_logout: None,
            error_update_login_session: None,
            error_update_device_flow: None,
            error_update_settings: None,
            error_update_extensibility: None,
            error_delete: None,
            fetch_task: None,
            route_service: RouteService::new(),
        }
    }
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestSettingsDetails);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestSettingsDetails => {
                let request = Request::get("http://127.0.0.1:8080/api/v1/1/tenants/settings")
                    .header("access_token", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImhleWthbGxAZ21haWwuY29tIiwiZXhwIjoxNjQzMDk0MTA0fQ.G_kEzjOwrzI_qD8Tco_4HTgXctsz4kUccl4e92WNZb8")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<TenantSettings, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", dataok));
                                Msg::GetSettingsDetails(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::GetSettings)
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_request_settings = None;
                self.loading_request_settings = true;
                true
            }
            Msg::GetSettingsDetails(data) => {
                self.tenant_settings = data;
                self.loading_request_settings = false;
                self.fetch_task = None;
                true
            }
            Msg::SetDefaultState => {
                self.loading_request_settings = false;
                self.loading_update_login_logout = false;
                self.loading_update_login_session = false;
                self.loading_update_device_flow = false;
                self.loading_update_settings = false;
                self.loading_update_extensibility = false;
                self.loading_delete = false;
                // self.error_request_settings = None;
                // self.error_update_login_logout = None;
                // self.error_update_login_session = None;
                // self.error_update_device_flow = None;
                // self.error_update_settings = None;
                // self.error_update_extensibility = None;
                // self.error_delete = None;
                true
            }
            Msg::InputString(value, data) => {
                match data {
                    // Data::AllowedLogoutUrls => {
                    //     self.tenant_settings.allowed_logout_urls = vec![value];
                    // }
                    // Data::DefaultRedirectionUri => {
                    //     self.tenant_settings.default_redirection_uri = value;
                    // }
                    // Data::SessionCookieMode => {
                    //     self.tenant_settings.session_cookie.mode = value;
                    // }
                    // Data::IdleSessionLifetime => {
                    //     if value.is_empty() {
                    //         self.tenant_settings.idle_session_lifetime = 0;
                    //     } else {
                    //         self.tenant_settings.idle_session_lifetime = value.parse::<u64>().unwrap();
                    //     }
                    // }
                    // Data::SessionLifetime => {
                    //     if value.is_empty() {
                    //         self.tenant_settings.session_lifetime = 0;
                    //     } else {
                    //         self.tenant_settings.session_lifetime = value.parse::<u64>().unwrap();
                    //     }
                    // }
                    // Data::DeviceFlowCharset => {
                    //     self.tenant_settings.device_flow.charset = value;
                    // }
                    // Data::DeviceFlowMask => {
                    //     self.tenant_settings.device_flow.mask = value;
                    // }
                    // Data::FlagsChangePwdFlowV1 => {
                    //     self.tenant_settings.flags.change_pwd_flow_v1 = !self.tenant_settings.flags.change_pwd_flow_v1;
                    //     self.link.send_message(Msg::UpdateSettings);
                    // }
                    // Data::FlagsEnableApisSection => {
                    //     self.tenant_settings.flags.enable_apis_section = !self.tenant_settings.flags.enable_apis_section;
                    //     self.link.send_message(Msg::UpdateSettings);
                    // }
                    // Data::FlagsEnableClientConnection => {
                    //     self.tenant_settings.flags.enable_client_connections = !self.tenant_settings.flags.enable_client_connections;
                    //     self.link.send_message(Msg::UpdateSettings);
                    // }
                    // Data::FlagsEnablePublicSignupUserExistsError => {
                    //     self.tenant_settings.flags.enable_public_signup_user_exists_error = !self.tenant_settings.flags.enable_public_signup_user_exists_error;
                    //     self.link.send_message(Msg::UpdateSettings);
                    // }
                    // Data::FlagsEnableAdfsWaadEmailVerification => {
                    //     self.tenant_settings.flags.enable_adfs_waad_email_verification = !self.tenant_settings.flags.enable_adfs_waad_email_verification;
                    //     self.link.send_message(Msg::UpdateSettings);
                    // }
                    Data::FlagsRevokeRefreshTokenGrant => {
                        self.tenant_settings.flags.revoke_refresh_token_grant = !self.tenant_settings.flags.revoke_refresh_token_grant;
                        self.link.send_message(Msg::UpdateSettings);
                    }
                    Data::FlagsDisableClickjackProtectionHeaders => {
                        self.tenant_settings.flags.disable_clickjack_protection_headers = !self.tenant_settings.flags.disable_clickjack_protection_headers;
                        self.link.send_message(Msg::UpdateSettings);
                    }
                }
                true
            }
            // Msg::UpdateLoginLogout => {
            //     ConsoleService::info(&format!("{:?}", self.tenant_settings));
            //     #[derive(Serialize, Debug, Clone)]
            //     pub struct DataLoginLogout {
            //         allowed_logout_urls: Vec<String>,
            //         default_redirection_uri: String,
            //     }
            //     let data_login_logout = DataLoginLogout {
            //         allowed_logout_urls: self.tenant_settings.allowed_logout_urls.clone(),
            //         default_redirection_uri: self.tenant_settings.default_redirection_uri.clone(),
            //     };
            //     ConsoleService::info(&format!("data settings = {:?}", data_settings));
            //     let request = Request::patch(format!("{}/tenant/v2/settings", API_URL))
            //         .header("Content-Type", "application/json")
            //         .header("access_token", "tokennotfromreducer")
            //         .body(Json(
            //             &data_login_logout
            //         ))
            //         .expect("Could not build request.");
            //     let callback = self.link.batch_callback(|response: Response<Json<Result<TenantSettings, anyhow::Error>>>| {
            //         let Json(data) = response.into_body();
            //         match data {
            //             Ok(dataok) => {
            //                 ConsoleService::info(&format!("{:?}", dataok));
            //                 vec![Msg::GetSettingsDetails(dataok), Msg::SetDefaultState]
            //             }
            //             Err(error) => {
            //                 ConsoleService::info(&error.to_string());
            //                 vec![Msg::ResponseError(error.to_string(), StateError::UpdateLoginLogout)]
            //             }
            //         }
            //     });
            //     let task = FetchService::fetch(request, callback).expect("failed to start request");
            //     self.loading_update_login_logout = true;
            //     self.fetch_task = Some(task);
            //     true
            // }
            // Msg::UpdateLoginSession => {
            //     ConsoleService::info(&format!("{:?}", self.tenant_settings));
            //     #[derive(Serialize, Debug, Clone)]
            //     pub struct SessionCookie {
            //         pub mode: String
            //     }
            //     #[derive(Serialize, Debug, Clone)]
            //     pub struct DataLoginSession {
            //         session_cookie: SessionCookie
            //     }
            //     let data_login_session = DataLoginSession {
            //         session_cookie: SessionCookie {
            //             mode: self.tenant_settings.session_cookie.mode.clone(),
            //         },
            //     };
            //     let data_login_logout = DataLoginLogout {
            //         allowed_logout_urls: self.tenant_settings.allowed_logout_urls.clone(),
            //         default_redirection_uri: self.tenant_settings.default_redirection_uri.clone(),
            //     };
            //     ConsoleService::info(&format!("data settings = {:?}", data_settings));
            //     let request = Request::patch(format!("{}/tenant/v2/settings", API_URL))
            //         .header("Content-Type", "application/json")
            //         .header("access_token", "tokennotfromreducer")
            //         .body(Json(
            //             &data_login_session
            //         ))
            //         .expect("Could not build request.");
            //     let callback = self.link.batch_callback(|response: Response<Json<Result<TenantSettings, anyhow::Error>>>| {
            //         let Json(data) = response.into_body();
            //         match data {
            //             Ok(dataok) => {
            //                 ConsoleService::info(&format!("{:?}", dataok));
            //                 vec![Msg::GetSettingsDetails(dataok), Msg::SetDefaultState]
            //             }
            //             Err(error) => {
            //                 ConsoleService::info(&error.to_string());
            //                 vec![Msg::ResponseError(error.to_string(), StateError::UpdateLoginSession)]
            //             }
            //         }
            //     });
            //     let task = FetchService::fetch(request, callback).expect("failed to start request");
            //     self.loading_update_login_session = true;
            //     self.fetch_task = Some(task);
            //     true
            // }
            // Msg::UpdateDeviceFlow => {
                // let data_device_flow = self.tenant_settings.device_flow.clone();
            //     let request = Request::patch(format!("{}/tenant/v2/settings", API_URL))
            //         .header("Content-Type", "application/json")
            //         .header("access_token", "tokennotfromreducer")
            //         .body(
            //             Json(
            //                 &data_device_flow
            //         ))
            //         .expect("Could not build request.");
            //     let callback = self.link.batch_callback(|response: Response<Json<Result<TenantSettings, anyhow::Error>>>| {
            //         let Json(data) = response.into_body();
            //         match data {
            //             Ok(dataok) => {
            //                 ConsoleService::info(&format!("{:?}", dataok));
            //                 vec![Msg::GetSettingsDetails(dataok), Msg::SetDefaultState]
            //             }
            //             Err(error) => {
            //                 ConsoleService::info(&error.to_string());
            //                 vec![Msg::ResponseError(error.to_string(), StateError::UpdateDeviceFlow)]
            //             }
            //         }
            //     });
            //     let task = FetchService::fetch(request, callback).expect("failed to start request");
            //     self.loading_update_device_flow = true;
            //     self.fetch_task = Some(task);
            //     true
            // }
            Msg::UpdateSettings => {
                
                let data_settings = self.tenant_settings.flags.clone();
                ConsoleService::info(&format!("{:?}", data_settings.clone()));
                let request = Request::patch("http://127.0.0.1:8080/api/v1/1/tenants/settings")
                    .header("Content-Type", "application/json")
                    .header("access_token", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImhleWthbGxAZ21haWwuY29tIiwiZXhwIjoxNjQzMDk0MTA0fQ.G_kEzjOwrzI_qD8Tco_4HTgXctsz4kUccl4e92WNZb8")
                    .body(Json(&data_settings))
                    .expect("Could not build request.");
                let callback = self.link.batch_callback(|response: Response<Json<Result<TenantSettings, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    match data {
                        Ok(dataok) => {
                            // ConsoleService::info(&format!("{:?}", dataok));
                            vec![Msg::GetSettingsDetails(dataok), Msg::SetDefaultState]
                        }
                        Err(error) => {
                            // ConsoleService::info(&error.to_string());
                            vec![Msg::ResponseError(error.to_string(), StateError::UpdateSettings)]
                        }
                    }
                });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_update_settings = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::DeleteTenant => {
                let request = Request::delete(format!("{}/tenant/v2/settings", API_URL))
                    .header("access_token", "tokennotfromreducer")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<ResponseMessage, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", dataok));
                                Msg::RedirectToGettingStarted
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::DeleteTenant)
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_delete = None;
                self.loading_delete = true;
                true
            }
            Msg::ResponseError(message, state) => {
                self.fetch_task = None;
                match state {
                    StateError::GetSettings => {
                        self.loading_request_settings = false;
                        self.error_request_settings = Some(message);
                    }
                    StateError::UpdateLoginLogout => {
                        self.loading_update_login_logout = false;
                        self.error_update_login_logout = Some(message);
                    }
                    StateError::UpdateLoginSession => {
                        self.loading_update_login_session = false;
                        self.error_update_login_session = Some(message);
                    }
                    StateError::UpdateDeviceFlow => {
                        self.loading_update_device_flow = false;
                        self.error_update_device_flow = Some(message);
                    }
                    StateError::UpdateSettings => {
                        self.loading_update_settings = false;
                        self.error_update_settings = Some(message);
                    }
                    StateError::DeleteTenant => {
                        self.loading_delete = false;
                        self.error_delete = Some(message);
                    }
                }
                true
            }
            Msg::RedirectToGettingStarted => {
                self.loading_delete = false;
                self.fetch_task = None;
                self.route_service.set_route("/getting-started", ());
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.loading_request_settings {
            html! {
                <div
                    style="
                        position: relative;
                        margin-top: 8rem;
                    "
                >
                    <Loading width=45 />
                </div>
            }
        } else if self.error_request_settings.is_some() {
            html! {
                <div class="alert alert-warning mb-5" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    { self.error_request_settings.clone().unwrap() }
                </div>
            }
        } else {
            html! {
                { self.view_content() }
            }
        }
    }
}


impl SettingsAdvanced {
    fn view_content (&self) -> Html {
        let TenantSettings {
            // change_password: _,
            // guardian_mfa_page: _,
            // default_audience: _,
            // default_directory: _,
            // error_page: _,
            // device_flow,
            flags,
            // friendly_name: _,
            // picture_url: _,
            // support_email: _,
            // support_url: _,
            // allowed_logout_urls,
            // session_lifetime,
            // idle_session_lifetime,
            sandbox_version: _,
            sandbox_versions_available: _,
            // default_redirection_uri,
            enabled_locales: _,
            // session_cookie,
        } = self.tenant_settings.clone();
        html! {
            <div>
                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >

                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                            style="font-size: 16px;"
                        >
                            {"Login and Logout"}
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Allowed Logout URLs"}
                                </p>
                                <div class="input-group mb-2">
                                    <textarea
                                        class="form-control"
                                        rows="4"
                                        placeholder="https://mycompany.org/logoutCallback"
                                        // value={allowed_logout_urls[0].clone()}
                                        // oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::AllowedLogoutUrls))
                                        disabled={ self.loading_update_login_logout }
                                    ></textarea>
                                </div>
                                <p
                                    class="mb-0"
                                >
                                    {"A set of URLs that are valid to redirect to after logout from Auth0 when no client_id is specified on the logout endpoint invocation. It's useful as a global list when SSO is enabled. Read more about this at"}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Tenant Login URI"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="https://mycompany.org/login"
                                        // value={ default_redirection_uri.clone() }
                                        // oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::DefaultRedirectionUri))
                                        disabled={ self.loading_update_login_logout }    
                                    />
                                </div>
                                <p
                                    class="mb-0"
                                >
                                    {"In some scenarios, Auth0 will need to redirect to your tenant’s login page. This URI needs to point to a route in your application that should redirect to your tenant’s /authorize endpoint. Learn more."}
                                </p>
                            </div>

                            <div
                                class="mt-3 mb-4"
                            >
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_update_login_logout {"loading"} else {""} )
                                    // onclick=self.link.callback(|_| Msg::UpdateLoginLogout)
                                    disabled={ self.loading_update_login_logout }
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                {
                                    if self.error_update_login_logout.is_some() {
                                    html! {
                                        <div class="alert alert-warning mt-3" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_update_login_logout.clone().unwrap() }
                                        </div>
                                    }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>

                        </div>
                    </div>
            
            
                </div>



                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                            style="font-size: 16px;"
                        >
                            {"Login Session Management"}
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Session Cookie Mode *"}
                                </p>
                                <div
                                    class="row"
                                >
                                    <div
                                        class="col-md-6 col-sm-12 mb-2"
                                    >
                                        <div
                                            // class=format!("card {}", if session_cookie.mode == String::from("persistent") {"border border-primary border-2"} else {""} )
                                            // onclick=self.link.callback(|_| Msg::InputString(String::from("persistent"), Data::SessionCookieMode))
                                            style=format!("cursor: pointer; {}", if self.loading_update_login_session {"pointer-events: none;"} else {""} )
                                        >
                                            <div class="card-body">
                                            <p class="card-title mb-2 fw-bold">{"Persistent Session"}</p>
                                            <p class="card-text text-color-disabled">{"Allows the user to retain their session cookie when re-opening the browser on the same device."}</p>
                                            </div>
                                        </div>
                                    </div>
                                    <div
                                        class="col-md-6 col-sm-12 mb-2"
                                    >
                                    <div
                                        // class=format!("card {}", if session_cookie.mode == String::from("non-persistent") {"border border-primary border-2"} else {""} )
                                        // onclick=self.link.callback(|_| Msg::InputString(String::from("non-persistent"), Data::SessionCookieMode))
                                        style=format!("cursor: pointer; {}", if self.loading_update_login_session {"pointer-events: none;"} else {""} )
                                    >
                                    <div class="card-body">
                                            <p class="card-title mb-2 fw-bold">{"Non-Persistent Session"}</p>
                                            <p class="card-text text-color-disabled">{"Invalidates the session cookie when the browser is closed."}</p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <p
                                    class="mb-0"
                                >
                                    {"By default, users will not be asked to log in again and will have a persistent cookie stored locally. This will affect all sessions managed by Auth0."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Inactivity Timeout *"}
                                </p>
                                <div class="input-group mb-3">
                                    <input
                                        type="number"
                                        class="form-control"
                                        aria-label="Timeout"
                                        // value={idle_session_lifetime.to_string().clone()}
                                        // oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::IdleSessionLifetime))
                                        disabled={ self.loading_update_login_session }
                                    />
                                    <span
                                        class="input-group-text"
                                    >
                                        {"minutes"}
                                    </span>
                                    </div>
                                <p
                                    class="mb-0"
                                >
                                    {"Users will be asked to log in again unless they are active within this period (max."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Require login after *"}
                                </p>
                                <div class="input-group mb-3">
                                    <input
                                        type="number"
                                        class="form-control"
                                        // value={session_lifetime.to_string().clone()}
                                        // oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::SessionLifetime))
                                        disabled={ self.loading_update_login_session }
                                    />
                                    <span class="input-group-text" id="basic-addon2">{"minutes"}</span>
                                    </div>
                                <p
                                    class="mb-0"
                                >
                                    {"Regardless of activity, users will be forced to log in after the period (max."}
                                </p>
                            </div>

                            <div
                                class="mt-3 mb-4"
                            >
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_update_login_session {"loading"} else {""} )
                                    // onclick=self.link.callback(|_| Msg::UpdateLoginSession)
                                    disabled={ if self.loading_update_login_session {true} else {false} }
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                {
                                    if self.error_update_login_session.is_some() {
                                    html! {
                                        <div class="alert alert-warning mt-3" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_update_login_session.clone().unwrap() }
                                        </div>
                                    }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>

                        </div>
                    </div>
            
            
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                            style="font-size: 16px;"
                        >
                            {"Device Flow User Code Format"}
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"User Code Character Set *"}
                                </p>
                                <select
                                    class="form-select mb-2"
                                    aria-label="Default select example"
                                    // onchange=self.link.callback(|e| {
                                    //     if let ChangeData::Select(select) = e {
                                    //         let value = select.value();
                                    //         Msg::InputString(value, Data::DeviceFlowCharset)
                                    //     } else {
                                    //         Msg::InputString(String::from("no value"), Data::DeviceFlowCharset)
                                    //     }
                                    // })
                                    disabled={ self.loading_update_device_flow }
                                >
                                    <option
                                        value="base20"
                                        // selected={ if device_flow.charset == String::from("base20") {true} else {false} }
                                    >
                                        {"Base-20 (BCDFGHJKLMNPQRSTVWXZ)"}
                                    </option>
                                    <option
                                        value="digits"
                                        // selected={ if device_flow.charset == String::from("digits") {true} else {false} }
                                    >
                                        {"Digits (0123456789)"}
                                    </option>
                                </select>
                                <p
                                    class="mb-0 text-color-disabled"
                                >
                                    {"The character set for generating a User Code."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"User Code Mask *"}
                                </p>
                                <div class="input-group mb-3">
                                    <input
                                        type="text"
                                        class="form-control"
                                        aria-label="Recipient's username"
                                        aria-describedby="basic-addon2"
                                        // value={device_flow.mask.clone()}
                                        // oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::DeviceFlowMask))
                                        disabled={ self.loading_update_device_flow }
                                    />
                                    <span class="input-group-text" id="basic-addon2">{"e.g BCDF-GHJK"}</span>
                                    </div>
                                <p
                                    class="mb-0 text-color-disabled"
                                >
                                    {"The mask is used to define the length of the User Code and to format the randomly generated User Code to a friendly, readable value with possible spaces or hyphens for readability."}
                                </p>
                            </div>

                            <div
                                class="mt-3 mb-4"
                            >
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_update_device_flow {"loading"} else {""} )
                                    // onclick=self.link.callback(|_| Msg::UpdateDeviceFlow)
                                    disabled={ self.loading_update_device_flow }
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                {
                                    if self.error_update_device_flow.is_some() {
                                        html! {
                                            <div class="alert alert-warning mt-3" role="alert">
                                                <i class="bi bi-exclamation-triangle me-2"></i>
                                                { self.error_update_device_flow.clone().unwrap() }
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>

                        </div>
                    </div>
            
            
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 mb-4"
                        >
                            <div
                                class="text-color-primary fw-bold mb-1"
                                style="font-size: 16px;"
                            >
                                {"Global Client Information"}
                            </div>
                            <p
                                class="text-color-disabled"
                            >
                                {"The global client ID and secret are used to generate tokens for legacy Auth0 APIs. Typically, you will not need these values. If you need to have the global client secret changed, please contact support."}
                            </p>
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Global Client ID"}
                                </p>
                                <div class="input-group mb-3">
                                    <input type="text" class="form-control" placeholder="Recipient's username" aria-label="Recipient's username" aria-describedby="button-addon2"/>
                                    <button class="btn btn-outline-secondary" type="button" id="button-addon2">
                                        <i class="bi bi-files ms-1"></i>
                                    </button>
                                </div>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Global Client Secret"}
                                </p>
                                <div class="input-group mb-3">
                                    <input type="text" class="form-control" placeholder="Recipient's username" aria-label="Recipient's username" aria-describedby="button-addon2"/>
                                    <button class="btn btn-outline-secondary" type="button" id="button-addon2">
                                        <i class="bi bi-eye-slash"></i>
                                        <i class="bi bi-eye"></i>
                                    </button>
                                    <button class="btn btn-outline-secondary" type="button" id="button-addon2">
                                        <i class="bi bi-files ms-1"></i>
                                    </button>
                                </div>
                            </div>

                        </div>
                    </div>
            
            
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 mb-4"
                        >
                            <div
                                class="text-color-primary fw-bold mb-1"
                                style="font-size: 16px;"
                            >
                                {"Settings"}
                            </div>
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Change Password flow v2"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input
                                        class="form-check-input"
                                        type="checkbox"
                                        id="flexSwitchCheckChecked"
                                        // checked={flags.change_pwd_flow_v1.clone()}
                                        // onclick=self.link.callback(|_| Msg::InputString(String::from(""), Data::FlagsChangePwdFlowV1))
                                    />
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"Enables a new version of the Change Password flow. We've deprecated the previous alternative and we strongly recommend enabling this option. This flag is presented only for backwards compatibility and once enabled you won't be able to move it back. You can configure how the Change Password widget will look like at the Password Reset tab inside the Universal Login section."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"OIDC Dynamic Application Registration"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input
                                        class="form-check-input"
                                        type="checkbox"
                                        id="flexSwitchCheckChecked"
                                        // checked={ flags.enable_apis_section.clone() }
                                        // onclick=self.link.callback(|_| Msg::InputString(String::from(""), Data::FlagsEnableApisSection))
                                        disabled={ self.loading_update_settings }
                                    />
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"Enables third-party developers to dynamically register applications for your APIs. Learn more"}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Enable Application Connections"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input
                                        class="form-check-input"
                                        type="checkbox"
                                        id="flexSwitchCheckChecked"
                                        // checked={ flags.enable_client_connections.clone() }
                                        // onclick=self.link.callback(|_| Msg::InputString(String::from(""), Data::FlagsEnableClientConnection))
                                        disabled={ self.loading_update_settings }
                                    />
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"This flag determines whether all current connections shall be enabled when a new Application is created."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Use a generic response in public signup API error message"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input
                                        class="form-check-input"
                                        type="checkbox"
                                        id="flexSwitchCheckChecked"
                                        // checked={ flags.enable_public_signup_user_exists_error.clone() }
                                        // onclick=self.link.callback(|_| Msg::InputString(String::from(""), Data::FlagsEnablePublicSignupUserExistsError))
                                        disabled={ self.loading_update_settings }
                                    />
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"If enabled, this will use a generic response in the public signup API which will prevent users from being able to find out if an e-mail address or username has previously registered"}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Enable email verification flow during login for Azure AD and ADFS connections"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input
                                        class="form-check-input"
                                        type="checkbox"
                                        id="flexSwitchCheckChecked"
                                        // checked={ flags.enable_adfs_waad_email_verification.clone() }
                                        // onclick=self.link.callback(|_| Msg::InputString(String::from(""), Data::FlagsEnableAdfsWaadEmailVerification))
                                        disabled={ self.loading_update_settings }
                                    />
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"If enabled, users will be presented with an email verification prompt during their first login when using Azure AD or ADFS connections"}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Refresh Token Revocation Deletes Grant"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input
                                        class="form-check-input"
                                        type="checkbox"
                                        id="flexSwitchCheckChecked"
                                        checked={ flags.revoke_refresh_token_grant.clone() }
                                        onclick=self.link.callback(|_| Msg::InputString(String::from(""), Data::FlagsRevokeRefreshTokenGrant))
                                        disabled={ self.loading_update_settings }
                                    />
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"Delete underlying grant when a Refresh Token is revoked via the Authentication API."}
                                </p>
                            </div>

                        </div>
                    </div>
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 mb-4"
                            style="font-size: 16px;"
                        >
                            <div
                                class="text-color-primary fw-bold"
                            >
                                {"Extensibility"}
                            </div>
                            <p
                                class="mb-0 text-color-disabled"
                            >
                                {"Use custom scripts to extend parts of Auth0's functionality, such as Rules, Hooks and Database Connections."}
                            </p>
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Runtime *"}
                                </p>
                                <select
                                    class="form-select mb-2"
                                    aria-label="Default select example"
                                    // onchange=self.link.callback(|e| {
                                    //     if let ChangeData::Select(select) = e {
                                    //         let value = select.value();
                                    //         Msg::InputString(value, Data::De)
                                    //     } else {
                                    //         Msg::InputString(String::from("no value"), Data::De)
                                    //     }
                                    // })
                                    disabled={ self.loading_update_device_flow }
                                >
                                    <option
                                        value="Node"
                                        // selected={ if }
                                    >{"Node 12"}</option>
                                </select>
                                <p
                                    class="mb-0 text-color-disabled"
                                >
                                    {"The NodeJS version environment used to execute your custom scripts."}
                                </p>
                            </div>
                            
                            <div
                                class="mt-3 mb-4"
                            >
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_update_device_flow {"loading"} else {""} )
                                    // onclick=self.link.callback(|_| Msg::UpdateDeviceFlow)
                                    disabled={ self.loading_update_device_flow }
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                {
                                    if self.error_update_device_flow.is_some() {
                                    html! {
                                        <div class="alert alert-warning mt-3" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_update_device_flow.clone().unwrap() }
                                        </div>
                                    }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>

                        </div>
                    </div>
            
            
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 mb-4"
                        >
                            <div
                                class="text-color-primary fw-bold mb-1"
                                style="font-size: 16px;"
                            >
                                {"Migrations"}
                            </div>
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Disable clickjacking protection for Classic Universal Login"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input
                                        class="form-check-input"
                                        type="checkbox"
                                        id="flexSwitchCheckChecked"
                                        checked={ flags.disable_clickjack_protection_headers.clone() }
                                        onclick=self.link.callback(|_| Msg::InputString(String::from(""), Data::FlagsDisableClickjackProtectionHeaders))
                                        disabled={ self.loading_update_settings }
                                    />
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"If enabled, additional HTTP security headers will not be included in the response to prevent embedding of the Universal Login prompts in an IFRAME."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Fixed Length of Access Token & Authorization Code"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckChecked" checked={true}/>
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"If enabled, the Auth0 platform issues Access Tokens and Authorization Codes of fixed length. By default, these are of variable length.Learn More"}
                                </p>
                            </div>

                        </div>
                    </div>
                </div>

                <div
                    style="font-size: 14px;"
                >
                    <p
                        class="fw-bold"
                    >
                        {"Danger Zone"}
                    </p>

                    <div class="alert alert-danger d-flex flex-row justify-content-between" role="alert">
                        <div>
                            <p
                                class="fw-bold"
                            >
                                {"Delete Connection"}
                            </p>
                            {"Once confirmed, this operation can't be undone!"}
                        </div>
                        <div
                            class="mb-5 mt-3"
                        >
                            <button
                                type="button"
                                class=format!("btn {} btn-danger position-relative", if self.loading_delete {"loading"} else {""} )
                                onclick=self.link.callback(|_| Msg::DeleteTenant)
                                disabled={ if self.loading_delete {true} else {false} }
                            >
                                <div class="telkom-label">
                                    {"Delete"}
                                </div>
                                <div class="telkom-spinner telkom-center">
                                    <div class="spinner-border spinner-border-sm" role="status"/>
                                </div>
                            </button>

                            {
                                if self.error_delete.is_some() {
                                html! {
                                    <div class="alert alert-warning mt-3" role="alert">
                                        <i class="bi bi-exclamation-triangle me-2"></i>
                                        { self.error_delete.clone().unwrap() }
                                    </div>
                                }
                                } else {
                                    html! {}
                                }
                            }

                        </div>
                    </div>
                </div>

            </div>
        }
    }
}