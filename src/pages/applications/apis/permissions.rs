use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
        storage::{ StorageService, Area },
    }
};
use types::{
	api::{ ApiDetails, ResponseApiDetails, Scope },
	ResponseMessage,
    LocalStorage,
    LOCALSTORAGE_KEY,
};
use crate::configs::server::API_URL;


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct PermissionsProps {
    pub api_details: ApiDetails,
}
pub struct Permissions {
    api_details: ApiDetails,
    scopes: Vec<Scope>,
    new_scope: Scope,
    access_token: String,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    loading_add_permission: bool,
    error_add_permission: Option<String>,
    loading_delete_permission: bool,
    error_delete_permission: Option<String>,
    error_request_api_details: Option<String>,
}

enum DataPermissionAdd {
    Value,
    Description,
}

#[derive(Debug, Clone, Copy)]
enum StateError {
    AddPermission,
    DeletePermission,
    RequestApiDetails,
}

pub enum Msg {
    Input(String, DataPermissionAdd),
    AddPermission,
    DeletePermission(usize),
    RequestUpdateApiDetails(ApiDetails, StateError),
    RequestApiDetails,
    GetApiDetails(ApiDetails),
    ResponseError(String, StateError),
}

impl Component for Permissions {
    type Message = Msg;
    type Properties = PermissionsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // ConsoleService::info(&format!("Permissions props, api details = {:?}", props.api_details));

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
        if localstorage_data.token.is_some() {
            access_token = localstorage_data.token.unwrap();
        } else {

        }


        Permissions {
            api_details: props.api_details.clone(),
            scopes: props.api_details.scopes.clone(),
            new_scope: Scope {
                value: "".to_string(),
                description: "".to_string(),
            },
            access_token,
            fetch_task: None,
            link,
            loading_add_permission: false,
            error_add_permission: None,
            loading_delete_permission: false,
            error_delete_permission: None,
            error_request_api_details: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(input, data) => {
                match data {
                    DataPermissionAdd::Value => {
                        self.new_scope.value = input;
                    }
                    DataPermissionAdd::Description => {
                        self.new_scope.description = input;
                    }
                }
                true
            }
            Msg::AddPermission => {
                // VALIDATION
                if self.new_scope.value.is_empty() || self.new_scope.description.is_empty() {
                    self.link.send_message(Msg::ResponseError("Input is empty".to_string(), StateError::AddPermission));
                    return false
                }

                let mut new_api_details = self.api_details.clone();
                new_api_details.scopes.push(self.new_scope.clone());
                self.link.send_message(Msg::RequestUpdateApiDetails(new_api_details, StateError::AddPermission));
                self.error_add_permission = None;
                self.loading_add_permission = true;
                false
            }
            Msg::DeletePermission(index_delete) => {
                let new_scopes = self.api_details.scopes
                .clone()
                .iter()
                .enumerate()
                .filter(|(index, _data)| {
                    *index != index_delete
                })
                .map(|(index, data)| {
                    data.clone()
                })
                .collect::<Vec<Scope>>();

                let mut new_api_details = self.api_details.clone();
                new_api_details.scopes = new_scopes;
                
                self.link.send_message(Msg::RequestUpdateApiDetails(new_api_details, StateError::DeletePermission));
                self.error_delete_permission = None;
                self.loading_delete_permission = true;
                false
            }
            Msg::RequestUpdateApiDetails(new_api_details, state) => {
                let request = Request::patch(format!("{}/api/v2/resource-server/{}", API_URL, self.api_details.resource_server_id.clone()))
                    .header("Content-Type", "application/json")
                    .header("access_token", self.access_token.clone())
                    .body(Json(&new_api_details))
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    move |response: Response<Json<Result<ApiDetails, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(response) => {
                                Msg::RequestApiDetails
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), state.clone())
                            }
                        }
                    }
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::RequestApiDetails => {
                let request = Request::get(format!("{}/api/v2/resource-server/{}", API_URL, self.api_details.resource_server_id))
                    .header("access_token", self.access_token.clone())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<ApiDetails, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                Msg::GetApiDetails(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::RequestApiDetails)
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::GetApiDetails(data) => {
                // DEFAULT STATE
                self.loading_add_permission = false;
                self.loading_delete_permission = false;
                self.fetch_task = None;
                
                // CLEAR INPUT
                self.new_scope = Scope {
                    value: "".to_string(),
                    description: "".to_string(),
                };

                self.api_details = data;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::AddPermission => {
                        self.loading_add_permission = false;
                        self.error_add_permission = Some(message);
                    }
                    StateError::DeletePermission => {
                        self.loading_delete_permission = false;
                        self.error_delete_permission = Some(message);
                    }
                    StateError::RequestApiDetails => {
                        self.error_request_api_details = Some(message);
                    }
                }
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {

            <div
                style="font-size: 14px;"
            >

                <div class="mb-4">

                    <div>
                        <div
                            class="fs-4 fw-bold"
                        >
                            {"Add a Permissions (Scope)"}
                        </div>
                        <p>{"Define the permissions (scopes) that this API uses."}</p>
                    </div>

                    <ul class="list-group list-group-flush">
                        <li class="list-group-item border-bottom-0 pb-0">
                            <div
                                class="d-flex"
                            >
                                <div
                                    class="flex-fill fw-bold"
                                >
                                    {"Permission"}
                                </div>
                                <div
                                    class="flex-fill fw-bold"
                                >
                                    {"Description"}
                                </div>
                                <div
                                    class="flex-shrink-1"
                                    style="min-width: 81px;"
                                >
                                </div>
                            </div>
                        </li>

                        <li class="list-group-item">
                            <div
                                class="d-flex"
                            >
                                <div
                                    class="flex-fill pe-2"
                                >
                                    <input
                                        type="text"
                                        class="form-control"
                                        value={ self.new_scope.value.clone() }
                                        oninput=self.link.callback(|data: InputData| Msg::Input(data.value, DataPermissionAdd::Value))
                                        disabled={ self.loading_add_permission }
                                    />
                                </div>
                                <div
                                    class="flex-fill pe-2"
                                >
                                    <input
                                        type="text"
                                        class="form-control"
                                        value={ self.new_scope.description.clone() }
                                        oninput=self.link.callback(|data: InputData| Msg::Input(data.value, DataPermissionAdd::Description))
                                        disabled={ self.loading_add_permission }
                                    />
                                </div>
                                <div
                                    class="flex-shrink-1"
                                    style="min-width: 81px;"
                                >
                                    <button
                                        type="button"
                                        class=format!("btn {} btn-outline-secondary position-relative", if self.loading_add_permission {"loading"} else {""} )
                                        onclick=self.link.callback(|_| Msg::AddPermission)
                                        disabled={ if self.loading_add_permission {true} else {false} }
                                    >
                                        <div class="telkom-label">
                                            <i class="bi bi-plus-lg me-1"></i>
                                            <span>{ "Add" }</span>
                                        </div>
                                        <div class="telkom-spinner telkom-center">
                                            <div class="spinner-border spinner-border-sm" role="status"/>
                                        </div>
                                    </button>
                                </div>
                            </div>
                        </li>
                        
                    </ul>

                    {
                        if self.error_add_permission.is_some() {
                            html! {
                                <div class="alert alert-warning mb-5" role="alert">
                                    <i class="bi bi-exclamation-triangle me-2"></i>
                                    { self.error_add_permission.clone().unwrap() }
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                
                <div>
                    <div>
                        <div
                            class="fs-4 fw-bold"
                        >
                            {"List of Permissions (Scopes)"}
                        </div>
                        <p>{"These are all the permissions (scopes) that this API uses."}</p>
                    </div>

                    {
                        if self.error_request_api_details.is_some() {
                            html! {
                                <div class="alert alert-warning mb-5" role="alert">
                                    <i class="bi bi-exclamation-triangle me-2"></i>
                                    { self.error_request_api_details.clone().unwrap() }
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }

                    {
                        if self.error_delete_permission.is_some() {
                            html! {
                                <div class="alert alert-warning mb-5" role="alert">
                                    <i class="bi bi-exclamation-triangle me-2"></i>
                                    { self.error_delete_permission.clone().unwrap() }
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                    
                    <div class=format!("position-relative {}", if self.loading_delete_permission {"loading"} else {""} )>
                        <ul
                            class="list-group list-group-flush telkom-label apis-permissions"
                        >
                            <li class="list-group-item">
                                <div
                                    class="d-flex"
                                >
                                    <div
                                        class="flex-fill fw-bold"
                                    >
                                        {"Permission"}
                                    </div>
                                    <div
                                        class="flex-fill fw-bold"
                                    >
                                        {"Description"}
                                    </div>
                                    <div
                                        class="flex-shrink-1"
                                        style="min-width: 81px;"
                                    >
                                    </div>
                                </div>
                            </li>

                            { self.view_list() }

                        </ul>

                        <div class="telkom-spinner telkom-center">
                            <div class="spinner-border spinner-border-sm" role="status"/>
                        </div>

                    </div>
                </div>

            </div>
        }
    }
}

impl Permissions {
    fn view_list (&self) -> Vec<Html> {
        self.api_details.scopes
        .clone()
        .iter()
        .enumerate()
        .map(|(index, scope)| {
            html! {
                <li class="list-group-item">
                    <div
                        class="d-flex"
                    >
                        <div
                            class="flex-fill"
                        >
                            { scope.value.clone() }
                        </div>
                        <div
                            class="flex-fill"
                        >
                            { scope.description.clone() }
                        </div>
                        <div
                            class="flex-shrink-1"
                            style="min-width: 81px;"
                        >
                            // <button
                            //     type="button"
                            //     class="btn btn-outline-secondary"
                            // >
                            //     <i class="bi bi-trash"></i>
                            // </button>
                            <button
                                type="button"
                                class="btn btn-outline-secondary"
                                onclick=self.link.callback( move |_| Msg::DeletePermission(index.clone()))
                                disabled={ self.loading_delete_permission }
                            >
                                <i class="bi bi-trash"></i>
                            </button>
                        </div>
                    </div>
                </li>
            }
        })
        .collect()
    }
}