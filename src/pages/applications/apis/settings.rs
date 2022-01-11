use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_router::components::RouterAnchor;
// use crate::app::AppRoute;
use router::AppRoute;
use super::quickstart::Quickstart;
use super::tab_settings::TabSettings;
use super::permissions::Permissions;
use super::machinetomachineapplications::MachineToMachineApplications;
use yew::services::{
    ConsoleService,
    storage::{ StorageService, Area },
};

use crate::types::api::{ ApiDetails, ResponseApiDetails };
use crate::configs::server::API_URL;
use crate::components::{
    loading2::Loading2,
    developers_note::DevelopersNote,
};
use crate::types::LocalStorage;
use crate::types::LOCALSTORAGE_KEY;


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ApisSettingsProps {
    // pub tenant_id: String,
    pub resource_server_id: String,
    // api_title: ApiTitle,
}

pub enum Content {
    Quickstart,
    Settings,
    Permissions,
    MachineToMachineApplications,
}

pub struct ApisSettings {
    content: Content,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    error: Option<String>,
    api_details: ApiDetails,
    resource_server_id: String,
    access_token: String,
}

pub enum Msg {
    ChangeContent(Content),
    RequestApiDetails,
    GetApiDetails(Result<ApiDetails, anyhow::Error>),
}

impl Component for ApisSettings {
    type Message = Msg;
    type Properties = ApisSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // ConsoleService::info(&format!("Api Settings props, tenant id = {}", props.tenant_id));
        ConsoleService::info(&format!("Api Settings props, api id = {}", props.resource_server_id));

        let storage = StorageService::new(Area::Local).expect("storage was disabled");
        let localstorage_data = {
            if let Json(Ok(data)) = storage.restore(LOCALSTORAGE_KEY) {
                ConsoleService::info(&format!("{:?}", data));
                data
            } else {
                ConsoleService::info("token does not exist");
                LocalStorage {
                    username: None,
                    email: None,
                    token: None,
                }
            }
        };

        ConsoleService::info(&format!("{:?}", localstorage_data));

        // IF LOCALSTORAGE EXISTS
        // UPDATE STATE
        let mut access_token = String::from("");
        if let Some(_) = localstorage_data.token {
            access_token = localstorage_data.token.unwrap();
        } else {
            
        }

        let api_details = ApiDetails::new();

        ApisSettings {
            content: Content::Quickstart,
            link,
            fetch_task: None,
            error: None,
            api_details,
            resource_server_id: props.resource_server_id,
            access_token,
        }
    }

    fn rendered(&mut self, first_render: bool) {

        if first_render {
            ConsoleService::info("This is first render");
            
            self.link.send_message(Msg::RequestApiDetails);
        }

    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeContent(content) => {
                self.content = content;
                true
            }
            Msg::RequestApiDetails => {
                let request = Request::get(format!("{}/api/v2/resource-server/{}", API_URL, self.resource_server_id))
                    // .header("Content-Type", "application/json")
                    .header("access_token", self.access_token.clone())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<ApiDetails, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::GetApiDetails(data)
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::GetApiDetails(response) => {
                match response {
                    Ok(data) => {
                        ConsoleService::info(&format!("{:?}", data));
                        self.api_details = data;
                    }
                    Err(error) => {
                        ConsoleService::info(&error.to_string());
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        // let tenant_id = self.api_details.tenant_id.clone();
        let tenant_id = String::from("tenant_id_not_from_reducer");
        html! {
            <div
                class="py-5 px-4 m-auto"
                style="max-width: 1048px; font-size:14px;"
            >
                <Anchor
                    route=AppRoute::ApisHome
                    classes="text-decoration-none domain-link-dark"
                >
                    <i class="bi bi-arrow-left me-2"></i>
                    {"Back to Apis"}
                </Anchor>

                {
                    if self.fetch_task.is_some() {
                        html! {
                            <div
                                style="
                                    position: relative;
                                    margin-top: 8rem;
                                "
                            >
                                <Loading2 width=45 />
                            </div>
                        }
                    } else {
                        html! {
                            { self.view_content() }
                        }
                    }
                }

                
            </div>
        }
    }
}


impl ApisSettings {
    fn view_content (&self) -> Html {
        let ApiDetails {
            // id: _,
            // name,
            // api_id: _,
            // api_type,
            // identifier,
            // token_exp: _,
            // token_exp_browser: _,
            // sign_algorithm: _,
            // rbac: _,
            // permission_acc_token: _,
            // allow_skip_user: _,
            // allow_off_acc: _,
            // tenant_id: _,

            resource_server_id: _,
            name,
            is_system,
            identifier,
            scopes: _,
            signing_alg: _,
            signing_secret: _,
            allow_offline_access: _,
            skip_consent_for_variable_first_party_clients: _,
            token_lifetime: _,
            token_lifetime_for_web: _,
            enforce_policies: _,
            token_dialect: _,
            client: _,

        } = self.api_details.clone();

        html! {
            <>
                <div
                    class="d-flex mb-5 mt-3"
                >
                    <div
                        style="flex: 0 0 auto; width: 64px; height: 64px; background-color: #eff0f2;"
                        class="d-flex justify-content-center align-items-center rounded me-4"
                    >
                        <i class="bi bi-server fs-3"></i>
                    </div>

                    <div
                        class="d-flex flex-column"
                    >
                        <h2>{name}</h2>
                        <div
                            class="text-muted"
                        >
                            <span
                                class="me-4"
                            >
                                { if is_system {"System API"} else {"Custom API"} }
                            </span>
                            <span>
                                {"Identifier"}
                            </span>
                            <span
                                class="rounded ms-2"
                                style="
                                    background-color: #eff0f2;
                                    white-space: nowrap;
                                    text-overflow: ellipsis;
                                    overflow: hidden;
                                    font-size: 14px;
                                    padding: 2px 6px;
                                    font-family: 'Roboto Mono', monospace;
                                "
                            >
                                { identifier }
                            </span>
                        </div>
                    </div>
                </div>

                <DevelopersNote message="Only the following tabs are working, 'Settings', 'Permissions'"/>

                <div
                    class="mb-4"
                >
                    <ul class="nav nav-tabs">
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Quickstart))
                            class="nav-item"
                        >
                        <a
                            // class="nav-link active"
                            class={
                                match self.content {
                                    Content::Quickstart => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            aria-current="page"
                            href="#"
                        >
                            {"Quick Start"}</a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Settings))
                            class="nav-item">
                        <a
                            // class="nav-link"
                            class={
                                match self.content {
                                    Content::Settings => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            href="#">{"Settings"}</a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Permissions))
                            class="nav-item">
                        <a
                            // class="nav-link"
                            class={
                                match self.content {
                                    Content::Permissions => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            href="#">{"Permissions"}</a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::MachineToMachineApplications))
                            class="nav-item">
                        <a
                            // class="nav-link"
                            class={
                                match self.content {
                                    Content::MachineToMachineApplications => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            href="#">{"Machine to Machine Applications"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{"Test"}</a>
                        </li>
                    </ul>
                </div>

                {
                    match self.content {
                        Content::Quickstart => html! { <Quickstart/> },
                        Content::Settings => html! { <TabSettings api_details=self.api_details.clone() /> },
                        Content::Permissions => html! { <Permissions api_details=self.api_details.clone() /> },
                        Content::MachineToMachineApplications => html! { <MachineToMachineApplications/> },
                    }
                }

            </>
        }
    }
}