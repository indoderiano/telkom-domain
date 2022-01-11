use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
    }
};
use yew_router::service::RouteService;
use crate::configs::server::API_URL;
use types::{
    roles::{
        Role,
        ResponseRoleDelete
    },
};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct TabSettingsProps {
    pub role: Role,
}

pub enum Data {
    Name,
    Description,
}

pub enum StateError {
    Update,
    Delete,
}

pub struct TabSettings {
    role: Role,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    loading_update: bool,
    error_update: Option<String>,
    loading_delete: bool,
    error_delete: Option<String>,
    route_service: RouteService,
}

pub enum Msg {
    Input(String, Data),
    Update,
    GetRoleDetails(Role),
    Delete,
    RedirectToRoles,
    ResponseError(String, StateError),
}

impl Component for TabSettings {
    type Message = Msg;
    type Properties = TabSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TabSettings {
            role: props.role,
            link,
            fetch_task: None,
            loading_update: false,
            error_update: None,
            loading_delete: false,
            error_delete: None,
            route_service: RouteService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(value, data) => {
                match data {
                    Data::Name => {
                        self.role.name = value;
                    }
                    Data::Description => {
                        self.role.description = value;
                    }
                }
                false
            }
            Msg::Update => {
                ConsoleService::info(&format!("role = {:?}", self.role));
                let request = Request::patch(format!("http://127.0.0.1:8080/api/v1/1/roles/{}", self.role.id.clone()))
                    .header("Content-Type", "application/json")
                    .header("access_token", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImhleWthbGxAZ21haWwuY29tIiwiZXhwIjoxNjQzMDk0MTA0fQ.G_kEzjOwrzI_qD8Tco_4HTgXctsz4kUccl4e92WNZb8")
                    .body(Json(&self.role))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Role, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        ConsoleService::info(&format!("ini yang di dapat = {:?}", data));
                        match data {
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("role details = {:?}", dataok));
                                Msg::GetRoleDetails(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::Update)
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_update = None;
                self.loading_update = true;
                true
            }
            Msg::GetRoleDetails(data) => {
                self.role = data;
                self.loading_update = false;
                self.fetch_task = None;
                true
            }
            Msg::Delete => {
                let request = Request::delete(format!("http://127.0.0.1:8080/api/v1/1/roles/{}",self.role.id.clone()))
                    .header("access_token", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImhleWthbGxAZ21haWwuY29tIiwiZXhwIjoxNjQzMDk0MTA0fQ.G_kEzjOwrzI_qD8Tco_4HTgXctsz4kUccl4e92WNZb8")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(|response: Response<Json<Result<ResponseRoleDelete, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                ConsoleService::info(&format!("{:?}", data));
                match data {
                    Ok(dataok) => {
                        // ConsoleService::info(&format!("{:?}", dataok));
                        ConsoleService::info(&format!("{:?}", dataok));
                        Msg::RedirectToRoles
                    }
                    Err(error) => {
                        ConsoleService::info(&error.to_string());
                        Msg::ResponseError(error.to_string(), StateError::Delete)
                    }
                }
                });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_delete = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::RedirectToRoles => {
                // self.loading_delete = false;
                self.fetch_task = None;
                self.route_service.set_route(&format!("/user-management/roles"), ());
                true
            }
            Msg::ResponseError (message, state) => {
                match state {
                    StateError::Update => {
                        self.fetch_task = None;
                        self.loading_update = false;
                        self.error_update = Some(message);
                    }
                    StateError::Delete => {
                        self.fetch_task = None;
                        self.loading_delete = false;
                        self.error_update = Some(message);
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
        let Role {
            id,
            name,
            description,
        } = self.role.clone();
        html! {
            <>
            <div class="mt-4 p-4">
                <form>
                    <div class="mb-3">
                        <label for="roleName" class="form-label">{"Name"}</label>
                        <input
                            type="text"
                            class="form-control w-50"
                            id="roleName"
                            value={ name.clone() }
                            disabled={ self.loading_update }
                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, Data::Name))
                        />
                    </div>
                    <div class="mb-3">
                        <label for="inputDescription" class="form-label">{"Description"}</label>
                        <input
                            type="text"
                            class="form-control w-50"
                            id="inputDescription"
                            value={ description.clone() }
                            disabled={ self.loading_update }
                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, Data::Description))
                        />
                    </div>

                        <div class="mb-3">
                            <label for="roleName" class="form-label">{"Name"}</label>
                            <input
                                type="text"
                                class="form-control w-50"
                                id="roleName"
                                value={ name.clone() }
                                disabled={ self.loading_update }
                            />
                        </div>


                        <div class="mb-3">
                            <label for="inputDescription" class="form-label">{"Description"}</label>
                            <input
                                type="text"
                                class="form-control w-50"
                                id="inputDescription"
                                value={ description.clone() }
                                disabled={ self.loading_update }
                            />
                        </div>


                        <div
                            // class="mt-3 mb-5"
                        >
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_update {"loading"} else {""} )
                                    onclick=self.link.callback(|_| Msg::Update)
                                    disabled={ self.loading_update }
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                {
                                    if self.error_update.is_some() {
                                    html! {
                                        <div class="alert alert-warning mt-3" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_update.clone().unwrap() }
                                        </div>
                                    }
                                    } else {
                                        html! {}
                                    }
                                }
                        </div>

                    //     // <button type="submit" class="btn btn-primary">{"Save"}</button>
                    </form>
                </div>


                <div class="mt-2 p-4 pt-0">
                    <p class="fw-bold fs-5">{"Danger Zone"}</p>
                    
                    <div class="alert alert-danger" role="alert">
                        <div class="row">
                            <div class="col">
                                <p class="text-danger fw-bold m-0">{"Delete Role"}</p>
                                <p class="text-danger m-0">{"Once confirmed, this operations can't be undone!"}</p>
                            </div>
                            <div class="col d-flex justify-content-end">
                                
                            <button
                                type="button"
                                class=format!("btn {} btn-danger position-relative", if self.loading_delete {"loading"} else {""} )
                                onclick=self.link.callback(|_| Msg::Delete)
                                disabled={ self.loading_delete }
                            >
                                <div class="telkom-label">
                                    {"Delete"}
                                </div>
                                <div class="telkom-spinner telkom-center">
                                    <div class="spinner-border spinner-border-sm" role="status"/>
                                </div>
                            </button>
                            {
                                if self.error_delete.is_some() {
                                    html! {
                                        <div class="alert alert-warning" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_delete.clone().unwrap() }
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                            </div>

                    //             // <button type="button" class="btn btn-danger">{"Remove this role!"}</button>
                            </div>
                        </div>
                </div>

            </>
        }
    }
}
