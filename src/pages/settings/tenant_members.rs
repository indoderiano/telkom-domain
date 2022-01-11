use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
    }
};
use types::{
    ResponseMessage,
    settings::{
        TenantMember,
    },
};
use serde::{
    Serialize,
};
use crate::components::{
    loading2::Loading2,
};
use crate::configs::server::API_URL;


#[derive(Serialize, Debug, Clone)]
pub struct MemberCreate {
    email: String,
    role: String,
}

pub enum DataMemberCreate {
    Email,
    Role,
}

pub enum StateError {
    RequestMembers,
    CreateMember,
}

pub struct SettingsTenantMembers {
    members_list: Option<Vec<TenantMember>>,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    loading_request_members: bool,
    error_request_members: Option<String>,
    show_modal_create_member: bool, 
    member_create: MemberCreate,
    loading_create_member: bool,
    error_create_member: Option<String>,
}

pub enum Msg {
    RequestMembers,
    GetMembers(Vec<TenantMember>),
    ShowModalCreate(bool),
    InputString(String, DataMemberCreate),
    CreateMember,
    ResponseError(String, StateError)
}

impl Component for SettingsTenantMembers {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SettingsTenantMembers {
            members_list: None,
            link,
            fetch_task: None,
            loading_request_members: false,
            error_request_members: None,
            show_modal_create_member: false,
            member_create: MemberCreate {
                email: String::from(""),
                role: String::from(""),
            },
            loading_create_member: false,
            error_create_member: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestMembers);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestMembers => {
                // Default State
                self.fetch_task = None;
                self.loading_create_member = false;

                let request = Request::get(format!("{}/tenant/v2/settings/members", API_URL))
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<TenantMember>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", dataok));
                                Msg::GetMembers(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::RequestMembers)
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_request_members = None;
                self.loading_request_members = true;
                true
            }
            Msg::GetMembers(members) => {
                self.members_list = Some(members);
                self.fetch_task = None;
                self.loading_request_members = false;
                true
            }
            Msg::ShowModalCreate(status) => {
                self.show_modal_create_member = status;
                true
            }
            Msg::InputString(value, data) => {
                match data {
                    DataMemberCreate::Email => {
                        self.member_create.email = value;
                    }
                    DataMemberCreate::Role => {
                        if self.member_create.role == String::from("Admin") {
                            self.member_create.role = String::from("");
                        } else {
                            self.member_create.role = String::from("Admin");
                        }
                    }
                }
                true
            }
            Msg::CreateMember => {
                let request = Request::post(format!("{}/tenant/v2/settings/members", API_URL))
                    .header("Content-Type", "application/json")    
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Json(&self.member_create))
                    .expect("Could not build request.");
                let callback = 
                    self.link.batch_callback(|response: Response<Json<Result<ResponseMessage, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", dataok));
                                vec![Msg::ShowModalCreate(false), Msg::RequestMembers]
                            }
                            Err(error) => {
                                vec![Msg::ResponseError(error.to_string(), StateError::CreateMember)]
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_create_member = None;
                self.loading_create_member = true;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::RequestMembers => {
                        self.fetch_task = None;
                        self.loading_request_members = false;
                        self.error_request_members = Some(message);
                    }
                    StateError::CreateMember => {
                        self.fetch_task = None;
                        self.loading_create_member = false;
                        self.error_create_member = Some(message);
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
        if self.loading_request_members {
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
        } else if self.error_request_members.is_some() {
            html! {
                <div class="alert alert-warning mb-5" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    { self.error_request_members.clone().unwrap() }
                </div>
            }
        } else {
            html! {
                { self.view_content() }
            }
        }
    }
}


impl SettingsTenantMembers {
    fn view_content (&self) -> Html {
        html! {
            <div>
                <div
                    class="d-flex mb-4"
                >
                    <div>
                        {"Tenant Members have varying levels of access to the Auth0 dashboard based on their Roles, which are set when adding new members and can be edited any time. Learn more."}
                    </div>
                    <div
                        class="d-flex ms-3"
                    >
                        <select class="form-select d-inline-block me-2 w-auto" aria-label="Default select example">
                            <option selected=true>{"Members"}</option>
                            <option value="1">{"One"}</option>
                            <option value="2">{"Two"}</option>
                            <option value="3">{"Three"}</option>
                        </select>
                        <button
                            type="button"
                            class="btn btn-primary text-nowrap"
                            onclick=self.link.callback(|_| Msg::ShowModalCreate(true))
                            // data-bs-toggle="modal" data-bs-target="#exampleModal"
                        >
                            <span>{"Add Member"}</span>
                        </button>
                    </div>
                </div>

                <table class="table">
                    <thead>
                    <tr>
                        <th scope="col">{"Names"}</th>
                        <th scope="col">{"Roles"}</th>
                        <th scope="col">{"MFA"}</th>
                    </tr>
                    </thead>
                    <tbody>
                        { 
                            
                            if self.error_request_members.is_some() {
                                html! {
                                    <div class="alert alert-warning mb-5" role="alert">
                                        <i class="bi bi-exclamation-triangle me-2"></i>
                                        { self.error_request_members.clone().unwrap() }
                                    </div>
                                }
                            } else if let Some(members) = self.members_list.clone() {
                                html! {
                                    <>
                                        { self.view_members(members) }
                                    </>
                                }
                            } else {
                                html! {
                                    <div
                                        class="text-align-center"
                                    >
                                        {"There are no users"}
                                    </div>
                                }
                            }
                        }
                    </tbody>
                </table>


                // MODAL NEW MEMBER
                <div
                    // class="modal fade"
                    class=format!("modal fade {}", if self.show_modal_create_member {"show"} else {""})
                    style="display: block;"
                    // id="exampleModal"
                    // tabindex="-1"
                    // aria-labelledby="exampleModalLabel"
                    // aria-hidden="true"
                >
                    <div class="modal-dialog modal-dialog-scrollable">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="exampleModalLabel">{"Add New Tenant Member"}</h5>
                                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body" style="font-size: 14px;">
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Email*"}</label>
                                    <div class="input-group mb-2">
                                        <input
                                            type="text"
                                            class="form-control"
                                            id="basic-url"
                                            aria-describedby="basic-addon3"
                                            placeholder="john@mail.com"
                                            oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, DataMemberCreate::Email))
                                            disabled={ if self.loading_create_member {true} else {false} }
                                        />
                                    </div>
                                </div>
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Roles"}</label>
                                    <div class="alert alert-warning mb-5" role="alert">
                                        <i class="bi bi-exclamation-triangle me-2"></i>
                                        {"In order to take advantage of all available roles, please consider upgrading your Auth0 subscription"}
                                    </div>


                                    <div>

                                        <div
                                            class="form-check mb-3"
                                            style=format!("{}", if self.loading_create_member {"pointer-events: none;"} else {""} )
                                            onclick=self.link.callback(|_| Msg::InputString(String::from("none"), DataMemberCreate::Role))
                                        >
                                            <input
                                                class="form-check-input"
                                                type="checkbox"
                                                value=""
                                                id="flexCheckDefault"
                                                checked={ if self.member_create.role == String::from("Admin") {true} else {false} }
                                            />
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Admin"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"Read and write access to all resources in the dashboard."}
                                                </p>
                                            </label>
                                        </div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Editor - Specific Apps"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"Read and write access to specific applications only."}
                                                </p>
                                            </label>
                                        </div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Editor - Connections"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"Read, write, and create access to all types of connections."}
                                                </p>
                                            </label>
                                        </div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Editor - Users"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"User Management operations (create, delete, block, unblock, reset MFA, reset password, update metadata, assign roles, etc.) and access to logs."}
                                                </p>
                                            </label>
                                        </div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Viewer - Users"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"Read-only access to users and logs."}
                                                </p>
                                            </label>
                                        </div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Viewer - Config Settings"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"Read-only access to all configuration settings (applications, APIs, rules, security settings, etc.), except for sensitive information such as secrets, billings, users, and logs."}
                                                </p>
                                            </label>
                                        </div>

                                    </div>

                                </div>
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                                // <button
                                //     type="button"
                                //     class="btn btn-primary"
                                //     onclick=self.link.callback( |_| Msg::CreateMember )
                                // >{"Invite"}</button>
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_create_member {"loading"} else {""} )
                                    onclick=self.link.callback(|_| Msg::CreateMember)
                                    disabled={ if self.loading_create_member {true} else {false} }
                                >
                                    <div class="telkom-label">
                                      {"Invite"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                      <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>
                            </div>
                            {
                                if self.error_create_member.is_some() {
                                    html! {
                                        <div class="modal-footer">
                                            <div class="alert alert-warning" role="alert">
                                                <i class="bi bi-exclamation-triangle me-2"></i>
                                                { self.error_create_member.clone().unwrap() }
                                            </div>
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
                    class=format!("modal-backdrop fade {}", if self.show_modal_create_member {"show"} else {""})
                />
            </div>
        }
    }

    fn view_members (&self, members: Vec<TenantMember>) -> Vec<Html> {
        members.iter().map(|member| {
            html! {
                <tr>
                    <td>
                        <div
                            class="p-2 d-flex"
                        >
                            <div
                                style="flex: 0 0 auto; width: 40px; height: 40px; background-color: rgb(100,100,100);"
                                class="d-flex justify-content-center align-items-center rounded-circle me-3"
                            >
                                <i class="bi bi-info-lg text-light"></i>
                            </div>

                            <div
                                class="d-grid"
                                style="min-width: 40px;"
                            >
                                <span
                                    class="fw-bold"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                        text-decoration: none;
                                    "
                                >
                                    {member.username.clone()}
                                </span>
                                <p
                                    class="mb-0 text-muted"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                    "
                                >
                                    {member.email.clone()}
                                </p>
                            </div>

                        </div>
                    </td>
                    <td>{member.roles.clone()}</td>
                    <td>
                        {
                            if member.is_mfa {
                                html! {
                                    <span
                                        class="badge fw-bolder"
                                        style="text-transform: uppercase; letter-spacing: 1px; font-size: 10px;"
                                    >{"able"}</span>
                                }
                            } else {
                                html! {
                                    <span
                                        class="badge bg-danger fw-bolder"
                                        style="text-transform: uppercase; letter-spacing: 1px; font-size: 10px;"
                                    >{"disabled"}</span>
                                }
                            }
                        }
                    </td>
                </tr>
            }
        })
        .collect()
    }
}