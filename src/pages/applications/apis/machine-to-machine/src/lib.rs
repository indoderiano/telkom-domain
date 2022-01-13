use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
    }
};
use types::{
	api::{ ApiDetails, ResponseApiDetails, Application },
	ResponseMessage,
};
use serde::{
    Serialize,
};
use loading::Loading;
use configs::server::API_URL;


pub enum StateError {
    RequestApplications,
    UpdateApplication,
}

pub struct MachineToMachineApplications {
    applications_list: Vec<Application>,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    loading_request_applications: bool,
    error_request_applications: Option<String>,
    loading_update_status: bool,
}

pub enum Msg {
    RequestApplications,
    GetApplications(Vec<Application>),
    UpdateStatus(bool, String),
    ResponseError(String, StateError),
}

impl Component for MachineToMachineApplications {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {

        MachineToMachineApplications {
            applications_list: vec![],
            link,
            fetch_task: None,
            loading_request_applications: false,
            error_request_applications: None,
            loading_update_status: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestApplications);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestApplications => {
                let request = Request::get(format!("{}/api/v2/tenantidnotfromreducer/resource-servers/applications", API_URL))
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<Application>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", dataok));
                                Msg::GetApplications(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::RequestApplications)
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_request_applications = None;
                self.loading_request_applications = true;
                true
            }
            Msg::GetApplications(data) => {
                self.applications_list = data;
                self.fetch_task = None;
                self.loading_request_applications = false;
                self.loading_update_status = false;
                true
            }
            Msg::UpdateStatus (status, id) => {
                #[derive(Serialize)]
                struct UpdateStatus {
                    status: bool
                }
                let update_status = UpdateStatus {
                    status,
                };
                let request = Request::put(format!("{}/api/v2/tenantidnotfromreducer/resource-servers/applications/{}", API_URL, id))
                    .header("Content-Type", "application/json")
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Json(&update_status))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<Application>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", dataok));
                                Msg::GetApplications(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::RequestApplications)
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_request_applications = None;
                self.loading_update_status = true;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::RequestApplications => {
                        self.fetch_task = None;
                        self.loading_request_applications = false;
                        self.error_request_applications = Some(message);
                        true
                    }
                    StateError::UpdateApplication => {
                        self.fetch_task = None;
                        true
                    }
                }
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.loading_request_applications {
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
        } else if self.error_request_applications.is_some() {
            html! {
                <div class="alert alert-warning mb-5" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    { self.error_request_applications.clone().unwrap() }
                </div>
            }
        } else {
            html! {
                <div
                    class="pb-5"
                    style="font-size: 14px;"
                >
        
                    <p>{"Here is a list of your Machine to Machine Applications. You can authorize these to request access tokens for this API by executing a client credentials exchange."}</p>
                    <p>{"Single Page and Native apps do not require further configuration. SPAs can execute the Implicit Grant to access APIs while Native Apps can do Authorize Code with PKCE for the same purpose."}</p>
        
                    <div class="mb-3">
                        <input
                            type="text"
                            class="form-control"
                            placeholder="Filter by Application Name or Client ID"
                        />
                    </div>

                    { self.view_list() }
        
                </div>
            }
        }
    }
}

impl MachineToMachineApplications {
    fn view_list (&self) -> Vec<Html> {
        self.applications_list.clone().iter().map(|app| {
            let Application{
                name,
                client_id,
                status,
            } = app.clone();
            html! {
                <div
                    class="d-flex border-bottom border-1 list-hover justify-content-between align-items-center"
                >
                    <div
                        class="p-3 d-flex"
                        style="width: 40%;"
                    >

                        <div
                            class="d-grid"
                            style="min-width: 40px;"
                        >
                            <span
                                class="fw-bold text-color-secondary"
                                style="
                                    white-space: nowrap;
                                    text-overflow: ellipsis;
                                    overflow: hidden;
                                    font-size: 14px;
                                    text-decoration: none;
                                "
                            >
                                { name }
                            </span>
                            <p
                                class="mb-0 text-muted text-color-primary"
                                style="
                                    white-space: nowrap;
                                    text-overflow: ellipsis;
                                    overflow: hidden;
                                    font-size: 14px;
                                "
                            >
                                <span
                                style="font-size: 14px; margin-right: 8px; white-space: nowrap;"
                            >
                                {"Client Id:"} 
                            </span>
                            <span
                                class="rounded"
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
                                { client_id.clone() }
                            </span>
                            </p>
                        </div>

                    </div>
                    <div
                        class="d-flex"
                        style="align-items: center;"
                    >
                        <span
                            class="me-2 fs-6"
                        >
                            {
                                if app.status {
                                    "Authorized"
                                } else {
                                    "Unauthorized"
                                }
                            }
                        </span>
                        <div class="form-check form-switch fs-4 d-inline-block">
                            <input
                                class="form-check-input"
                                type="checkbox"
                                checked={ status.clone() }
                                onclick=self.link.callback(move |_| Msg::UpdateStatus(!status.clone(), client_id.clone()))
                                disabled={ self.loading_update_status.clone() }
                            />
                        </div>
                    </div>
                    
                </div>
            }
        })
        .collect()
    }
}