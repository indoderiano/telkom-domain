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
use crate::components::loading2::Loading2;
use crate::pages::management::users::modal_assign_roles::ModalAssignRoles;
use crate::configs::server::API_URL;
use crate::types::{
    users::{ UserRole, UserDetails },
    ResponseMessage,
    LocalStorage,
    LOCALSTORAGE_KEY,
};
use yew_router::service::RouteService;


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct UserTabRolesProps {
    pub user_details: UserDetails,
}

pub struct UserTabRoles {
    user_details: UserDetails,
    access_token: String,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    user_roles: Vec<UserRole>,
    loading_get_user_roles: bool,
    error_get_user_roles: Option<String>,
    show_modal_delete_role: bool,
    index_role_delete: Option<usize>,
    loading_delete_roles: bool,
    error_delete_roles: Option<String>,
    route_service: RouteService,
}

pub enum Msg {
    RequestUserRoles,
    GetUserRoles(Vec<UserRole>),
    ShowModalDeleteRole(bool, Option<usize>),
    Delete,
    ResponseError(String, StateError),
    RedirectToRoles,
}

pub enum StateError {
    GetUserRoles,
    Delete,
}

impl Component for UserTabRoles {
    type Message = Msg;
    type Properties = UserTabRolesProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let user_roles = UserRoles::new();

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

        UserTabRoles {
            user_details: props.user_details,
            access_token,
            link,
            fetch_task: None,
            user_roles: vec![],
            loading_get_user_roles: false,
            error_get_user_roles: None,
            show_modal_delete_role: false,
            index_role_delete: None,
            loading_delete_roles: false,
            error_delete_roles: None,
            route_service: RouteService::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestUserRoles)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestUserRoles => {
                // default state
                self.loading_delete_roles = false;
                self.fetch_task = None;
                self.show_modal_delete_role = false;
                self.index_role_delete = None;

                let request = Request::get(format!("{}/api/v2/users/{}/roles", API_URL, self.user_details.user_id.clone()))
                    .header("access_token", self.access_token.clone())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<UserRole>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => Msg::GetUserRoles(dataok),
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::GetUserRoles)
                            }
                        }
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.loading_get_user_roles = true;
                true
            }
            Msg::GetUserRoles(data) => {
                self.user_roles = data;
                self.fetch_task = None;
                self.loading_get_user_roles = false;
                true
            }
            Msg::ShowModalDeleteRole(state, index_selected) => {
                self.show_modal_delete_role = state;
                self.index_role_delete = index_selected;
                true
            }
            Msg::Delete => {

                // VALIDATION
                if self.index_role_delete.is_some() {
                    #[derive(Serialize, Debug, Clone, PartialEq)]
                    struct DataRemoveRoles {
                        roles: Vec<String>
                    }
    
                    let data_remove_roles = DataRemoveRoles {
                        roles: vec![
                            self.user_roles[self.index_role_delete.unwrap()].id.clone()
                        ]
                    };

                    let request = Request::delete(format!("{}/api/v2/users/{}/roles", API_URL, self.user_details.user_id.clone()))
                        .header("access_token", self.access_token.clone())
                        .header("Content-Type", "application/json")
                        .body(Json(&data_remove_roles))
                        .expect("Could not build request");
                    let callback = self.link.callback(|response: Response<Json<Result<(), anyhow::Error>>>| {
                        // let Json(data) = response.into_body();
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        
                        match status_number {
                            204 => {
                                Msg::RequestUserRoles
                            }
                            _ => {
                                match data {
                                    Ok(dataok) => {
                                        // ConsoleService::info(&format!("{:?}", dataok));
                                        Msg::RequestUserRoles
                                    }
                                    Err(error) => {
                                        ConsoleService::info(&error.to_string());
                                        Msg::ResponseError(error.to_string(), StateError::Delete)
                                    }
                                }
                            }
                        }
                    });
                    let task = FetchService::fetch(request, callback).expect("failed to start request");
                    self.loading_delete_roles = true;
                    self.fetch_task = Some(task);
                } else {
                    self.link.send_message(Msg::ResponseError("No roles have been selected".to_string(), StateError::Delete))
                }

                // remove role from vector
                // let new_roles: Vec<UserRole> = self.user_roles
                // .iter()
                // .enumerate()
                // .filter(|(i, e)| {
                //     if self.index_role_delete.is_some() {
                //         *i != self.index_role_delete.unwrap()
                //     } else {
                //         true
                //     }
                // })
                // .map(|(_s, x)| {
                //     x.clone()
                // })
                // .collect();
                // ConsoleService::info(&format!("new roles = {:?}", new_roles));

                
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::GetUserRoles => {
                        self.loading_get_user_roles = false;
                        self.error_get_user_roles = Some(message);
                    }
                    StateError::Delete => {
                        self.loading_delete_roles = false;
                        self.error_delete_roles = Some(message);
                    }
                }
                self.fetch_task = None;
                true
            }
            Msg::RedirectToRoles => {
                self.loading_delete_roles = false;
                self.fetch_task = None;
                self.route_service.set_route(&format!("/{}/roles", "tenant_id_not_from_reducer"), ());
                true
            }
            
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <div class="mt-4">

                <div class="row">
                    <div class="col text-center d-flex justify-content-start m-0">
                        <p>{"All Roles assigned to this User."}</p>
                    </div>
                    <div class="col d-flex justify-content-end">
                        <button
                            type="button"
                            class="btn btn-primary"
                            data-bs-toggle="modal"
                            data-bs-target="#assignRoles"
                            // onclick=self.link.callback(|_| Msg::RequestApis)
                        >
                            {"Assign Roles"}
                        </button>
                    </div>
                </div>

            <div class="mt-4 table-responsive">
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">{"Name"}</th>
                            <th scope="col">{"Description"}</th>
                            <th scope="col">{"Assignment"}</th>
                            <th scope="col"></th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            if !self.loading_get_user_roles && !self.error_get_user_roles.is_some() {
                                html! { 
                                    <>
                                        { self.view_content() }
                                    </>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </tbody>
                </table>
                {
                    if self.loading_get_user_roles {
                        html! {
                            <div style="position: relative; margin-top:4rem;">
                                <Loading2 width = 45 />
                            </div>
                        }
                    } else if self.error_get_user_roles.is_some() {
                        html! {
                            <div class="alert alert-warning mb-5" role="alert">
                                <i class="bi bi-exclamation-triangle me-2"></i>
                                { self.error_get_user_roles.clone().unwrap() }
                            </div>
                        }
                    } else {
                        html! { }
                    }
                }
            </div>

            // MODAL DELETE ROLE
            <div
                class=format!("modal fade {}", if self.show_modal_delete_role {"show"} else {""})
                // id="exampleModal"
                // tabindex="-1"
                // aria-labelledby="exampleModalLabel"
                // aria-hidden="true"
            >
                <div class="modal-dialog modal-dialog-centered">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title" id="exampleModalLabel">{"Remove from Role?"}</h5>
                            <button
                                type="button"
                                class="btn-close"
                                data-bs-dismiss="modal"
                                aria-label="Close"
                                onclick=self.link.callback(move |_| Msg::ShowModalDeleteRole(false, None))
                            ></button>
                        </div>
                        <div class="modal-body">
                            {
                                format!(
                                    "Are you sure that you want to remove {} from role '{}'?",
                                    self.user_details.name,
                                    if self.index_role_delete.is_some() { self.user_roles[self.index_role_delete.unwrap()].name.clone() }
                                    else {"".to_string()}
                                )
                            }
                            // {"Are you sure that you want to remove Yeska Haganta from role '"}
                            
                            // {
                            //     if self.index_role_delete.is_some() {
                            //         html! {
                            //             <>
                            //                 { self.user_roles[self.index_role_delete.unwrap()].name.clone() }
                            //             </>
                            //         }
                            //     } else {
                            //         html! {}
                            //     }
                            // }
                            // {"'?"}
                        </div>
                        <div class="modal-footer">
                            <button
                                type="button"
                                class="btn btn-outline-secondary"
                                data-bs-dismiss="modal"
                                onclick=self.link.callback(move |_| Msg::ShowModalDeleteRole(false, None))
                            >
                                {"Cancel"}
                            </button>
                            <button 
                                type="button" 
                                class=format!("btn {} btn-danger position-relative", if self.loading_delete_roles {"loading"} else {""} )
                                onclick=self.link.callback(|_|Msg::Delete)
                                disabled={ self.loading_delete_roles }
                            >
                                <div class="telkom-label">
                                    {"Yes, remove"}
                                </div>
                                <div class="telkom-spinner telkom-center">
                                    <div class="spinner-border spinner-border-sm" role="status"/>
                                </div>
                            </button>
                        </div>
                        {
                            if self.error_delete_roles.is_some() {
                                html! {
                                    <div class="modal-footer">
                                        <div class="alert alert-warning" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_delete_roles.clone().unwrap() }
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

            // MODAL ASSIGN ROLES
            <ModalAssignRoles user_roles=self.user_roles.clone() user_id=self.user_details.user_id.clone() />
        </div>


        <div
            class=format!("modal-backdrop fade {}", if self.show_modal_delete_role {"show"} else {""})
            onclick=self.link.callback(move |_| Msg::ShowModalDeleteRole(false, None))
        />
            </>
        }
    }

}

impl UserTabRoles {
    fn view_content(&self) -> Vec<Html> {
        self.user_roles
        .iter()
        .enumerate()
        .map(|(i, role)| {
            let UserRole {
                id: _,
                name,
                description,
            } = role.clone();

            html! {
                <tr>
                    <th scope="row" class="align-middle">
                        <a href="">{name}</a>
                    </th>
                    <td class="align-middle">{description}</td>
                    <td class="align-middle">{"Direct"}</td>
                    <td class="text-end">
                        <button 
                            type="button" 
                            class="btn btn-outline-secondary px-2 py-1" 
                            // data-bs-toggle="modal" 
                            // data-bs-target="#exampleModal"
                            onclick=self.link.callback(move |_| Msg::ShowModalDeleteRole(true, Some(i)))
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                                <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                                <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                            </svg>
                        </button>
                    </td>
                </tr>
            }


        }).collect()

    }
}
