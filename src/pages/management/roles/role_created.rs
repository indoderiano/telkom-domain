use crate::app::AppRoute;
use crate::types::roles::Role;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        storage::{Area, StorageService},
        ConsoleService,
    },
};

use serde::Serialize;

use crate::types::LocalStorage;
use crate::types::LOCALSTORAGE_KEY;

use crate::components::{
    loading2::Loading2,
    developers_note::DevelopersNote,
};
use crate::configs::server::API_URL;
use yew_router::components::RouterAnchor;

pub enum StateError {
    RequestRoles,
    CreateRole,
}

pub enum DataRole {
    Name,
    Description,
}

pub struct RolesCreated {
    access_token: String,
    roles: Vec<Role>,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    loading_request_roles: bool,
    error_request_roles: Option<String>,
    new_role: Role,
    loading_create_role: bool,
    error_create_role: Option<String>,
}

pub enum Msg {
    RequestRoles,
    GetRoles(Vec<Role>),
    CreateRole,
    InputRole(String, DataRole),
    ResponseError(String, StateError),
}

impl Component for RolesCreated {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
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

        // if local storage exists then update state
        let mut access_token = String::from("");

        if let Some(_) = localstorage_data.token {
            access_token = localstorage_data.token.unwrap()
        } else {
        }

        RolesCreated {
            access_token,
            roles: vec![],
            link,
            fetch_task: None,
            loading_request_roles: false,
            error_request_roles: None,
            new_role: Role::new(),
            loading_create_role: false,
            error_create_role: None,
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
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<Role>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", dataok));
                                Msg::GetRoles(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::RequestRoles)
                            }
                        }
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_request_roles = None;
                self.loading_request_roles = true;
                true
            }
            Msg::GetRoles(data) => {
                self.roles = data;
                self.fetch_task = None;
                self.loading_request_roles = false;
                self.error_request_roles = None;
                self.loading_create_role = false;
                self.error_create_role = None;
                true
            }
            Msg::InputRole(value, data) => {
                match data {
                    DataRole::Name => {
                        self.new_role.name = value;
                    }
                    DataRole::Description => {
                        self.new_role.description = value;
                    }
                }
                false
            }
            Msg::CreateRole => {
                let request = Request::post(format!("{}/api/v2/roles", API_URL))
                    .header("Content-Type", "application/json")
                    .header("access_token", self.access_token.clone())
                    .body(Json(&self.new_role))
                    .expect("Could not build request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Role, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("{:?}", dataok));
                                    Msg::RequestRoles
                                }
                                Err(error) => {
                                    Msg::ResponseError(error.to_string(), StateError::CreateRole)
                                }
                            }
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_create_role = None;
                self.loading_create_role = true;
                true
            }
            Msg::ResponseError(message, state) => match state {
                StateError::RequestRoles => {
                    self.fetch_task = None;
                    self.loading_request_roles = false;
                    self.error_request_roles = Some(message);
                    true
                }
                StateError::CreateRole => {
                    self.fetch_task = None;
                    self.loading_create_role = false;
                    self.error_create_role = Some(message);
                    true
                }
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

                <div class="mx-auto pt-5 pb-5 px-4" style="max-width: 1048px;">

                    <div class="row">

                        <div class="col-9">
                            <h1 class="fw-bold">{"Roles"}</h1>
                        </div>

                        <div class="col-3 d-flex justify-content-end">
                            <button type="button" data-bs-toggle="modal" data-bs-target="#addRoleModal" class="btn btn-primary text-center">
                                <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                                <span>{"Create Role"}</span>
                            </button>
                        </div>
                    </div>

                    <div class="mt-3">
                        <p class="text-muted fs-6">{"Create and manage Roles for your applications. Roles contain collections of Permissions and can be assigned to Users."} </p>
                    </div>


                    {
                        if self.loading_request_roles {
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
                        } else if self.error_request_roles.is_some() {
                            html! {
                                <div class="alert alert-warning mb-5" role="alert">
                                    <i class="bi bi-exclamation-triangle me-2"></i>
                                    { self.error_request_roles.clone().unwrap() }
                                </div>
                            }
                        } else {
                            html! { self.view_content() }
                        }
                    }

            </div>


            <div class="modal fade" id="addRoleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                <div class="modal-dialog modal-dialog-scrollable" role="document">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title" id="exampleModalLabel">{"New Role"}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close">
                            </button>
                        </div>
                        <div class="modal-body">
                            <div class="form-group">
                                <label for="exampleInputEmail1">{"Name "} <span style="color:red">{"*"}</span></label>
                                <input
                                    type="text"
                                    class="form-control"
                                    id="exampleInputEmail1"
                                    oninput=self.link.callback(|data: InputData| Msg::InputRole(data.value, DataRole::Name))
                                />
                            </div>
                            <div class="form-group mt-3 mb-3">
                                <label for="exampleInputPassword1">{"Description "}<span style="color:red">{"*"}</span></label>
                                <input
                                    type="text"
                                    class="form-control"
                                    id="exampleInputPassword1"
                                    oninput=self.link.callback(|data: InputData| Msg::InputRole(data.value, DataRole::Description))
                                />
                            </div>


                        </div>
                        <div class="modal-footer">
                            <button
                                type="button"
                                class="btn btn-secondary"
                                data-bs-dismiss="modal"
                                disabled={ self.loading_create_role }
                            >
                                {"Cancel"}
                            </button>

                            <button
                                type="button"
                                class=format!("btn {} btn-primary position-relative", if self.loading_create_role {"loading"} else {""} )
                                onclick=self.link.callback(|_| Msg::CreateRole)
                                disabled={ self.loading_create_role }
                            >
                                <div class="telkom-label">
                                    {"Create"}
                                </div>
                                <div class="telkom-spinner telkom-center">
                                    <div class="spinner-border spinner-border-sm" role="status"/>
                                </div>
                            </button>

                        </div>

                        {
                            if self.error_create_role.is_some() {
                                html! {
                                    <div class="alert alert-warning" role="alert">
                                        <i class="bi bi-exclamation-triangle me-2"></i>
                                        { self.error_create_role.clone().unwrap() }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }


                    </div>
                </div>
            </div>


            </>
        }
    }
}

impl RolesCreated {
    fn view_content(&self) -> Html {
        html! {
            <div class="mt-5">
                <DevelopersNote message="There is still bugs, work in progress"/>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">{"Name"}</th>
                            <th scope="col">{"Description"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { self.view_list() }
                    </tbody>
                </table>
            </div>
        }
    }

    fn view_list(&self) -> Vec<Html> {
        type Anchor = RouterAnchor<AppRoute>;
        self.roles.clone().iter().map(|role| {
            let Role {
                id,
                name,
                description,
            } = role.clone();
            html! {
                <tr>
                    <td>
                        <Anchor
                            route=AppRoute::RoleSettings { role_id: role.id.clone() }
                            classes="dropdown-item fs-7 text-color-secondary"
                        >
                            { name.clone() }
                        </Anchor>
                    </td>
                    <td>
                        <div class="row">
                            <div class="col-10">
                                <p>{description.clone()}</p>
                            </div>
                            <div class="col-2">
                                <button
                                    type="button"
                                    style="flex: 0 0 auto; width: 30px; height: 30px;"
                                    class="btn d-flex justify-content-center align-items-center rounded border"
                                    role="button"
                                    id="dropdownMenuButton1"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    <i class="bi bi-three-dots"></i>
                                </button>
                                <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                                    <li>
                                        <Anchor route=AppRoute::RoleSettings { role_id: role.id.clone() } classes="dropdown-item fs-7">
                                            {"View Details"}
                                        </Anchor>
                                    </li>
                                    // <li>
                                    //     <Anchor route=AppRoute::ApisSettings{ api_id: 0} classes="dropdown-item fs-7">
                                    //         {"Assign To Users"}
                                    //     </Anchor>
                                    // </li>
                                    // <li>
                                    //     <Anchor route=AppRoute::ApisSettings{ api_id: 0} classes="dropdown-item fs-7">
                                    //         {"Delete Role"}
                                    //     </Anchor>
                                    // </li>
                                </ul>
                            </div>
                        </div>
                    </td>
                </tr>
            }
        })
        .collect()
    }
}
