use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
    }
};
use serde::{
    // Deserialize,
    Serialize,
};
use types::settings::{
    TenantSettings,
    ErrorPage,
};
use crate::configs::server::API_URL;
use loading::Loading;


// #[derive(Clone, Debug, Eq, PartialEq, Properties)]
// pub struct SettingsTabGeneralProps {
//     pub tenant_settings: TenantSettings,
// }

pub enum StateError {
    GetSettings,
    UpdateSettings,
    UpdateEnvironmentTag,
    UpdateAuthorization,
    UpdateErrorPage,
    UpdateLanguage,
}

pub enum Data {
    FriendlyName,
    PictureUrl,
    SupportEmail,
    SupportUrl,
    // environment tag
    DefaultAudience,
    DefaultDirectory,
    ErrorPage,
    // Language

}

#[derive(Serialize, Debug, Clone)]
struct DataSettings {
    friendly_name: String,
    picture_url: String,
    support_email: String,
    support_url: String,
}
// Data environment
#[derive(Serialize, Debug, Clone)]
struct DataAuthorization {
    default_audience: String,
    default_directory: String,
}
#[derive(Serialize, Debug, Clone)]
struct DataErrorPage {
    error_page: ErrorPage
}
// #[derive(Serialize, Debug, Clone)]
// struct DataLanguage {
//     default_language: String,
//     supported_language: String,
// }

pub struct SettingsGeneral {
    tenant_settings: TenantSettings,
    link: ComponentLink<Self>,
    loading_request_settings: bool,
    error_request_settings: Option<String>,
    fetch_task: Option<FetchTask>,
    loading_update_settings: bool,
    loading_update_environment_tag: bool,
    loading_update_authorization: bool,
    loading_update_error_page: bool,
    loading_update_language: bool,
    error_update_settings: Option<String>,
    error_update_environment_tag: Option<String>,
    error_update_authorization: Option<String>,
    error_update_error_page: Option<String>,
    error_update_language: Option<String>,
}

pub enum Msg {
    RequestSettingsDetails,
    GetSettingsDetails(TenantSettings),

    InputString(String, Data),
    InputBool(bool, Data),
    UpdateSettings,
    UpdateEnvironmentTag,
    UpdateAuthorization,
    UpdateErrorPage,
    UpdateLanguage,
    GetTenantSettings(TenantSettings),
    ResponseError(String, StateError),
}

impl Component for SettingsGeneral {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SettingsGeneral {
            tenant_settings: TenantSettings::new(),
            loading_request_settings: false,
            error_request_settings: None,
            link,
            fetch_task: None,
            loading_update_settings: false,
            loading_update_environment_tag: false,
            loading_update_authorization: false,
            loading_update_error_page: false,
            loading_update_language: false,
            error_update_settings: None,
            error_update_environment_tag: None,
            error_update_authorization: None,
            error_update_error_page: None,
            error_update_language: None, 
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
                    // .header("Content-Type", "application/json")
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
            // Msg::InputString(value, data) => {
            //     match data {
            //         Data::FriendlyName => {
            //             self.tenant_settings.friendly_name = value;
            //             true
            //         }
            //         Data::PictureUrl => {
            //             self.tenant_settings.picture_url = value;
            //             true
            //         }
            //         Data::SupportEmail => {
            //             self.tenant_settings.support_email = value;
            //             true
            //         }
            //         Data::SupportUrl => {
            //             self.tenant_settings.support_url = value;
            //             true
            //         }
            //         Data::DefaultAudience => {
            //             self.tenant_settings.default_audience = value;
            //             true
            //         }
            //         Data::DefaultDirectory => {
            //             self.tenant_settings.default_directory = value;
            //             true
            //         }
            //         Data::ErrorPage => {
            //             self.tenant_settings.error_page.url = value;
            //             true
            //         }
            //         // _ => {
            //         //     false
            //         // }
            //     }
            // }
            // Msg::InputBool(value, data) => {
            //     match data {
            //         Data::ErrorPage => {
            //             self.tenant_settings.error_page.show_log_link = value;
            //             true
            //         }
            //         _ => {
            //             false
            //         }
            //     }
            // }
            // Msg::UpdateSettings => {
            //     let data_settings = DataSettings {
            //         friendly_name: self.tenant_settings.friendly_name.clone(),
            //         picture_url: self.tenant_settings.picture_url.clone(),
            //         support_email: self.tenant_settings.support_email.clone(),
            //         support_url: self.tenant_settings.support_url.clone(),
            //     };
            //     ConsoleService::info(&format!("data settings = {:?}", data_settings));
            //     let request = Request::patch(format!("{}/tenant/v2/settings", API_URL))
            //         .header("Content-Type", "application/json")
            //         .header("access_token", "tokennotfromreducer")
            //         .body(Json(
            //             &data_settings
            //         ))
            //         .expect("Could not build request.");
            //     let callback = self.link.callback(|response: Response<Json<Result<TenantSettings, anyhow::Error>>>| {
            //         let Json(data) = response.into_body();
            //         match data {
            //             Ok(dataok) => {
            //                 ConsoleService::info(&format!("{:?}", dataok));
            //                 Msg::GetTenantSettings(dataok)
            //             }
            //             Err(error) => {
            //                 ConsoleService::info(&error.to_string());
            //                 Msg::ResponseError(error.to_string(), StateError::UpdateSettings)
            //             }
            //         }
            //     });
            //     let task = FetchService::fetch(request, callback).expect("failed to start request");
            //     self.loading_update_settings = true;
            //     self.fetch_task = Some(task);
            //     true
            // }
            // Msg::UpdateAuthorization => {
            //     let data_authorization = DataAuthorization {
            //         default_audience: self.tenant_settings.default_audience.clone(),
            //         default_directory: self.tenant_settings.default_directory.clone(),
            //     };
            //     ConsoleService::info(&format!("data authorization = {:?}", data_authorization));
            //     let request = Request::patch(format!("{}/tenant/v2/settings", API_URL))
            //         .header("Content-Type", "application/json")
            //         .header("access_token", "tokennotfromreducer")
            //         .body(Json(&data_authorization))
            //         .expect("Could not build request.");
            //     let callback = self.link.callback(|response: Response<Json<Result<TenantSettings, anyhow::Error>>>| {
            //         let Json(data) = response.into_body();
            //         match data {
            //             Ok(dataok) => {
            //                 ConsoleService::info(&format!("{:?}", dataok));
            //                 Msg::GetTenantSettings(dataok)
            //             }
            //             Err(error) => {
            //                 ConsoleService::info(&error.to_string());
            //                 Msg::ResponseError(error.to_string(), StateError::UpdateAuthorization)
            //             }
            //         }
            //     });
            //     let task = FetchService::fetch(request, callback).expect("failed to start request");
            //     self.loading_update_authorization = true;
            //     self.fetch_task = Some(task);
            //     true
            // }
            // Msg::UpdateErrorPage => {
            //     let data_error_page = DataErrorPage {
            //         error_page: self.tenant_settings.error_page.clone()
            //     };
            //     ConsoleService::info(&format!("data error page = {:?}", data_error_page));
            //     let request = Request::patch(format!("{}/tenant/v2/settings", API_URL))
            //         .header("Content-Type", "application/json")
            //         .header("access_token", "tokennotfromreducer")
            //         .body(Json(
            //             &data_error_page
            //         ))
            //         .expect("Could not build request.");
            //     let callback = self.link.callback(|response: Response<Json<Result<TenantSettings, anyhow::Error>>>| {
            //         let Json(data) = response.into_body();
            //         match data {
            //             Ok(dataok) => {
            //                 ConsoleService::info(&format!("{:?}", dataok));
            //                 Msg::GetTenantSettings(dataok)
            //             }
            //             Err(error) => {
            //                 ConsoleService::info(&error.to_string());
            //                 Msg::ResponseError(error.to_string(), StateError::UpdateErrorPage)
            //             }
            //         }
            //     });
            //     let task = FetchService::fetch(request, callback).expect("failed to start request");
            //     self.loading_update_error_page = true;
            //     self.fetch_task = Some(task);
            //     true
            // }
            Msg::GetTenantSettings(data) => {
                self.fetch_task = None;
                self.loading_update_settings = false;
                self.loading_update_authorization = false;
                self.loading_update_error_page = false;
                self.error_update_settings = None;
                self.tenant_settings = data;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::GetSettings => {
                        self.fetch_task = None;
                        self.loading_request_settings = false;
                        self.error_request_settings = Some(message);
                    }
                    StateError::UpdateSettings => {
                        self.fetch_task = None;
                        self.loading_update_settings = false;
                        self.error_update_settings = Some(message);
                    }
                    StateError::UpdateEnvironmentTag => {
                        self.fetch_task = None;
                        self.loading_update_environment_tag = false;
                        self.error_update_environment_tag = Some(message);
                    }
                    StateError::UpdateAuthorization => {
                        self.fetch_task = None;
                        self.loading_update_authorization = false;
                        self.error_update_authorization = Some(message);
                    }
                    StateError::UpdateErrorPage => {
                        self.fetch_task = None;
                        self.loading_update_error_page = false;
                        self.error_update_error_page = Some(message);
                    }
                    StateError::UpdateLanguage => {
                        self.fetch_task = None;
                        self.loading_update_language = false;
                        self.error_update_language = Some(message);
                    }
                }
                true
            }
            _ => {
                false
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


impl SettingsGeneral {
    fn view_content (&self) -> Html {
        let TenantSettings {
            // change_password: _,
            // guardian_mfa_page: _,
            // default_audience,
            // default_directory,
            // error_page,
            // device_flow: _,
            flags: _,
            // friendly_name,
            // picture_url,
            // support_email,
            // support_url,
            // allowed_logout_urls: _,
            // session_lifetime: _,
            // idle_session_lifetime: _,
            sandbox_version: _,
            sandbox_versions_available: _,
            // default_redirection_uri: _,
            enabled_locales: _,
            // session_cookie: _,
        } = self.tenant_settings.clone();
        html! {
            <div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-5"
                    style="font-size: 14px;"
                >

                    <div
                        class="text-color-primary fw-bold mb-4"
                        style="font-size: 16px;"
                    >
                        {"Tenant Information"}
                    </div>
                    <div
                        class="row border-bottom mb-3"
                    >
                        <div
                            class="col-lg-6 text-muted mb-4"
                            style="font-size: 14px;"
                        >
                            {"Tenant Name"}
                        </div>
                        <div
                            class="col-lg-6 mb-3"
                        >
                            <span
                                class="text-color-primary"
                            >
                                {"dev-ofzd5p1b"}
                            </span>
                        </div>
                    </div>

                    <div
                        class="row border-bottom mb-3"
                    >
                        <div
                            class="col-lg-6 text-muted mb-4"
                            style="font-size: 14px;"
                        >
                            {"Region"}
                        </div>
                        <div
                            class="col-lg-6 mb-3"
                        >
                            <span
                                class="text-color-primary"
                            >
                                {"AU"}
                            </span>
                        </div>
                    </div>

                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 text-muted mb-4"
                            style="font-size: 14px;"
                        >
                            {"Environment"}
                        </div>
                        <div
                            class="col-lg-6 mb-3"
                        >
                            <span
                                class="text-color-primary"
                            >
                                {"Development"}
                            </span>
                        </div>
                    </div>
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-5"
                    style="font-size: 14px;"
                >

                    <div
                        class="row border-bottom"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                            style="font-size: 16px;"
                        >
                            {"Settings"}
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Friendly Name"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="My Company Inc."
                                        // value={friendly_name.clone()}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::FriendlyName))
                                        disabled={ if self.loading_update_settings {true} else {false} }
                                    />   
                                </div>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Logo URL"}
                                </p>
                                <div
                                    class="d-flex justify-content-center align-items-center border rounded-top"
                                    style="height: 120px;"
                                >
                                    <img
                                        src="https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/badge.png"
                                        style="height: 60px;"
                                    />
                                </div>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="Your logo URL"
                                        // value={picture_url}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::PictureUrl))
                                        disabled={ if self.loading_update_settings {true} else {false} }
                                    />   
                                </div>
                                <p
                                    class="mb-0"
                                >
                                    {"If a URL is not provided, the Auth0 logo will be used."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Support Email"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="support@my_company.com"
                                        // value={support_email}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::SupportEmail))
                                        disabled={ if self.loading_update_settings {true} else {false} }
                                    />
                                </div>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Support URL"}
                                </p>
                                <div class="input-group mb-4">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="https://my-company.org/support"
                                        // value={support_url}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::SupportUrl))
                                        disabled={ if self.loading_update_settings {true} else {false} }
                                    />
                                </div>
                            </div>

                            <div
                                class="mt-3 mb-5"
                            >
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_update_settings {"loading"} else {""} )
                                    onclick=self.link.callback(|_| Msg::UpdateSettings)
                                    disabled={ if self.loading_update_settings {true} else {false} }
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                {
                                    if self.error_update_settings.is_some() {
                                    html! {
                                        <div class="alert alert-warning mt-3" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_update_settings.clone().unwrap() }
                                        </div>
                                    }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>

                        </div>
                    </div>
            
            
                    
                    <div
                        class="row border-bottom mt-5"
                    >
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="text-color-primary fw-bold mb-4"
                                style="font-size: 16px;"
                            >
                                {"Environment Tag"}
                            </div>

                            <p
                                class="mt-2 text-color-disabled"
                            >
                                {"Assign an environment tag to your tenant to differentiate between development, staging and production environments."}
                            </p>
                            <p
                                class="text-color-disabled"
                            >
                                {"Higher rate limits are applied to tenants tagged as Production with a paid subscription."}
                            </p>
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Assign Environment Tag"}
                                </p>
                                <div class="card card-hover mb-2" style="cursor: pointer;">
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <div
                                                style="flex: 0 0 auto; width: 40px; height: 40px;"
                                                class="d-flex justify-content-center align-items-center rounded me-3 border bg-domain-secondary"
                                            >
                                                <i class="bi bi-code-slash fs-5 text-color-secondary"></i>
                                            </div>

                                            <div
                                                class="d-grid"
                                                style="min-width: 40px;"
                                            >
                                                <div
                                                    class="text-decoration-none fw-bold mb-0"
                                                >
                                                    <span
                                                        class="fw-bold"
                                                        style="
                                                            white-space: nowrap;
                                                            text-overflow: ellipsis;
                                                            overflow: hidden;
                                                            text-decoration: none;
                                                        "
                                                    >
                                                        {"Development"}
                                                    </span>
                                                </div>
                                                <p
                                                    class="mb-0 text-muted"
                                                >
                                                    {"The tenant is mainly used by engineers as a working environment to make configuration changes."}
                                                </p>
                                            </div>
                            
                                        </div>
                                    </div>
                                </div>
                                <div class="card card-hover mb-2" style="cursor: pointer;">
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <div
                                                style="flex: 0 0 auto; width: 40px; height: 40px;"
                                                class="d-flex justify-content-center align-items-center rounded me-3 border bg-domain-secondary"
                                            >
                                                <i class="bi bi-search fs-5 text-color-secondary"></i>
                                            </div>

                                            <div
                                                class="d-grid"
                                                style="min-width: 40px;"
                                            >
                                                <div
                                                    class="text-decoration-none fw-bold mb-0"
                                                >
                                                    <span
                                                        class="fw-bold"
                                                        style="
                                                            white-space: nowrap;
                                                            text-overflow: ellipsis;
                                                            overflow: hidden;
                                                            text-decoration: none;
                                                        "
                                                    >
                                                        {"Staging"}
                                                    </span>
                                                </div>
                                                <p
                                                    class="mb-0 text-muted"
                                                >
                                                    {"The tenant is mainly used by your testing team and is used to test changes before releasing them to Production."}
                                                </p>
                                            </div>
                            
                                        </div>
                                    </div>
                                </div>
                                <div class="card card-hover mb-2" style="cursor: pointer;">
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <div
                                                style="flex: 0 0 auto; width: 40px; height: 40px;"
                                                class="d-flex justify-content-center align-items-center rounded me-3 border bg-domain-secondary"
                                            >
                                                <i class="bi bi-check2 fs-4 text-color-secondary"></i>
                                            </div>

                                            <div
                                                class="d-grid"
                                                style="min-width: 40px;"
                                            >
                                                <div
                                                    class="text-decoration-none fw-bold mb-0"
                                                >
                                                    <span
                                                        class="fw-bold"
                                                        style="
                                                            white-space: nowrap;
                                                            text-overflow: ellipsis;
                                                            overflow: hidden;
                                                            text-decoration: none;
                                                        "
                                                    >
                                                        {"Production"}
                                                    </span>
                                                </div>
                                                <p
                                                    class="mb-0 text-muted"
                                                >
                                                    {"The tenant is pointed to a production instance used by your end users. This environment should be treated carefully since it could break your application."}
                                                </p>
                                            </div>
                            
                                        </div>
                                    </div>
                                </div>

                            </div>
                            
                            <div
                                class="mt-3 mb-5"
                            >
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_update_environment_tag {"loading"} else {""} )
                                    onclick=self.link.callback(|_| Msg::UpdateEnvironmentTag)
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                {
                                    if self.error_update_environment_tag.is_some() {
                                    html! {
                                        <div class="alert alert-warning mt-3" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_update_environment_tag.clone().unwrap() }
                                        </div>
                                    }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
            
                        </div>
                    </div>
            
            
                    <div
                        class="row border-bottom mt-5"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                        >
                            {"API Authorization Settings"}
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Default Audience"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="https://your-default-endpoint/"
                                        // value={default_audience}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::DefaultAudience))
                                        disabled={ if self.loading_update_authorization {true} else {false} }
                                    />
                                </div>
                                <p>
                                    {"API Audience to use by default for API Authorization flows . Note: This setting is equivalent to appending the audience to every authorization request made to the tenant for every application. This will cause new behavior that might result in breaking changes for some of your applications. If you require assistance, contact support."}
                                </p>
                            </div>
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Default Directory"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="Connection Name"
                                        // value={default_directory}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::DefaultDirectory))
                                        disabled={ if self.loading_update_authorization {true} else {false} }
                                    />
                                </div>
                                <p>
                                    {"Name of the connection to be use for Password Grant exchanges. The default_directory value should be the exact name of an existing connections of one of the following strategies: ad, auth0, email, sms, waad, adfs"}
                                </p>
                            </div>

                            <div
                                class="mt-3 mb-5"
                            >
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_update_authorization {"loading"} else {""} )
                                    onclick=self.link.callback(|_| Msg::UpdateAuthorization)
                                    disabled={ if self.loading_update_authorization {true} else {false} }
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                {
                                    if self.error_update_authorization.is_some() {
                                    html! {
                                        <div class="alert alert-warning mt-3" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_update_authorization.clone().unwrap() }
                                        </div>
                                    }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
            
                        </div>
                    </div>


                    <div
                        class="row border-bottom mt-5"
                    >
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="text-color-primary fw-bold mb-4"
                                style="font-size: 16px;"
                            >
                                {"Error Pages"}
                            </div>
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Default Error Page"}
                                </p>
                                <div
                                    class="card card-hover mb-2"
                                    style=format!("cursor: pointer; {}", if self.loading_update_error_page {"pointer-events: none;"} else {""} )
                                    onclick=self.link.callback(|_| Msg::InputBool(false, Data::ErrorPage))
                                >
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <input
                                                class="form-check-input me-2"
                                                type="radio"
                                                name="flexRadioDefault"
                                                id="flexRadioDefault1"
                                                // checked={ if error_page.show_log_link { false } else { true } }
                                            />

                                            <div
                                                class="d-grid"
                                                style="min-width: 40px;"
                                            >
                                                <div
                                                    class="text-decoration-none fw-bold mb-0"
                                                >
                                                    <span
                                                        class="fw-bold"
                                                        style="
                                                            white-space: nowrap;
                                                            text-overflow: ellipsis;
                                                            overflow: hidden;
                                                            text-decoration: none;
                                                        "
                                                    >
                                                        {"Generic"}
                                                    </span>
                                                </div>
                                                <p
                                                    class="mb-0 text-muted"
                                                >
                                                    {"Use a generic error page generated from your account data"}
                                                </p>
                                            </div>
                            
                                        </div>
                                    </div>
                                </div>
                                <div
                                    class="card card-hover mb-4"
                                    style=format!("cursor: pointer; {}", if self.loading_update_error_page {"pointer-events: none;"} else {""} )
                                    onclick=self.link.callback(|_| Msg::InputBool(true, Data::ErrorPage))
                                >
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <input
                                                class="form-check-input me-2"
                                                type="radio"
                                                name="flexRadioDefault"
                                                id="flexRadioDefault2"
                                                // checked={ if error_page.show_log_link { true } else { false } }
                                            />

                                            <div
                                                class="d-grid"
                                                style="min-width: 40px;"
                                            >
                                                <div
                                                    class="text-decoration-none fw-bold mb-0"
                                                >
                                                    <span
                                                        class="fw-bold"
                                                        style="
                                                            white-space: nowrap;
                                                            text-overflow: ellipsis;
                                                            overflow: hidden;
                                                            text-decoration: none;
                                                        "
                                                    >
                                                        {"Custom"}
                                                    </span>
                                                </div>
                                                <p
                                                    class="mb-0 text-muted"
                                                >
                                                    {"Redirect users to a specified URL instead of showing the default error page"}
                                                </p>
                                            </div>
                            
                                        </div>
                                    </div>
                                </div>

                                // {
                                //     if self.tenant_settings.error_page.show_log_link {
                                //         html!{
                                //             <div
                                //                 class="mb-4"
                                //             >
                                //                 <p class="mb-2 fw-bold">
                                //                     {"Custom error page URL *"}
                                //                 </p>
                                //                 <div class="input-group mb-2">
                                //                     <input
                                //                         type="text"
                                //                         class="form-control bg-input-grey"
                                //                         aria-label="Dollar amount (with dot and two decimal places)"
                                //                         placeholder="http://mycompany.com/error/"
                                //                         value={error_page.url}
                                //                         oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::ErrorPage))
                                //                         disabled={ if self.loading_update_error_page {true} else {false} }
                                //                     />
                                //                 </div>
                                //             </div>
                                //         }
                                //     } else {
                                //         html! {}
                                //     }
                                // }

                            </div>
                            
                            <div
                                class="mt-3 mb-5"
                            >
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_update_error_page {"loading"} else {""} )
                                    onclick=self.link.callback(|_| Msg::UpdateErrorPage)
                                    disabled={ if self.loading_update_error_page {true} else {false} }
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                {
                                    if self.error_update_error_page.is_some() {
                                    html! {
                                        <div class="alert alert-warning mt-3" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_update_error_page.clone().unwrap() }
                                        </div>
                                    }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
            
                        </div>
                    </div>

                    <div
                        class="row mt-5"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                            style="font-size: 16px;"
                        >
                            {"Languages"}
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Default Language"}
                                </p>
                                <select class="form-select" aria-label="Default select example">
                                    <option selected=true>{"Open this select menu"}</option>
                                    <option value="1">{"English (en)"}</option>
                                    <option value="2">{"Indonesian (id)"}</option>
                                </select>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Support Languages"}
                                </p>
                                <div
                                    class="d-flexs"
                                >
                                    <div
                                        class="form-check mb-2 d-flex align-items-center d-inline-flex"
                                        style="width: 49%;"
                                    >
                                        <input class="form-check-input me-2 mt-0" style="font-size: 16px;" type="checkbox"/>
                                        <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                            {"English (en)"}
                                        </label>
                                    </div>

                                    <div
                                        class="form-check mb-2 d-flex align-items-center w-50 d-inline-flex"
                                        style="width: 49%;"
                                    >
                                        <input class="form-check-input me-2 mt-0" style="font-size: 16px;" type="checkbox"/>
                                        <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                            {"Indonesian (id)"}
                                        </label>
                                    </div>

                                    <div
                                        class="form-check mb-2 d-flex align-items-center w-50 d-inline-flex"
                                        style="width: 49%;"    
                                    >
                                        <input class="form-check-input me-2 mt-0" style="font-size: 16px;" type="checkbox"/>
                                        <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                            {"Croatian (hr)"}
                                        </label>
                                    </div>

                                </div>
                            </div>

                            <div
                                class="mt-3 mb-5"
                            >
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_update_language {"loading"} else {""} )
                                    onclick=self.link.callback(|_| Msg::UpdateLanguage)
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                {
                                    if self.error_update_language.is_some() {
                                    html! {
                                        <div class="alert alert-warning mt-3" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_update_language.clone().unwrap() }
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
            </div>
        }
    }
}