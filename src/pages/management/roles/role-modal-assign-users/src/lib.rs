use yew::{
    prelude::*,
    format::{Json, Nothing},
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
        storage::{ StorageService, Area }
    },
};
use configs::server::API_URL;
use types::{
    roles::{ Role, RoleUser },
    users::{ UserTitle },
    ResponseMessage,
    LocalStorage,
    LOCALSTORAGE_KEY,
};
use serde::Serialize;
use loading::Loading;
use developers_note::DevelopersNote;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ModalAssignUsersProps {
    pub role_users: Vec<RoleUser>,
    pub role: Role,
}

pub struct ModalAssignUsers {
    // SERVICES
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,

    // DATA
    access_token: String,
    role_users: Vec<RoleUser>,
    role: Role,
    users: Vec<UserTitle>,
    selected_users: Vec<UserTitle>,

    // LAYOUT STATE
    loading_get_users: bool,
    error_get_users: Option<String>,
    loading_assign_users: bool,
    error_assign_users: Option<String>,
    message: Option<String>,
    is_select_users_open: bool,
}

pub enum StateError {
    RequestUsers,
    RequestAssignUsers,
}
pub enum Msg {
    RequestUsers,
    GetUsers(Vec<UserTitle>),

    ClickSelectUsers,
    SelectUser(String),
    UnselectUser(String),
    RequestAssignUsers,
    GetResponseAssignUsers,
    ResponseError(String, StateError),
}

impl Component for ModalAssignUsers {
    type Message = Msg;
    type Properties = ModalAssignUsersProps;

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

        ConsoleService::info(&format!("user roles = {:?}", props.role_users));

        ModalAssignUsers {
            link,
            fetch_task: None,
            access_token,
            role_users: props.role_users,
            role: props.role,
            loading_get_users: false,
            error_get_users: None,
            users: vec![],
            // selected_api_id: None,
            // selected_api_name: None,
            is_select_users_open: false,
            selected_users: Vec::new(),
            error_assign_users: None,
            loading_assign_users: false,
            message: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestUsers);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestUsers => {
                let request = Request::get(format!("{}/api/v2/users", API_URL))
                    .header("access_token", self.access_token.clone())
                    .body(Nothing)
                    .expect("Could not build request");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<UserTitle>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        // ConsoleService::info(&format!("{:?}", data));
                        match data{
                            Ok(dataok) => Msg::GetUsers(dataok), 
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::RequestUsers)
                            }
                        }
                    }
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_get_users = None;
                self.loading_get_users = true;
                true
            }
            Msg::GetUsers(data) => {
                self.users = data;
                self.loading_get_users = false;
                self.fetch_task = None;
                true
            }
            Msg::ClickSelectUsers => {
                self.is_select_users_open = !self.is_select_users_open;
                true
            }
            Msg::SelectUser(index) => {
                self.is_select_users_open = false;
                ConsoleService::info(&format!("role index = {}", index));
                let index_integer = index.parse::<usize>().unwrap();
                // VALIDATION
                // IF ROLE ALREADY SELECTED, THEN REMOVE ROLE FROM SELECTED ROLES
                if self.selected_users
                .clone()
                .iter()
                .any(|data| {
                    *data.user_id == self.users[index_integer].user_id
                })
                {
                    // REMOVE ROLES
                    let new_selected_users = self.selected_users
                    .clone()
                    .iter()
                    .filter(|data| {
                        *data.user_id != self.users[index_integer].user_id
                    })
                    .map(|data| {
                        data.clone()
                    })
                    .collect();

                    self.selected_users = new_selected_users;
                } else {
                    self.selected_users.push(self.users[index_integer].clone());
                }

                // ConsoleService::info(&format!("new selected roles id = {:?}", self.selected_users));
                
                true
            }
            Msg::UnselectUser(id) => {
                self.is_select_users_open = true;
                let new_selected_users = self.selected_users
                .clone()
                .iter()
                .filter(|data| {
                    *data.user_id != id
                })
                .map(|data| {
                    data.clone()
                })
                .collect();

                self.selected_users = new_selected_users;
                true
            }
            Msg::RequestAssignUsers => {
                // VALIDATION
                if self.selected_users.len() == 0 {
                    self.link.send_message(Msg::ResponseError(String::from("There is no roles to assign"), StateError::RequestAssignUsers));
                    false
                } else {
                    #[derive(Serialize, Debug, Clone, PartialEq)]
                    struct DataAssignUsers {
                        users: Vec<String>
                    }
                    let data_assign_users = DataAssignUsers {
                        users: self.selected_users
                        .clone()
                        .iter()
                        .map(|data| {
                            data.user_id.clone()
                        })
                        .collect()
                    };
                    let request = Request::post(format!("{}/api/v2/roles/{}/users", API_URL, self.role.id))
                        .header("access_token", self.access_token.clone())
                        .header("Content-Type", "application/json")
                        .body(Json(&data_assign_users))
                        .expect("Could not build request");
                    let callback = self.link.callback(
                        |response: Response<Json<Result<(), anyhow::Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            let status_number = meta.status.as_u16();

                            match status_number {
                                200 => {
                                    Msg::GetResponseAssignUsers
                                }
                                _ => {
                                    match data{
                                        Ok(dataok) => Msg::GetResponseAssignUsers, 
                                        Err(error) => {
                                            Msg::ResponseError(error.to_string(), StateError::RequestAssignUsers)
                                        }
                                    }
                                }
                            }
                            // ConsoleService::info(&format!("{:?}", data));
                        }
                    );
    
                    let task = FetchService::fetch(request, callback).expect("failed to start request");
                    self.fetch_task = Some(task);
                    self.error_assign_users = None;
                    self.loading_assign_users = true;
                    true
                }
            }
            Msg::GetResponseAssignUsers => {
                // SERVICE
                self.fetch_task = None;

                // DATA
                self.selected_users = Vec::new();

                // LAYOUT
                self.loading_assign_users = false;
                self.message = Some(String::from("Users are assigned to role"));
                self.is_select_users_open = false;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::RequestUsers => {
                        self.loading_get_users = false;
                        self.error_get_users = Some(message);
                    }
                    StateError::RequestAssignUsers => {
                        self.loading_assign_users = false;
                        self.error_assign_users = Some(message);
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.role_users != props.role_users {
            self.role_users = props.role_users;
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
                            <h5 class="modal-title" id="exampleModalLabel">
                                { format!("Assign {} role to users", self.role.name.clone()) }
                            </h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        {
                            if self.loading_get_users {
                                html! {
                                    <div
                                        class="modal-body pt-2"
                                        style="position: relative;"
                                    >
                                        <Loading width=45 />
                                    </div>
                                }
                            } else if self.error_get_users.is_some() {
                                html! {
                                    <div
                                        class="modal-body"
                                    >
                                        <div class="alert alert-warning mb-5" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_get_users.clone().unwrap() }
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="modal-body">
                                        <label for="exampleDataList" class="form-label mb-2">{"Select users you want to assign this role to."}</label>
                                        // <input class="form-control" list="listAPIOptions" id="exampleDataList" placeholder="Select an API..."/>
                                        
                                        <div class="fw-bold">
                                            {"Select users"}
                                        </div>
                                        <div class="dropdown">
                                            <button
                                                class="form-select text-start"
                                                style="padding: 0.125rem 2.25rem 0.125rem 0.25rem; min-height: 40px;"
                                                type="button"
                                                // id="dropdownRoles"
                                                // data-bs-toggle="dropdown"
                                                aria-expanded="false"
                                                onclick=self.link.callback(|_| Msg::ClickSelectUsers)
                                            >
                                                {
                                                    if self.selected_users.len() == 0 {
                                                        html! {}
                                                    } else {
                                                        html! {
                                                            <>
                                                                { self.view_selected_users() }
                                                            </>
                                                        }
                                                    }
                                                }
                                                <span class="ms-1">
                                                    { "Select a user..." }
                                                </span>
                                            </button>
                                            <ul
                                                class=format!("dropdown-menu w-100 {}", if self.is_select_users_open {"show"} else {""})
                                                // aria-labelledby="dropdownRoles"
                                            >
                                                { self.view_option_users() }
                                            </ul>
                                        </div>

                                    </div>
                                }
                            }
                        }
                        <div class="modal-footer">
                            <button
                                type="button"
                                class="btn btn-secondary"
                                data-bs-dismiss="modal"
                                aria-label="Close"
                            >
                                {"Cancel"}
                            </button>

                            <button
                                type="button"
                                class=format!("btn {} btn-primary position-relative", if self.loading_assign_users {"loading"} else {""} )
                                onclick=self.link.callback(|_| Msg::RequestAssignUsers)
                                disabled={ self.loading_assign_users }
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
                            if self.error_assign_users.is_some() {
                                html! {
                                    <div class="alert alert-warning" role="alert">
                                        { self.error_assign_users.clone().unwrap() }
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


impl ModalAssignUsers {
    fn view_option_users(&self) -> Vec<Html> {
        let option_users = self.users
        .clone()
        .iter()
        .filter(|user| {
            ConsoleService::info(&format!("user roles = {:?}", self.role_users));
            ConsoleService::info(&format!("selected roles = {:?}", self.selected_users));
            // ONLY RETURN ROLE THAT IS NOT IN SELECTED OR IN USER ROLES
            if self.role_users
            .clone()
            .iter()
            .any(|role_user| {
                *role_user.user_id == user.user_id
            }) {
                false
            } else if self.selected_users
            .clone()
            .iter()
            .any(|selected_user| {
                *selected_user.user_id == user.user_id
            }) {
                false
            } else {
                true
            }
        })
        .map(|role| {
            role.clone()
        }).collect::<Vec<UserTitle>>();

        if option_users.len() == 0 {
            vec![
                html!{
                    <li class="ps-2">
                        {"There is no role available"}
                    </li>
                }
            ]
        } else {
            self.users
            .clone()
            .iter()
            .enumerate()
            .filter(|(index, user)| {
                // ONLY RETURN ROLE THAT IS NOT IN SELECTED OR IN USER ROLES
                if self.role_users
                .clone()
                .iter()
                .any(|role_user| {
                    *role_user.user_id == user.user_id
                }) {
                    false
                } else if self.selected_users
                .clone()
                .iter()
                .any(|selected_user| {
                    *selected_user.user_id == user.user_id
                }) {
                    false
                } else {
                    true
                }
            })
            .map(|(index, user)| {
                // let Role {
                //     id,
                //     name,
                //     description: _,
                // } = role.clone();
                html! {
                    <li
                        onclick=self.link.callback(move |_| Msg::SelectUser(index.to_string()))
                    >
                        <a class="dropdown-item" href="#">{ user.email.clone() }</a>
                    </li>
                }
            }).collect()
        }

    }
    fn view_selected_users(&self) -> Vec<Html> {
        self.selected_users
        .clone()
        .iter()
        .map(|user| {
            // let Role {
            //     id,
            //     name,
            //     description: _
            // } = role.clone();
            let acc = user.clone();
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
                        >{ user.email.clone() }</label>
                        <button
                            type="button"
                            class="btn-close ms-1"
                            style="font-size: 9px;"
                            onclick=self.link.callback(move |_| Msg::UnselectUser(acc.user_id.clone()))
                            // data-bs-dismiss="modal"
                            // aria-label="Close"
                        />
                    </label>
                </div>
            }
        }).collect()
    }
}