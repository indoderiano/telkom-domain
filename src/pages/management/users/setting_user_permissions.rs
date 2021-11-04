use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

use yew::services::ConsoleService;
use crate::components::loading2::Loading2;
use crate::configs::server::API_URL;
use crate::types::users::{UserPermissions};

pub struct UserTabPermissions {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    loading_get_user_permission: bool,
    user_permissions: Vec<UserPermissions>,
    error_user_permission_list: Option<String>
}

pub enum StateError{
    UserPermissionList
}

pub enum Msg {
    DefaultState,
    RequestUserPermissions,
    GetUserPermissions(Vec<UserPermissions>),
    ResponseError(String, StateError),
}

impl UserTabPermissions {
    fn view_user_permissions(&self) -> Vec<Html> {
        self.user_permissions.iter().map(|user|{
           html! {
               <tr>
                       <th scope="row">{&user.permission_name}</th>
                       <td>{&user.description}</td>
                       <td>{&user.resource_server_name}</td>
                       <td>{"Direct"}</td>
                       <td>
                           <button type="button" class="btn btn-outline-secondary px-2 py-1" data-bs-toggle="modal" data-bs-target="#permissionDeleteModal">
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

impl Component for UserTabPermissions {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        UserTabPermissions {
            link,
            fetch_task: None,
            loading_get_user_permission: false,
            user_permissions: Vec::new(),
            error_user_permission_list: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestUserPermissions);
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::DefaultState => {
                self.loading_get_user_permission = false;
                true
            }
            Msg::RequestUserPermissions => {
                ConsoleService::info("ini di request user permissions");
                let request = Request::get(format!("{}/users/tenantid/users/:id/permissions", API_URL))
                    .header("access_token", "telkomidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<UserPermissions>, anyhow::Error>>>| {
                        let Json(dataok) = response.into_body();
                        ConsoleService::info(&format!("{:?}", dataok));
                        match dataok{
                            Ok(dataok) => Msg::GetUserPermissions(dataok), 
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::UserPermissionList)
                            }
                        }
                    }
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_user_permission_list = None;
                self.loading_get_user_permission = true;
                true
            }
            Msg::GetUserPermissions(data) => {
                ConsoleService::info("ini di get user permissions");
                self.user_permissions = data;
                self.loading_get_user_permission = false;
                self.fetch_task = None;
                true
            }
            Msg::ResponseError(message, state) => {
                ConsoleService::info("ini di info response error");
                match state{
                    StateError::UserPermissionList => {
                        self.loading_get_user_permission = false;
                        self.error_user_permission_list = Some(message);
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
            <>
                <div class="mt-4">
                    <div class="row">
                        <div class="col d-flex justify-content-start">
                            <p>{"List of permissions this user has."}</p>
                        </div>
                        <div class="col d-flex justify-content-end">
                            <button type="button" class="btn btn-primary">{"Assign Permissions"}</button>
                        </div>
                    </div>
                </div>
                <div class="mt-4 table-responsive">
                    <table class="table">
                        <thead>
                            <tr>
                                <th scope="col">{"Name"}</th>
                                <th scope="col">{"Description"}</th>
                                <th scope="col">{"API"}</th>
                                <th scope="col">{"Assignment"}</th>
                                <th scope="col"></th>
                            </tr>
                        </thead>
                        {
                            if self.loading_get_user_permission {
                                html!{
                                    <div style="position: relative; margin-top:8rem;">
                                        <Loading2 width = 45 />
                                    </div>
                                }
                            } else if self.error_user_permission_list.is_some() {
                                html! {
                                    <tr>
                                        <div class="alert alert-warning mb-5" role="alert">
                                        <i class="bi bi-exclamation-triangle me-2"></i>
                                        { self.error_user_permission_list.clone().unwrap() }
                                        </div>
                                    </tr>
                                }
                            } else {
                                html! {
                                    <>
                                        {self.view_user_permissions()}
                                    </>
                                }
                            }
                        }
                        // <tbody>
                        //         <tr>
                        //             <th scope="row">{"create:client_grants"}</th>
                        //             <td>{"Create New Data"}</td>
                        //             <td>{"Example API"}</td>
                        //             <td>{"Direct"}</td>
                        //             <td>
                        //                 <button type="button" class="btn btn-outline-secondary px-2 py-1" data-bs-toggle="modal" data-bs-target="#permissionDeleteModal">
                        //                     <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                        //                         <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                        //                         <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                        //                       </svg>
                        //                 </button>
                        //             </td>
                        //         </tr>
                        //         <tr>
                        //             <th scope="row">{"read:client_grants"}</th>
                        //             <td>{"Read Data"}</td>
                        //             <td>{"Example API"}</td>
                        //             <td>{"Direct"}</td>
                        //             <td>
                        //                 <button type="button" class="btn btn-outline-secondary px-2 py-1" data-bs-toggle="modal" data-bs-target="#permissionDeleteModal">
                        //                     <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                        //                         <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                        //                         <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                        //                       </svg>
                        //                 </button>
                        //             </td>
                        //         </tr>
                        //         <tr>
                        //             <th scope="row">{"update:client_grants"}</th>
                        //             <td>{"Update New Data"}</td>
                        //             <td>{"Example API"}</td>
                        //             <td>{"Direct"}</td>
                        //             <td>
                        //                 <button type="button" class="btn btn-outline-secondary px-2 py-1" data-bs-toggle="modal" data-bs-target="#permissionDeleteModal">
                        //                     <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                        //                         <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                        //                         <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                        //                       </svg>
                        //                 </button>
                        //             </td>
                        //         </tr>
                        //         <tr>
                        //             <th scope="row">{"delete:client_grants"}</th>
                        //             <td>{"Delete Data"}</td>
                        //             <td>{"Example API"}</td>
                        //             <td>{"Direct"}</td>
                        //             <td>
                        //                 <button type="button" class="btn btn-outline-secondary px-2 py-1" data-bs-toggle="modal" data-bs-target="#permissionDeleteModal">
                        //                     <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                        //                         <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                        //                         <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                        //                       </svg>
                        //                 </button>
                        //             </td>
                        //         </tr>
                        //     </tbody>
                    </table>


                    <div class="modal fade" id="permissionDeleteModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                        <div class="modal-dialog modal-dialog-centered">
                            <div class="modal-content">
                                <div class="modal-header">
                                    <h5 class="modal-title" id="exampleModalLabel">{"Remove from Role?"}</h5>
                                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                                </div>
                                <div class="modal-body">
                                    {"Are you sure that you want to unassign permission 'create:client_grants'?"}
                                </div>
                                <div class="modal-footer">
                                    <button type="button" class="btn btn-outline-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                                    <button type="button" class="btn btn-danger">{"Yes, remove"}</button>
                                </div>
                            </div>
                        </div>
                    </div>





                </div>
            </>
        }
    }
}
