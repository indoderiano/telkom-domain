use super::tab_permission::TabPermissions;
use super::tab_setting::TabSettings;
use super::tab_users::TabUsers;
use crate::app::AppRoute;
use crate::components::loading2::Loading2;
use crate::configs::server::API_URL;
use crate::types::roles::Role;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use yew_router::components::RouterAnchor;

pub enum Content {
    Settings,
    Permissions,
    Users,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct RoleSettingsProps {
    pub role_id: String,
}

pub enum StateError {
    RequestRoleDetails,
}

pub struct ViewDetail {
    content: Content,
    link: ComponentLink<Self>,
    role_id: String,
    fetch_task: Option<FetchTask>,
    role: Role,
    loading_request_role: bool,
    error_request_role: Option<String>,
}

pub enum Msg {
    ChangeContent(Content),
    RequestRoleDetails,
    GetRoleDetails(Role),
    ResponseError(String, StateError),
}

impl Component for ViewDetail {
    type Message = Msg;
    type Properties = RoleSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ViewDetail {
            content: Content::Settings,
            link,
            role_id: props.role_id,
            fetch_task: None,
            role: Role::new(),
            loading_request_role: false,
            error_request_role: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestRoleDetails);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeContent(content) => {
                self.content = content;
                true
            }
            Msg::RequestRoleDetails => {
                let request = Request::get(format!("http://127.0.0.1:8080/api/v1/1/roles/{}", self.role_id.clone()))
                    .header("access_token", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImhleWthbGxAZ21haWwuY29tIiwiZXhwIjoxNjQzMDk0MTA0fQ.G_kEzjOwrzI_qD8Tco_4HTgXctsz4kUccl4e92WNZb8")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Role, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("role details = {:?}", dataok));
                                    Msg::GetRoleDetails(dataok)
                                }
                                Err(error) => Msg::ResponseError(
                                    error.to_string(),
                                    StateError::RequestRoleDetails,
                                ),
                            }
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_request_role = None;
                self.loading_request_role = true;
                true
            }
            Msg::GetRoleDetails(data) => {
                self.role = data;
                self.fetch_task = None;
                self.loading_request_role = false;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::RequestRoleDetails => {
                        self.loading_request_role = false;
                        self.error_request_role = Some(message);
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
        html! {
            if self.loading_request_role {
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
            } else if self.error_request_role.is_some() {
                html! {
                    <div class="alert alert-warning mb-5" role="alert">
                        <i class="bi bi-exclamation-triangle me-2"></i>
                        { self.error_request_role.clone().unwrap() }
                    </div>
                }
            } else {
                self.view_content()
            }
        }
    }
}

impl ViewDetail {
    fn view_content(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        let Role {
            id,
            name,
            description: _,
        } = self.role.clone();
        html! {
            <>
            <div class="mx-auto pt-5 pb-5 px-4" style="max-width: 1048px;">
                <div>
                    <Anchor route=AppRoute::RolesCreated classes="text-decoration-none text-muted">
                        <i class="bi bi-arrow-left"></i>
                        <span>{"Back To Roles"}</span>
                    </Anchor>
                </div>

                <div class="mt-2">
                    <p class="fw-bold fs-2">{ name.clone() }</p>
                    <div class="pt-2">
                        <span class="text-muted">{"Role ID"}</span>
                        <code class="text-dark ms-2" style="background-color: #eff0f2; font-family: Roboto, sans-serif;">{ id.clone() }</code>
                    </div>
                </div>

                <div class="mt-4">
                    <ul class="nav nav-tabs">
                            <li onclick=self.link.callback(|_|Msg::ChangeContent(Content::Settings)) class="nav-item">
                                <a class={
                                   match self.content {
                                       Content::Settings => "nav-link active",
                                       _ => "nav-link"
                                   }
                                }
                                aria-current="page"
                                href="#">{"Settings"}</a>
                            </li>
                            <li onclick=self.link.callback(|_|Msg::ChangeContent(Content::Permissions)) class="nav-item">
                                <a class={
                                    match self.content{
                                        Content::Permissions => "nav-link active",
                                        _ => "nav-link"
                                    }
                                }
                                href="#">{"Permissions"}</a>
                            </li>
                            <li onclick=self.link.callback(|_|Msg::ChangeContent(Content::Users)) class="nav-item">
                                <a class={
                                    match self.content{
                                        Content::Users => "nav-link active",
                                        _ => "nav-link"
                                    }
                                } href="#">{"Users"}</a>
                            </li>
                    </ul>
                </div>

                {
                    match self.content {
                        Content::Settings => html! { <TabSettings role=self.role.clone()/> },
                        Content::Permissions => html! { <TabPermissions/> },
                        Content::Users => html! { <TabUsers/> }
                    }
                }



            </div>
            </>
        }
    }
}
