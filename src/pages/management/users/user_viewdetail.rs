use super::setting_user_details::UserTabDetails;
use super::setting_user_devices::UserTabDevices;
use super::setting_user_history::UserTabHistory;
use super::setting_user_json::UserTabRawJson;
use super::setting_user_auth_app::UserTabAuthorizedApp;
use super::setting_user_permissions::UserTabPermissions;
use super::setting_user_roles::UserTabRoles;
use crate::app::AppRoute;
use crate::components::{
    loading2::Loading2,
    developers_note::DevelopersNote,
};
use crate::configs::server::API_URL;
use crate::types::users::{UserDetails};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
        storage::{ StorageService, Area }
    },
};
use yew_router::components::RouterAnchor;
use crate::types::LocalStorage;
use crate::types::LOCALSTORAGE_KEY;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct UserSettingsProps {
    pub tenant_id: String,
    pub user_id: String,
    pub id : u32,
}

pub enum Content {
    UserTabDetails,
    UserTabDevices,
    UserTabHistory,
    UserTabRawJson,
    UserTabAuthorizedApp,
    UserTabPermissions,
    UserTabRoles,
}

pub struct UserViewDetail {
    // id: u32,
    user_id: String,
    access_token: String,
    content: Content,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    error: Option<String>,
    user_details: UserDetails,
}

pub enum Msg {
    ChangeContent(Content),
    RequestUserDetails,
    GetUserDetails(Result<UserDetails, anyhow::Error>),
}

impl Component for UserViewDetail {
    type Message = Msg;
    type Properties = UserSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // ConsoleService::info(&format!(
        //     "User setting props, tenant id = {}",
        //     props.tenant_id
        // ));
        // ConsoleService::info(&format!("User setting props, user id = {}", props.user_id));


        // GET LOCALSTORAGE
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


        let user_details = UserDetails::new();

        UserViewDetail {
            // id: props.id,
            user_id: props.user_id,
            access_token,
            content: Content::UserTabDetails,
            link,
            fetch_task: None,
            error: None,
            user_details,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestUserDetails)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeContent(content) => {
                self.content = content;
                true
            }
            Msg::RequestUserDetails => {
                
                let request = Request::get(format!("{}/api/v2/users/{}", API_URL, self.user_id.clone()))
                    .header("access_token", self.access_token.clone())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<UserDetails, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::GetUserDetails(data)
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::GetUserDetails(response) => {
                ConsoleService::info(&format!(" ========>>>>> {:?}", response));
                match response {
                    Ok(data) => {
                        ConsoleService::info(&format!(" ini di get user details ====>  {:?}", data));
                        self.user_details = data;
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
        let tenant_id = String::from("tenant_id_not_from_reducer");
        html! {
            <>
                <div
                    class="container mx-auto py-5 px-4"
                    style="max-width: 1048px"
                >
                    <Anchor
                        route=AppRoute::UsersManagement {tenant_id: tenant_id}
                        classes="text-decoration-none domain-link-dark"
                    >
                        <i class="bi bi-arrow-left me-2"></i>
                        {"Back to Users"}
                    </Anchor>
                    {
                        if self.fetch_task.is_some() {
                            html! {
                                <div style="position: relative; margin-top:8rem;">
                                    <Loading2 width = 45 />
                                </div>
                            }
                        } else {
                            html! {
                                {self.view_content()}
                            }
                        }
                    }
                </div>
            </>
        }
    }
}

impl UserViewDetail {
    fn view_content(&self) -> Html {
        let UserDetails {
            // id,
            user_id,
            email,
            email_verified: _,
            username: _,
            phone_number: _,
            phone_verified: _,
            created_at: _,
            updated_at: _,
            identities: _,
            app_metadata: _,
            user_metadata: _,
            picture: _,
            name,
            nickname: _,
            multifactor: _,
            last_ip: _,
            last_login: _,
            logins_count: _,
            blocked: _,
            given_name: _,
            family_name: _,
        } = self.user_details.clone();

        html! {
            <>
                <div class="mt-2 mb-5">
                    <div class="row">
                        <div class="col">




                            <div
                                class="d-flex mt-3"
                            >
                                <div
                                    style="flex: 0 0 auto; width: 64px; height: 64px; background-color: #eff0f2;"
                                    class="d-flex justify-content-center align-items-center rounded me-4"
                                >
                                    <i class="bi bi-person-fill fs-1"></i>
                                </div>
            
                                <div
                                    class="d-flex flex-column"
                                >
                                    <h2>{ email }</h2>
                                    <div
                                        class="text-muted"
                                    >
                                        <span>
                                            { "user_id" }
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
                                            { user_id }
                                        </span>
                                    </div>
                                </div>
                            </div>






                            // <p class="mb-0" style="font-size: 32px; font-weight: bold">
                            //     { email }
                            // </p>
                            // <p class="text-muted mb-0">
                            //     { "user_id : " }
                            //     <span> <code style="background-color: beige; color: black">{user_id}</code></span>
                            // </p>

                        </div>
                        <div class="col-auto">
                            <div class="dropdown">
                                <button
                                    class="btn btn-primary dropdown-toggle mt-3 position-relative"
                                    role="button"
                                    id="dropdownMenuLink"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                    disabled={true}
                                >
                                    {"Actions"}
                                    <span
                                        class="position-absolute top-0 translate-middle-y badge rounded-pill bg-warning"
                                        style="left: 10px;"    
                                    >
                                        { "Inactive" }
                                    </span>
                                 </button>

                                <ul class="dropdown-menu" aria-labelledby="dropdownMenuLink">
                                    <li>
                                        <a class="dropdown-item" href="#"><i class="bi bi-envelope me-2"></i
                                            ><span>{"Send Verification Email"}</span></a
                                        >
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a class="dropdown-item" href="#">{"Change Email"}</a>
                                    </li>
                                    <li>
                                        <a class="dropdown-item" href="#">{"Change Password"}</a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a class="dropdown-item" href="#">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="me-1" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line></svg>
                                            <span> {"Block"} </span>
                                        </a>
                                    </li>
                                    <li>
                                        <a class="dropdown-item" href="#">
                                            <i class="bi bi-trash text-danger"></i>
                                            <span class="text-danger">{"Delete"}</span>
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>

                </div>

                <DevelopersNote message="Only the following tabs are working, 'Details', 'Permissions', and 'Roles'"/>


                <ul class="nav nav-tabs" id="myTab" role="tablist" style="font-size:14px;">
                    <li
                        onclick = self.link.callback(|_|Msg::ChangeContent(Content::UserTabDetails))
                        class="nav-item fw-bold">
                        <button
                            class={
                                match self.content {
                                    Content::UserTabDetails => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            id="user-details-tab"
                            data-bs-toggle="tab"
                            data-bs-target="#detailtab"
                            type="button" role="tab"
                            aria-controls="detailtab"
                            aria-selected="true">{"Details"}
                        </button>
                    </li>
                    <li
                        onclick =self.link.callback(|_|Msg::ChangeContent(Content::UserTabDevices))
                        class="nav-item fw-bold"
                        role="presentation"
                    >
                        <button
                            class={
                                match self.content {
                                    Content::UserTabDevices => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            id="user-devices-tab"
                            data-bs-toggle="tab"
                            data-bs-target="#devicetab"
                            type="button" role="tab"
                            aria-controls="devicetab"
                            aria-selected="false">{"Devices"}
                        </button>
                    </li>
                    <li 
                        onclick = self.link.callback(|_| Msg::ChangeContent(Content::UserTabHistory))
                        class="nav-item fw-bold" 
                        role="presentation">
                        <button 
                            class={
                                match self.content {
                                    Content::UserTabHistory => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            id="user-history-tab" 
                            data-bs-toggle="tab" 
                            data-bs-target="#historytab" 
                            type="button" role="tab" 
                            aria-controls="historytab" 
                            aria-selected="false">{"History"}
                        </button>
                    </li>
                    <li 
                        onclick = self.link.callback(|_|Msg::ChangeContent(Content::UserTabRawJson))
                        class="nav-item" 
                        role="presentation">
                        <button 
                            class={
                                match self.content {
                                    Content::UserTabRawJson => "nav-link active",
                                    _ => "nav-link"
                                }
                            } 
                            id="rawjson-tab" 
                            data-bs-toggle="tab" 
                            data-bs-target="#rawjsontab" 
                            type="button" 
                            role="tab" 
                            aria-controls="rawjsontab" 
                            aria-selected="false">{"RAW JSON"}
                        </button>
                    </li>
                    <li 
                        onclick = self.link.callback(|_|Msg::ChangeContent(Content::UserTabAuthorizedApp))
                        class="nav-item" 
                        role="presentation">
                        <button 
                            class= {
                                match self.content {
                                    Content::UserTabAuthorizedApp => "nav-link active",
                                    _ => "nav-link"
                                }
                            } 
                            id="authorapp-tab" 
                            data-bs-toggle="tab" 
                            data-bs-target="#authorapptab" 
                            type="button" 
                            role="tab" 
                            aria-controls="authorapptab" 
                            aria-selected="false">{"Authorized Applications"}
                        </button>
                    </li>
                    <li 
                        onclick = self.link.callback(|_| Msg::ChangeContent(Content::UserTabPermissions))
                        class="nav-item" 
                        role="presentation">
                        <button 
                            class= {
                                match self.content {
                                    Content::UserTabPermissions => "nav-link active",
                                    _ => "nav-link"
                                }
                            } 
                            id="permission-tab" 
                            data-bs-toggle="tab" 
                            data-bs-target="#permissiontab" 
                            type="button" 
                            role="tab" 
                            aria-controls="permissiontab" 
                            aria-selected="false">{"Permissions"}
                        </button>
                    </li>
                    <li 
                        onclick = self.link.callback(|_|Msg::ChangeContent(Content::UserTabRoles))
                        class="nav-item" 
                        role="presentation">
                        <button 
                            class= {
                                match self.content {
                                    Content::UserTabRoles => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            id="roles-tab" 
                            data-bs-toggle="tab" 
                            data-bs-target="#rolestab" 
                            type="button" 
                            role="tab" 
                            aria-controls="roles" 
                            aria-selected="false">{"Roles"}
                        </button>
                    </li>
                </ul>

                {
                    match self.content {
                        Content::UserTabDetails => html! {<UserTabDetails user_details=self.user_details.clone()/>},
                        Content::UserTabDevices => html! {<UserTabDevices/>},
                        Content::UserTabHistory => html! {<UserTabHistory/>},
                        Content::UserTabRawJson => html! {<UserTabRawJson/>},
                        Content::UserTabAuthorizedApp => html! {<UserTabAuthorizedApp/>},
                        Content::UserTabPermissions => html! {<UserTabPermissions user_id=self.user_details.user_id.clone()/>},
                        Content::UserTabRoles => html! {<UserTabRoles user_details=self.user_details.clone()/>},
                    }
                }

            </>
        }
    }
}
