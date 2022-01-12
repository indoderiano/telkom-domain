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
    roles::Role,
    users::{ UserRole },
    ResponseMessage,
    LocalStorage,
    LOCALSTORAGE_KEY,
};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ModalAssignRolesProps {
    pub user_roles: Vec<UserRole>,
    pub user_id: String,
}

pub struct ModalAssignRoles {
    // SERVICES
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,

    // DATA
    access_token: String,
    user_roles: Vec<UserRole>,
    user_id: String,
    roles: Vec<Role>,
    selected_roles: Vec<Role>,

    // LAYOUT STATE
    loading_get_roles: bool,
    error_get_roles: Option<String>,
    loading_assign_roles: bool,
    error_assign_roles: Option<String>,
    message: Option<String>,
    is_select_roles_open: bool,
}

pub enum StateError {
    RequestRoles,
    RequestAssignRoles,
}
pub enum Msg {
    RequestRoles,
    GetRoles(Vec<Role>),

    ClickSelectRoles,
    SelectRole(String),
    UnselectRole(String),
    RequestAssignRoles,
    GetResponseAssignRoles,
    ResponseError(String, StateError),
}

impl Component for ModalAssignRoles {
    type Message = Msg;
    type Properties = ModalAssignRolesProps;

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

        ConsoleService::info(&format!("user roles = {:?}", props.user_roles));

        ModalAssignRoles {
            link,
            fetch_task: None,
            access_token,
            user_roles: props.user_roles,
            user_id: props.user_id,
            loading_get_roles: false,
            error_get_roles: None,
            roles: Vec::new(),
            // selected_api_id: None,
            // selected_api_name: None,
            is_select_roles_open: false,
            selected_roles: Vec::new(),
            error_assign_roles: None,
            loading_assign_roles: false,
            message: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestRoles);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestRoles => {
                let request = Request::get(format!("{}/api/v2/roles", API_URL))
                    .header("access_token", self.access_token.clone())
                    .body(Nothing)
                    .expect("Could not build request");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<Role>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        // ConsoleService::info(&format!("{:?}", data));
                        match data{
                            Ok(dataok) => Msg::GetRoles(dataok), 
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::RequestRoles)
                            }
                        }
                    }
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_get_roles = None;
                self.loading_get_roles = true;
                true
            }
            Msg::GetRoles(apis) => {
                self.roles = apis;
                self.loading_get_roles = false;
                self.fetch_task = None;
                true
            }
            Msg::ClickSelectRoles => {
                self.is_select_roles_open = !self.is_select_roles_open;
                true
            }
            Msg::SelectRole(index) => {
                self.is_select_roles_open = false;
                ConsoleService::info(&format!("role index = {}", index));
                let index_integer = index.parse::<usize>().unwrap();
                // VALIDATION
                // IF ROLE ALREADY SELECTED, THEN REMOVE ROLE FROM SELECTED ROLES
                if self.selected_roles
                .clone()
                .iter()
                .any(|data| {
                    *data.id == self.roles[index_integer].id
                })
                {
                    // REMOVE ROLES
                    let new_selected_roles = self.selected_roles
                    .clone()
                    .iter()
                    .filter(|data| {
                        *data.id != self.roles[index_integer].id
                    })
                    .map(|data| {
                        data.clone()
                    })
                    .collect();

                    self.selected_roles = new_selected_roles;
                } else {
                    self.selected_roles.push(self.roles[index_integer].clone());
                }

                // ConsoleService::info(&format!("new selected roles id = {:?}", self.selected_roles));
                
                true
            }
            Msg::UnselectRole(id) => {
                self.is_select_roles_open = true;
                let new_selected_roles = self.selected_roles
                .clone()
                .iter()
                .filter(|data| {
                    *data.id != id
                })
                .map(|data| {
                    data.clone()
                })
                .collect();

                self.selected_roles = new_selected_roles;
                true
            }
            Msg::RequestAssignRoles => {
                // VALIDATION
                if self.selected_roles.len() == 0 {
                    self.link.send_message(Msg::ResponseError(String::from("There is no roles to assign"), StateError::RequestAssignRoles));
                    false
                } else {
                    #[derive(Serialize, Debug, Clone, PartialEq)]
                    struct DataAssignRoles {
                        roles: Vec<String>
                    }
                    let data_assign_roles = DataAssignRoles {
                        roles: self.selected_roles
                        .clone()
                        .iter()
                        .map(|data| {
                            data.id.clone()
                        })
                        .collect()
                    };
                    let request = Request::post(format!("{}/api/v2/users/{}/roles", API_URL, self.user_id))
                        .header("access_token", self.access_token.clone())
                        .header("Content-Type", "application/json")
                        .body(Json(&data_assign_roles))
                        .expect("Could not build request");
                    let callback = self.link.callback(
                        |response: Response<Json<Result<(), anyhow::Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            let status_number = meta.status.as_u16();

                            match status_number {
                                204 => {
                                    Msg::GetResponseAssignRoles
                                }
                                _ => {
                                    match data{
                                        Ok(dataok) => Msg::GetResponseAssignRoles, 
                                        Err(error) => {
                                            Msg::ResponseError(error.to_string(), StateError::RequestAssignRoles)
                                        }
                                    }
                                }
                            }
                            // ConsoleService::info(&format!("{:?}", data));
                        }
                    );
    
                    let task = FetchService::fetch(request, callback).expect("failed to start request");
                    self.fetch_task = Some(task);
                    self.error_assign_roles = None;
                    self.loading_assign_roles = true;
                    true
                }
            }
            Msg::GetResponseAssignRoles => {
                // SERVICE
                self.fetch_task = None;

                // DATA
                self.selected_roles = Vec::new();

                // LAYOUT
                self.loading_assign_roles = false;
                self.message = Some(String::from("Roles are assigned to user"));
                self.is_select_roles_open = false;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::RequestRoles => {
                        self.loading_get_roles = false;
                        self.error_get_roles = Some(message);
                    }
                    StateError::RequestAssignRoles => {
                        self.loading_assign_roles = false;
                        self.error_assign_roles = Some(message);
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.user_roles != props.user_roles {
            self.user_roles = props.user_roles;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="modal fade" id="assignRoles" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                <div
                    class="modal-dialog modal-dialog-centered"
                    style="max-width: 50%; overflow: hidden;"
                >
                    <div class="modal-content pt-4 pe-5 pb-4 ps-5">
                        <div class="modal-header">
                            <h5 class="modal-title" id="exampleModalLabel">{"Add Roles"}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        {
                            if self.loading_get_roles {
                                html! {
                                    <div
                                        class="modal-body pt-2"
                                        style="position: relative;"
                                    >
                                        <Loading2 width=45 />
                                    </div>
                                }
                            } else if self.error_get_roles.is_some() {
                                html! {
                                    <div
                                        class="modal-body"
                                    >
                                        <div class="alert alert-warning mb-5" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_get_roles.clone().unwrap() }
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="modal-body">
                                        <label for="exampleDataList" class="form-label">{"Select roles to assign to this user"}</label>
                                        // <input class="form-control" list="listAPIOptions" id="exampleDataList" placeholder="Select an API..."/>
                                        

                                        <div class="dropdown">
                                            <button
                                                class="form-select text-start"
                                                style="padding: 0.125rem 2.25rem 0.125rem 0.25rem; min-height: 40px;"
                                                type="button"
                                                // id="dropdownRoles"
                                                // data-bs-toggle="dropdown"
                                                aria-expanded="false"
                                                onclick=self.link.callback(|_| Msg::ClickSelectRoles)
                                            >
                                                {
                                                    if self.selected_roles.len() == 0 {
                                                        html! {}
                                                    } else {
                                                        html! {
                                                            <>
                                                                { self.view_selected_roles() }
                                                            </>
                                                        }
                                                    }
                                                }
                                                <span class="ms-1">
                                                    { "Select Role" }
                                                </span>
                                            </button>
                                            <ul
                                                class=format!("dropdown-menu w-100 {}", if self.is_select_roles_open {"show"} else {""})
                                                // aria-labelledby="dropdownRoles"
                                            >
                                                { self.view_option_roles() }
                                            </ul>
                                        </div>

                                        
                                        // <select
                                        //     // id="listAPIOptions"
                                        //     class="form-select mb-2"
                                        //     aria-label="Select Api"
                                        //     onchange=self.link.callback(|e| {
                                        //         if let ChangeData::Select(select) = e {
                                        //             let value = select.value();
                                        //             // Msg::Input(value, DataUserCreate::Connection)
                                        //             Msg::SelectRole(value)
                                        //         } else {
                                        //             Msg::SelectRole(String::from("no index"))
                                        //             // Msg::Input(String::from("no value"), DataUserCreate::Connection)
                                        //         }
                                        //     })
                                        // >
                                        //     <option>
                                        //         {"-- Select Role --"}
                                        //     </option>
                                        //     { self.view_option_roles() }
                                        // </select>
                                    </div>
                                }
                            }
                        }
                        <div class="modal-footer">
                            <button
                                type="button"
                                class=format!("btn {} btn-primary position-relative", if self.loading_assign_roles {"loading"} else {""} )
                                onclick=self.link.callback(|_| Msg::RequestAssignRoles)
                                disabled={ self.loading_assign_roles }
                            >
                                <div class="telkom-label">
                                    {"Assign"}
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
                            if self.error_assign_roles.is_some() {
                                html! {
                                    <div class="alert alert-warning" role="alert">
                                        { self.error_assign_roles.clone().unwrap() }
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


impl ModalAssignRoles {
    fn view_option_roles(&self) -> Vec<Html> {
        let option_roles = self.roles
        .clone()
        .iter()
        .filter(|role| {
            ConsoleService::info(&format!("user roles = {:?}", self.user_roles));
            ConsoleService::info(&format!("selected roles = {:?}", self.selected_roles));
            // ONLY RETURN ROLE THAT IS NOT IN SELECTED OR IN USER ROLES
            if self.user_roles
            .clone()
            .iter()
            .any(|user_role| {
                *user_role.id == role.id
            }) {
                false
            } else if self.selected_roles
            .clone()
            .iter()
            .any(|selected_role| {
                *selected_role.id == role.id
            }) {
                false
            } else {
                true
            }
        })
        .map(|role| {
            role.clone()
        }).collect::<Vec<Role>>();

        if option_roles.len() == 0 {
            vec![
                html!{
                    <li class="ps-2">
                        {"There is no role available"}
                    </li>
                }
            ]
        } else {
            self.roles
            .clone()
            .iter()
            .enumerate()
            .filter(|(index, role)| {
                // ONLY RETURN ROLE THAT IS NOT IN SELECTED OR IN USER ROLES
                if self.user_roles
                .clone()
                .iter()
                .any(|user_role| {
                    *user_role.id == role.id
                }) {
                    false
                } else if self.selected_roles
                .clone()
                .iter()
                .any(|selected_role| {
                    *selected_role.id == role.id
                }) {
                    false
                } else {
                    true
                }
            })
            .map(|(index, role)| {
                let Role {
                    id,
                    name,
                    description: _,
                } = role.clone();
                html! {
                    <li
                        onclick=self.link.callback(move |_| Msg::SelectRole(index.to_string()))
                    >
                        <a class="dropdown-item" href="#">{ name }</a>
                    </li>
                }
            }).collect()
        }

    }
    fn view_selected_roles(&self) -> Vec<Html> {
        self.selected_roles
        .clone()
        .iter()
        .map(|role| {
            let Role {
                id,
                name,
                description: _
            } = role.clone();
            html! {
                <div
                    class="d-inline-block me-1"
                >
                    <label
                        class="btn btn-outline-secondary align-middle"
                        style="font-size: 12px;"
                        for="btn-check-outlined"
                    >
                        <label
                            class="form-check-label text-dark"
                            for="inlineCheckbox1"
                        >{ name.clone() }</label>
                        <button
                            type="button"
                            class="btn-close ms-1"
                            style="font-size: 9px;"
                            onclick=self.link.callback(move |_| Msg::UnselectRole(id.clone()))
                            // data-bs-dismiss="modal"
                            // aria-label="Close"
                        />
                    </label>
                </div>
            }
        }).collect()
    }
}