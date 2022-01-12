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
use crate::pages::management::users::modal_assign_permissions::ModalAssignPermissions;
use crate::configs::server::API_URL;
use crate::types::{
    users::{UserPermissions},
    api::{ ApiTitle, Scope },
    ResponseMessage,
    LocalStorage,
    LOCALSTORAGE_KEY,
};
use yew_router::service::RouteService;


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct UserTabPermissionsProps {
    pub user_id: String,
}

pub struct UserTabPermissions {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    user_id: String,
    access_token: String,
    user_permissions: Vec<UserPermissions>,
    loading_get_user_permission: bool,
    error_user_permission_list: Option<String>,
    show_modal_delete_permission: bool,
    index_permission_delete: Option<usize>,
    loading_delete_permissions: bool,
    error_delete_permissions: Option<String>,
    route_service: RouteService,

    // assign permission
    // loading_get_apis: bool,
    // error_get_apis: Option<String>,
    // apis: Vec<ApiTitle>,
    // selected_api_id: Option<String>,
    // selected_permissions: Option<Vec<Scope>>
}

pub enum StateError{
    UserPermissionList,
    Delete,
    // RequestApis,
}

pub enum Msg {
    DefaultState,
    RequestUserPermissions,
    GetUserPermissions(Vec<UserPermissions>),
    ShowModalDeletePermission(bool, Option<usize>),
    Delete,
    ResponseError(String, StateError),
    RedirectToPermissions,

    // RequestApis,
    // GetApis(Vec<ApiTitle>),
    // SelectApi(String),
}

impl Component for UserTabPermissions {
    type Message = Msg;
    type Properties = UserTabPermissionsProps;

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
        
        UserTabPermissions {
            link,
            fetch_task: None,
            user_id: props.user_id,
            access_token,
            user_permissions: Vec::new(),
            loading_get_user_permission: false,
            error_user_permission_list: None,
            show_modal_delete_permission: false,
            index_permission_delete: None,
            loading_delete_permissions: false,
            error_delete_permissions: None,
            route_service: RouteService::new(),

            // Modal assign permission
            // loading_get_apis: false,
            // error_get_apis: None,
            // apis: Vec::new(),
            // selected_api_id: None,
            // selected_permissions: None,
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

                // default state
                self.loading_delete_permissions = false;
                self.fetch_task = None;
                self.show_modal_delete_permission = false;
                self.index_permission_delete = None;

                let request = Request::get(format!("{}/api/v2/users/{}/permissions", API_URL, self.user_id.clone()))
                    .header("access_token", self.access_token.clone())
                    .body(Nothing)
                    .expect("Could not build request");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<UserPermissions>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        ConsoleService::info(&format!("{:?}", data));
                        match data{
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
            Msg::ShowModalDeletePermission(state, index_selected) => {
                self.show_modal_delete_permission = state;
                self.index_permission_delete = index_selected;
                true
            }
            Msg::ResponseError(message, state) => {
                ConsoleService::info("ini di info response error");
                match state{
                    StateError::UserPermissionList => {
                        self.loading_get_user_permission = false;
                        self.error_user_permission_list = Some(message);
                    }
                    StateError::Delete => {
                        self.loading_delete_permissions = false;
                        self.error_delete_permissions = Some(message);
                    }
                    // StateError::RequestApis => {
                    //     self.loading_get_apis = false;
                    //     self.error_get_apis = Some(message)
                    // }
                }
                self.fetch_task = None;
                true
            }
            Msg::Delete => {
                // remove permission from vector
                // let new_permissions: Vec<UserPermissions> = self.user_permissions
                // .iter()
                // .enumerate()
                // .filter(|(i, e)| {
                //     if self.index_permission_delete.is_some() {
                //         *i != self.index_permission_delete.unwrap()
                //     } else {
                //         true
                //     }
                // })
                // .map(|(_s, x)| {
                //     x.clone()
                // })
                // .collect();
                // ConsoleService::info(&format!("new permissions = {:?}", new_permissions));

                // let request = Request::delete(format!("{}/users/tenant_id/users/auth0|7CYXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI/permissions", API_URL))
                //     .header("access_token", "telkomidtelkomdomain")
                //     .header("Content-Type", "application/json")
                //     .body(Json(&new_permissions))
                //     .expect("could not build request");
                // let callback = self.link.callback(|response: Response<Json<Result<ResponseMessage, anyhow::Error>>>|{
                //     let Json(data) = response.into_body();
                //     match data{
                //         Ok(dataok) => {
                //             ConsoleService::info(&format!("{:?}", dataok));
                //             Msg::RequestUserPermissions
                //         }
                //         Err(error) => {
                //             ConsoleService::info(&error.to_string());
                //             Msg::ResponseError(error.to_string(), StateError::Delete)
                //         }
                //     }
                // });
                // let task = FetchService::fetch(request, callback).expect("failed to start request");
                // self.loading_delete_permissions = true;
                // self.fetch_task = Some(task);
                // true

                #[derive(Serialize, Debug, Clone, PartialEq)]
                struct SelectedPermission {
                    permission_name: String,
                    resource_server_identifier: String,
                }
                #[derive(Serialize, Debug, Clone, PartialEq)]
                struct DataDeletePermissions {
                    permissions: Vec<SelectedPermission>,
                }

                let data_delete_permissions = DataDeletePermissions {
                    permissions: vec![
                        SelectedPermission {
                            permission_name: self.user_permissions[self.index_permission_delete.unwrap()].permission_name.clone(),
                            resource_server_identifier: self.user_permissions[self.index_permission_delete.unwrap()].resource_server_identifier.clone()
                        }
                    ]
                };
                let request = Request::delete(format!("{}/api/v2/users/{}/permissions", API_URL, self.user_id))
                    .header("access_token", self.access_token.clone())
                    .header("Content-Type", "application/json")
                    .body(Json(&data_delete_permissions))
                    .expect("could not build request");
                let callback = self.link.callback(|response: Response<Json<Result<(), anyhow::Error>>>|{
                    let (meta, Json(data)) = response.into_parts();
                    let status_number = meta.status.as_u16();

                    match status_number {
                        204 => {
                            Msg::RequestUserPermissions
                        }
                        _ => {
                            match data {
                                Ok(dataok) => {
                                    Msg::RequestUserPermissions
                                }
                                Err(error) => {
                                    Msg::ResponseError(error.to_string(), StateError::Delete)
                                }
                            }
                        }
                    }
                });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_delete_permissions = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::RedirectToPermissions => {
                self.loading_delete_permissions = false;
                self.fetch_task = None;
                self.route_service.set_route(&format!("/{}/permissions", "tenant_id_not_from_reducer"), ());
                true
            }

            // Msg::RequestApis => {
            //     let request = Request::get(format!("{}/api/v2/resource-server", API_URL))
            //         .header("access_token", self.access_token.clone())
            //         .body(Nothing)
            //         .expect("Could not build request");
            //     let callback = self.link.callback(
            //         |response: Response<Json<Result<Vec<ApiTitle>, anyhow::Error>>>| {
            //             let Json(data) = response.into_body();
            //             // ConsoleService::info(&format!("{:?}", data));
            //             match data{
            //                 Ok(dataok) => Msg::GetApis(dataok), 
            //                 Err(error) => {
            //                     Msg::ResponseError(error.to_string(), StateError::RequestApis)
            //                 }
            //             }
            //         }
            //     );

            //     let task = FetchService::fetch(request, callback).expect("failed to start request");
            //     self.fetch_task = Some(task);
            //     self.error_get_apis = None;
            //     self.loading_get_apis = true;
            //     true
            // }
            // Msg::GetApis(apis) => {
            //     self.apis = apis;
            //     self.loading_get_apis = false;
            //     self.fetch_task = None;
            //     true
            // }
            // Msg::SelectApi(index) => {
            //     ConsoleService::info(&format!("index = {}", index));
            //     if index.is_empty() {
            //         ConsoleService::info("index is empty");
            //     } else {
            //         ConsoleService::info(&format!("selected api id = {}", self.apis[index.parse::<usize>().unwrap()].resource_server_id));
            //         ConsoleService::info(&format!("selected permissions are = {:?}", self.apis[index.parse::<usize>().unwrap()].scopes));
            //         self.selected_api_id = Some(self.apis[index.parse::<usize>().unwrap()].resource_server_id.clone());
            //         self.selected_permissions = Some(self.apis[index.parse::<usize>().unwrap()].scopes.clone());
            //     }
            //     true
            // }
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
                            <button
                                type="button"
                                class="btn btn-primary"
                                data-bs-toggle="modal"
                                data-bs-target="#addPermissions"
                                // onclick=self.link.callback(|_| Msg::RequestApis)
                            >
                                {"Assign Permissions"}
                            </button>
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
                        <tbody>
                        {
                            if !self.loading_get_user_permission && !self.error_user_permission_list.is_some() {
                                html! { 
                                    <> 
                                        { self.view_user_permissions() }
                                    </>
                                }
                            } else {
                                html! {}
                            }
                        }
                        </tbody>
                        
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

                    {
                        if self.loading_get_user_permission {
                        // if true {
                            html!{
                                <div style="margin-top:6rem;">
                                    <Loading2 width = 45 />
                                </div>
                            }
                        } else if self.error_user_permission_list.is_some() {
                            html! {
                                <div class="alert alert-warning" role="alert">
                                <i class="bi bi-exclamation-triangle me-2"></i>
                                { self.error_user_permission_list.clone().unwrap() }
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }


                    { self.modal_delete_permission() }


                    <ModalAssignPermissions user_permissions=self.user_permissions.clone() user_id=self.user_id.clone() />




                </div>
            </>
        }
    }
}


impl UserTabPermissions {
    fn view_user_permissions(&self) -> Vec<Html> {
        // https://stackoverflow.com/questions/58737024/how-to-get-the-index-of-the-current-element-being-processed-in-the-iteration-wit
        self.user_permissions
        .iter()
        .enumerate()
        .map(|(i, user)|{
           html! {
               <tr class="align-middle">
                    <th scope="row">{&user.permission_name}</th>
                    <td>{&user.description}</td>
                    <td>{&user.resource_server_name}</td>
                    <td>{"Direct"}</td>
                    <td>
                        <button
                            type="button"
                            class="btn btn-outline-secondary px-2 py-1"
                            // data-bs-toggle="modal"
                            // data-bs-target="#permissionDeleteModal"
                            onclick=self.link.callback(move |_| Msg::ShowModalDeletePermission(true, Some(i)))
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

    fn modal_delete_permission(&self) -> Html {
        html! {
            <>
                <div
                    class=format!("modal fade {}", if self.show_modal_delete_permission {"show"} else {""})
                    // id="permissionDeleteModal"
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
                                    onclick=self.link.callback(move |_| Msg::ShowModalDeletePermission(false, None))
                                ></button>
                            </div>
                            <div class="modal-body">
                                {"Are you sure that you want to unassign permission '"}
                                { 
                                    if self.index_permission_delete.is_some() {
                                        html! {
                                            <>
                                                { self.user_permissions[self.index_permission_delete.unwrap()].permission_name.clone() }
                                            </>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                {"'?"}
                            </div>
                            <div class="modal-footer">
                                <button
                                    type="button"
                                    class="btn btn-outline-secondary"
                                    data-bs-dismiss="modal"
                                    onclick=self.link.callback(move |_| Msg::ShowModalDeletePermission(false, None))
                                >
                                    {"Cancel"}
                                </button>
                                <button
                                    type="button"
                                    class=format!("btn {} btn-danger position-relative", if self.loading_delete_permissions {"loading"} else {""} )
                                    onclick=self.link.callback(|_| Msg::Delete)
                                    disabled={ self.loading_delete_permissions }
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
                                if self.error_delete_permissions.is_some() {
                                    html! {
                                        <div class="modal-footer">
                                            <div class="alert alert-warning" role="alert">
                                                <i class="bi bi-exclamation-triangle me-2"></i>
                                                { self.error_delete_permissions.clone().unwrap() }
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
                    class=format!("modal-backdrop fade {}", if self.show_modal_delete_permission {"show"} else {""})
                    onclick=self.link.callback(move |_| Msg::ShowModalDeletePermission(false, None))
                />
            </>
        }
    }

}