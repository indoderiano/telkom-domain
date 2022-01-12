use yew::{
    prelude::*,
    format::{Json, Nothing},
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
        storage::{ StorageService, Area }
    },
};
use serde::Serialize;
use crate::components::{
    loading2::Loading2,
    developers_note::DevelopersNote,
};
use crate::configs::server::API_URL;
use crate::types::{
    users::{ UserPermissions },
    api::{ ApiTitle, Scope },
    ResponseMessage,
    LocalStorage,
    LOCALSTORAGE_KEY,
};

#[derive(Serialize, Debug, Clone, PartialEq)]
struct SelectedPermission {
    permission_name: String,
    resource_server_identifier: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ModalAssignPermissionsProps {
    pub user_permissions: Vec<UserPermissions>,
    pub user_id: String,
}

pub struct ModalAssignPermissions {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    access_token: String,
    user_permissions: Vec<UserPermissions>,
    user_id: String,
    loading_get_apis: bool,
    error_get_apis: Option<String>,
    apis: Vec<ApiTitle>,
    // selected_api_id: Option<String>,
    // selected_api_name: Option<String>,
    selected_api: Option<ApiTitle>,
    option_permissions: Option<Vec<Scope>>,
    selected_permissions: Vec<SelectedPermission>,
    error_assign_permissions: Option<String>,
    loading_assign_permissions: bool,
    message: Option<String>,
}

pub enum StateError {
    RequestApis,
    RequestAssignPermissions,
}
pub enum Msg {
    RequestApis,
    GetApis(Vec<ApiTitle>),
    SelectApi(String),
    SelectPermission(usize),
    RequestAssignPermissions,
    GetResponseAssignPermissions(String),
    ResponseError(String, StateError),
}

impl Component for ModalAssignPermissions {
    type Message = Msg;
    type Properties = ModalAssignPermissionsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // GET LOCALSTORAGE
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

        // UPDATE STATE
        let mut access_token = String::from("");
        if let Some(_) = localstorage_data.token {
            access_token = localstorage_data.token.unwrap();
        } else {
            
        }

        ConsoleService::info(&format!("user permissions = {:?}", props.user_permissions));

        ModalAssignPermissions {
            link,
            fetch_task: None,
            access_token,
            user_permissions: props.user_permissions,
            user_id: props.user_id,
            loading_get_apis: false,
            error_get_apis: None,
            apis: Vec::new(),
            // selected_api_id: None,
            // selected_api_name: None,
            selected_api: None,
            option_permissions: None,
            selected_permissions: Vec::new(),
            error_assign_permissions: None,
            loading_assign_permissions: false,
            message: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestApis);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestApis => {
                let request = Request::get(format!("{}/api/v2/resource-server", API_URL))
                    .header("access_token", self.access_token.clone())
                    .body(Nothing)
                    .expect("Could not build request");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<ApiTitle>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        // ConsoleService::info(&format!("{:?}", data));
                        match data{
                            Ok(dataok) => Msg::GetApis(dataok), 
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::RequestApis)
                            }
                        }
                    }
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_get_apis = None;
                self.loading_get_apis = true;
                true
            }
            Msg::GetApis(apis) => {
                self.apis = apis;
                self.loading_get_apis = false;
                self.fetch_task = None;
                true
            }
            Msg::SelectApi(index) => {
                ConsoleService::info(&format!("index = {}", index));
                let index_integer = index.parse::<usize>().unwrap();
                if index.is_empty() {
                    ConsoleService::info("index is empty");
                } else {
                    // ConsoleService::info(&format!("selected api id = {}", self.apis[index_integer].resource_server_id));
                    // ConsoleService::info(&format!("selected permissions are = {:?}", self.apis[index_integer].scopes));
                    // self.selected_api_id = Some(self.apis[index_integer].resource_server_id.clone());
                    // self.selected_api_name = Some(self.apis[index_integer].name.clone());
                    self.selected_api = Some(self.apis[index_integer].clone());
                    self.option_permissions = Some(self.apis[index_integer].scopes.clone());
                }
                true
            }
            Msg::SelectPermission(index) => {
                ConsoleService::info(&format!("selected index is {}", index));

                let mut selected_permission = SelectedPermission {
                    permission_name: String::from(""),
                    resource_server_identifier: String::from(""),
                };

                // SECURE WAY TO DEFINE VARIABLE
                // if self.option_permissions.is_some() && self.selected_api.is_some() {
                //     selected_permission = SelectedPermission {
                //         permission_name: self.option_permissions.clone().unwrap()[index].value,
                //         resource_server_identifier: self.selected_api.clone().unwrap().identifier,
                //     };
                // }

                // SECURE WAY TO DEFINE VARIABLE
                if let Some(permissions) = self.option_permissions.clone() {
                    if let Some(api) = self.selected_api.clone() {
                        selected_permission = SelectedPermission {
                            permission_name: permissions[index].value.clone(),
                            resource_server_identifier: api.identifier.clone(),
                        };
                    }
                }

                ConsoleService::info(&format!("selected permission is {:?}", selected_permission));

                // CHECK IF PERMISSION TARGETED IS ALREADY SELECTED
                if self.selected_permissions
                .clone()
                .iter()
                .any(|data| {
                    *data == selected_permission
                })
                {
                    // REMOVE PERMISSION FROM SELECTED PERMISSIONS
                    let new_selected_permissions = self.selected_permissions
                    .clone()
                    .iter()
                    .filter(|data| {
                        **data != selected_permission
                    })
                    .map(|data| {
                        data.clone()
                    })
                    .collect::<Vec<SelectedPermission>>();

                    self.selected_permissions = new_selected_permissions;
                } else {
                    // let new_selected_permissions = self.selected_permissions.clone().push(selected_permission);
                    // self.selected_permissions = new_selected_permissions;
                    self.selected_permissions.push(selected_permission);
                }


                ConsoleService::info(&format!("new selected permissions = {:?}", self.selected_permissions));

                true
            }
            Msg::RequestAssignPermissions => {
                // VALIDATION
                if self.selected_permissions.len() == 0 {
                    self.link.send_message(Msg::ResponseError(String::from("There is no permission to assign"), StateError::RequestAssignPermissions));
                    false
                } else {
                    #[derive(Serialize, Debug, Clone, PartialEq)]
                    struct DataAssignPermissions {
                        permissions: Vec<SelectedPermission>
                    }
                    let data_assign_permissions = DataAssignPermissions {
                        permissions: self.selected_permissions.clone()
                    };
                    let request = Request::post(format!("{}/api/v2/users/{}/permissions", API_URL, self.user_id))
                        .header("access_token", self.access_token.clone())
                        .header("Content-Type", "application/json")
                        .body(Json(&data_assign_permissions))
                        .expect("Could not build request");
                    let callback = self.link.callback(
                        |response: Response<Json<Result<String, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            // ConsoleService::info(&format!("{:?}", data));
                            match data{
                                Ok(dataok) => Msg::GetResponseAssignPermissions(dataok), 
                                Err(error) => {
                                    Msg::ResponseError(error.to_string(), StateError::RequestAssignPermissions)
                                }
                            }
                        }
                    );
    
                    let task = FetchService::fetch(request, callback).expect("failed to start request");
                    self.fetch_task = Some(task);
                    self.error_assign_permissions = None;
                    self.loading_assign_permissions = true;
                    true
                }
            }
            Msg::GetResponseAssignPermissions(message) => {
                self.fetch_task = None;
                self.loading_assign_permissions = false;
                self.message = Some(message);
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::RequestApis => {
                        self.loading_get_apis = false;
                        self.error_get_apis = Some(message);
                    }
                    StateError::RequestAssignPermissions => {
                        self.loading_assign_permissions = false;
                        self.error_assign_permissions = Some(message);
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.user_permissions != props.user_permissions {
            self.user_permissions = props.user_permissions;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="modal fade" id="addPermissions" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                <div
                    class="modal-dialog modal-dialog-centered"
                    style="max-width: 50%; overflow: hidden;"
                >
                    <div class="modal-content pt-4 pe-5 pb-4 ps-5">
                        <div class="modal-header">
                            <h5 class="modal-title" id="exampleModalLabel">{"Add Permissions"}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        {
                            if self.loading_get_apis {
                                html! {
                                    <div
                                        class="modal-body pt-2"
                                        style="position: relative;"
                                    >
                                        <Loading2 width=45 />
                                    </div>
                                }
                            } else if self.error_get_apis.is_some() {
                                html! {
                                    <div
                                        class="modal-body"
                                    >
                                        <div class="alert alert-warning mb-5" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_get_apis.clone().unwrap() }
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="modal-body">
                                        <label for="exampleDataList" class="form-label">{"Select permissions from existing APIs"}</label>
                                        // <input class="form-control" list="listAPIOptions" id="exampleDataList" placeholder="Select an API..."/>
                                        <select
                                            // id="listAPIOptions"
                                            class="form-select mb-2"
                                            aria-label="Select Api"
                                            onchange=self.link.callback(|e| {
                                                if let ChangeData::Select(select) = e {
                                                    let value = select.value();
                                                    // Msg::Input(value, DataUserCreate::Connection)
                                                    Msg::SelectApi(value)
                                                } else {
                                                    Msg::SelectApi(String::from("no index"))
                                                    // Msg::Input(String::from("no value"), DataUserCreate::Connection)
                                                }
                                            })
                                        >
                                            <option>
                                                {"-- Select Api --"}
                                            </option>
                                            { self.view_apis() }
                                        </select>
                                        { self.view_permissions() }
                                    </div>
                                }
                            }
                        }
                        <div class="modal-footer">
                            <button
                                type="button"
                                class=format!("btn {} btn-primary position-relative", if self.loading_assign_permissions {"loading"} else {""} )
                                onclick=self.link.callback(|_| Msg::RequestAssignPermissions)
                                disabled={ self.loading_assign_permissions }
                            >
                                <div class="telkom-label">
                                    {"Add Permissions"}
                                </div>
                                <div class="telkom-spinner telkom-center">
                                    <div class="spinner-border spinner-border-sm" role="status"/>
                                </div>
                            </button>
                        </div>
                        {
                            if self.message.is_some() {
                                html! {
                                    <div class="alert alert-success" role="alert">
                                        { self.message.clone().unwrap() }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        {
                            if self.error_assign_permissions.is_some() {
                                html! {
                                    <div class="alert alert-warning" role="alert">
                                        { self.error_assign_permissions.clone().unwrap() }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        <DevelopersNote
                            message="Feature autoclose modal is not yet implemented"
                        />
                        <div class="modal-footer">
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}


impl ModalAssignPermissions {
    fn view_apis(&self) -> Vec<Html> {
        self.apis
        .clone()
        .iter()
        .enumerate()
        .map(|(index, api)| {
            let ApiTitle {
                resource_server_id,
                name,
                is_system: _,
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
                tenant_id: _,
            } = api.clone();
            html! {
                <option
                    value={index.to_string()}
                >
                    { name }
                </option>
            }
        }).collect()
    } 

    fn view_permissions (&self) -> Html {

        // if self.option_permissions.is_some() {
        //     ConsoleService::info(&format!("option permissions = {:?}", self.option_permissions.clone().unwrap()));
        // }
        // ConsoleService::info(&format!("user permissions = {:?}", self.user_permissions));

        if self.option_permissions.is_some() {
            if self.option_permissions.clone().unwrap().len() > 0 {
                html! {
                    <div
                        class="border border-1 rounded p-3 mt-3"
                        style="max-height: 250px; overflow-y: scroll;"
                    >
                        {
                            
                            self.option_permissions
                            .clone()
                            .unwrap()
                            .iter()
                            .enumerate()
                            .filter(|(index, permission)| {
                                let mut already_assigned = false;
                                if self.selected_api.is_some() {
                                    for user_permission in self.user_permissions.clone() {
                                        // ConsoleService::info(&format!("permission {:?}", permission.value));
                                        if user_permission.permission_name == permission.value && user_permission.resource_server_name == self.selected_api.clone().unwrap().name {
                                            // ConsoleService::info(&format!("permission {:?} is already assigned", permission.value));
                                            already_assigned = true;
                                        }
                                    }
                                }
                                !already_assigned
                            })
                            .map(|(index, permission)| {
                                let checked = self.selected_permissions
                                .clone()
                                .iter()
                                .any(|selected_permission| {
                                    *selected_permission.permission_name == permission.value
                                });
    
                                html! {
                                    <div
                                        class="d-inline-block m-2"
                                        onclick=self.link.callback(move |_| Msg::SelectPermission(index.clone()))
                                    >
                                        <input
                                            type="checkbox"
                                            class="btn-check"
                                        />
                                        <label
                                            class="btn btn-outline-secondary"
                                            for="btn-check-outlined"
                                        >
                                            <div class="form-check form-check-inline">
                                                <input
                                                    class="form-check-input"
                                                    type="checkbox"
                                                    // id="inlineCheckbox1"
                                                    // value="option1"
                                                    checked={checked}
                                                />
                                                <label
                                                    class="form-check-label text-dark"
                                                    for="inlineCheckbox1"
                                                >{ permission.value.clone() }</label>
                                            </div>
                                        </label>
                                    </div>
                                }
                            }).collect::<Vec<Html>>()
                        }
                    </div>
                }
            } else {
                html! {
                    <div>
                        {"There is no permission in this Resource Server"}
                    </div>
                }
            }
        } else {
            html! {}
        }

    }
}