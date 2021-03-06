use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{ FetchService, FetchTask, Request, Response },
        storage::{ Area, StorageService },
        ConsoleService,
    },
};
use yew_router::components::RouterAnchor;
use router::AppRoute;
use types::{
    roles::Role,
    LocalStorage,
    LOCALSTORAGE_KEY,
};
use configs::server::API_URL;
use loading::Loading;
mod raw;
mod context_data;
use raw::Raw;
use context_data::ContextData;


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct LogDetailsProps {
    pub tenant_id: String,
    pub log_id: String,
}

pub enum StateError {
    RequestLogDetails,
}

pub enum Content {
    Raw,
    ContextData,
}

pub struct LogDetails {
    access_token: String,
    link: ComponentLink<Self>,
    tenant_id: String,
    log_id: String,
    fetch_task: Option<FetchTask>,
    log: String,
    loading_request_log: bool,
    error_request_log: Option<String>,
    content: Content,
}

pub enum Msg {
    RequestLogDetails,
    GetLogDetails(String),
    ChangeContent(Content),
    ResponseError(String, StateError),
}

impl Component for LogDetails {
    type Message = Msg;
    type Properties = LogDetailsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        let storage = StorageService::new(Area::Local).expect("storage was disabled");
        let localstorage_data = {
            if let Json(Ok(data)) = storage.restore(LOCALSTORAGE_KEY) {
                data
            } else {
                LocalStorage {
                    username: None,
                    email: None,
                    token: None,
                }
            }
        };

        let mut access_token = String::from("");

        if let Some(token) = localstorage_data.token {
            access_token = token;
        } else {}
        
        LogDetails {
            access_token,
            link,
            tenant_id: props.tenant_id,
            log_id: props.log_id,
            fetch_task: None,
            log: String::from("LOG"),
            loading_request_log: false,
            error_request_log: None,
            content: Content::Raw,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // self.link.send_message(Msg::RequestLogDetails);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestLogDetails => {
                // let request = Request::get(format!("{}/api/v2/roles/{}", API_URL, self.role_id.clone()))
                //     .header("access_token", self.access_token.clone())
                //     .body(Nothing)
                //     .expect("Could not build request.");
                // let callback =
                //     self.link
                //         .callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                //             let Json(data) = response.into_body();
                //             match data {
                //                 Ok(dataok) => {
                //                     Msg::GetLogDetails(dataok)
                //                 }
                //                 Err(error) => Msg::ResponseError(
                //                     error.to_string(),
                //                     StateError::RequestLogDetails,
                //                 ),
                //             }
                //         });
                // let task = FetchService::fetch(request, callback).expect("failed to start request");
                // self.fetch_task = Some(task);
                // self.error_request_log = None;
                // self.loading_request_log = true;
                true
            }
            Msg::GetLogDetails(data) => {
                self.log = data;
                self.fetch_task = None;
                self.loading_request_log = false;
                true
            }
            Msg::ChangeContent(content) => {
                self.content = content;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::RequestLogDetails => {
                        self.loading_request_log = false;
                        self.error_request_log = Some(message);
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
        html! {
            <div>
                <div class="domain-content">
                    <div>
                        <Anchor route=AppRoute::LogsHome { tenant_id: self.tenant_id.clone() } classes="text-decoration-none text-muted">
                            <i class="bi bi-arrow-left"></i>
                            <span>{"Back To Logs"}</span>
                        </Anchor>
                    </div>

                    <div class="mt-2">
                        <h2 class="title">{ "Logs" }</h2>
                    </div>

                    {
                        if self.loading_request_log {
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
                        } else if self.error_request_log.is_some() {
                            html! {
                                <div class="alert alert-warning mb-5" role="alert">
                                    <i class="bi bi-exclamation-triangle me-2"></i>
                                    { self.error_request_log.clone().unwrap() }
                                </div>
                            }
                        } else {
                            self.view_content()
                        }
                    }

                </div>
            </div>


        }
    }
}

impl LogDetails {
    fn view_content(&self) -> Html {
        html! {
            <>
                <div class="card p-3 mt-3">
                    <div class="card-body">
                        <div>
                            <div class="fw-bold" style="font-size: 16px;">
                                {"Summary"}
                            </div>
                            <div class="row mt-3">
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"Occured"}</p>
                                    <p class="mb-1">{"4 hours ago"}</p>
                                    <p class="mb-1">{"at 2022-01-31 12:35:50.341 UTC"}</p>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1 ">{"Type"}</p>
                                    <p class="mb-1">{"API Read Operation"}</p>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"Description"}</p>
                                    <p class="mb-1">{"Get a client"}</p>
                                </div>
                            </div>
                            <div class="row mt-3">
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"Connection"}</p>
                                    <p class="mb-1">{"N/A"}</p>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4 mb-1">
                                    <p class="text-muted mb-1">{"Application"}</p>
                                    <p class="mb-1">{"MrlpRDQKGK9ENLgHd89jWTMkKvf0O7t9"}</p>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"User"}</p>
                                    <p>{"N/A"}</p>
                                </div>
                            </div>

                        </div>
                    </div>
                </div>

                <div class="card p-3 mt-3">
                    <div class="card-body">
                        <div class="fw-bold" style="font-size: 16px;">
                            {"Details"}
                        </div>

                        <ul class="nav nav-tabs mt-4" id="myTab" role="tablist" style="font-size:14px;">
                            <li
                                onclick = self.link.callback(|_|Msg::ChangeContent(Content::Raw))
                                class="nav-item fw-bold">
                                <button
                                    class={
                                        match self.content {
                                            Content::Raw => "nav-link active",
                                            _ => "nav-link"
                                        }
                                    }
                                    id="user-details-tab"
                                    data-bs-toggle="tab"
                                    data-bs-target="#detailtab"
                                    type="button" role="tab"
                                    aria-controls="detailtab"
                                    aria-selected="true">{"Raw"}
                                </button>
                            </li>
                            <li
                                onclick =self.link.callback(|_|Msg::ChangeContent(Content::ContextData))
                                class="nav-item fw-bold"
                                role="presentation"
                            >
                                <button
                                    class={
                                        match self.content {
                                            Content::ContextData => "nav-link active",
                                            _ => "nav-link"
                                        }
                                    }
                                    id="user-devices-tab"
                                    data-bs-toggle="tab"
                                    data-bs-target="#devicetab"
                                    type="button" role="tab"
                                    aria-controls="devicetab"
                                    aria-selected="false">{"Context Data"}
                                </button>
                            </li>
                        </ul>

                        {
                            match self.content {
                                Content::Raw => html! {<Raw/>},
                                Content::ContextData => html! {<ContextData/>},
                            }
                        }
                    </div>
                </div>
            </>
        }
    }
}
