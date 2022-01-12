use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        storage::{ StorageService, Area },
    },
};

use crate::app::AppRoute;
use crate::types::{
    application::{AppList, AppCreate},
    ResponseMessage,
};
use yew::services::ConsoleService;
use yew_router::components::RouterAnchor;

use crate::components::loading2::Loading2;

use crate::types::LocalStorage;
use crate::types::LOCALSTORAGE_KEY;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct AppProps {
    pub tenant_id: String,
}

pub enum StateError {
    AppList,
    AppCreate,
}

pub enum DataAppCreate {
    Name,
    AppType,
}

pub struct ApplicationHome {
    tenant_id: String,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    loading_get_app: bool,
    app_list: Vec<AppList>,
    error_app_list: Option<String>,
    app_create: AppCreate,
    show_modal_create: bool,
    loading_create_app: bool,
    error_app_create: Option<String>,
    access_token: String,
}

pub enum Msg {
    DefaultState,
    RequestAppList,
    GetAppList(Vec<AppList>),
    Input(String, DataAppCreate),
    ShowModalCreate(bool),
    Create,
    ResponseError(String, StateError),
}


impl Component for ApplicationHome {
    type Message = Msg;
    type Properties = AppProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info(&format!("Apps home props, tenant id = {}", props.tenant_id));
        
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

        let app_create = AppCreate::new();
        
        ApplicationHome {
            tenant_id: props.tenant_id,
            error_app_list: None,
            fetch_task: None,
            link,
            loading_get_app: false,
            app_list: Vec::new(),
            app_create,
            show_modal_create: false,
            loading_create_app: false,
            error_app_create: None,
            access_token,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info("This is first render");
            self.link.send_message(Msg::RequestAppList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DefaultState => {
                self.show_modal_create = false;
                self.loading_get_app = false;
                // self.error_app_list: None;
                self.loading_create_app = false;
                // self.error_app_create: None;
                self.app_create.name = String::from("");
                self.app_create.app_type = String::from("");
                true
            }
            Msg::RequestAppList => {
                let request = Request::get("https://evening-cliffs-55855.herokuapp.com/api/v2/clients")
                    .header("access_token", self.access_token.clone())
                    .header("Content-Type", "application/json")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<AppList>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        // ConsoleService::info(&format!("{:?}", &data.unwrap()));
                        // Msg::GetAppList(data.unwrap())
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", &dataok));
                                Msg::GetAppList(dataok)
                            } 
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::AppList)
                            }
                        }
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_app_list = None;
                self.loading_get_app = true;
                true
            }
            Msg::GetAppList(data) => {
                self.app_list = data;
                self.loading_get_app = false;
                self.fetch_task = None;
                true
            }
            Msg::Input(input, data) => {
                match data {
                    DataAppCreate::Name => {
                        self.app_create.name = input;
                    }
                    DataAppCreate::AppType => {
                        self.app_create.app_type = input;
                    }
                }
                true
            }
            Msg::ShowModalCreate(state) => {
                self.show_modal_create = state;
                true
            }
            Msg::Create => {
                ConsoleService::info(&format!("{:?}", self.app_create));
                let request = Request::post("https://evening-cliffs-55855.herokuapp.com/api/v2/clients")
                    .header("Content-Type", "application/json")
                    .header("access_token", self.access_token.clone())
                    .body(Json(&self.app_create))
                    .expect("Could not build request.");
                let callback = 
                    self.link.batch_callback(|response: Response<Json<Result<AppList, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("ini response berhasil{:?}", dataok));
                                vec![Msg::DefaultState, Msg::RequestAppList]
                            }
                            Err(error) => {
                                ConsoleService::info(&error.to_string());
                                vec![Msg::ResponseError(error.to_string(), StateError::AppCreate)]
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_create_app = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::AppList => {
                        self.loading_get_app = false;
                        self.error_app_list = Some(message);
                    }
                    StateError::AppCreate => {
                        self.loading_create_app = false;
                        self.error_app_create = Some(message);
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
        type Anchor = RouterAnchor<AppRoute>;
        html! {
        <>
                    <div
                        class="col py-3"
                    >
                        <div>
                            <div
                                class="mx-auto pt-5 pb-5 px-4"
                                style="max-width: 1048px;"
                            >
                                <div
                                    class="mb-5"
                                >
                                    <div
                                        class="d-flex flex-row mb-3"
                                    >
                                    <div
                                        class="flex-fill fs-3 fw-bold"
                                    >
                                        {"Applications"}
                                    </div>
                                    <div>
                                        <button
                                            type="button"
                                            class="btn btn-primary d-flex align-items-center"
                                            // data-bs-toggle="modal"
                                            // data-bs-target="#exampleModal"
                                            onclick=self.link.callback(|_| {Msg::ShowModalCreate(true)})
                                        >
                                            <i
                                                class="bi bi-plus me-2"
                                                style="margin-left: -5px;"
                                            >
                                            </i>
                                            <span>{"Create Application"}</span>
                                        </button>
                                    </div>
                                    </div>
                                    <p>{"Setup a application to use for Authentication."}</p>
                                </div>

                        // <!-- LIST -->
                        
                        {
                    if self.loading_get_app {
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
                    } else if self.error_app_list.is_some() {
                        html! {
                            <div class="alert alert-warning mb-5" role="alert">
                                <i class="bi bi-exclamation-triangle me-2"></i>
                                { self.error_app_list.clone().unwrap() }
                            </div>
                        }
                    } else if self.app_list.len() == 0 {
                        html! {
                            <>
                                { self.view_app_list_empty() }
                            </>
                        }
                    } else {
                        html! {
                            <>
                                { self.view_app_list() }
                            </>
                        }
                    }
                }




                        // <!-- Modal -->
                        // <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                        //     <div class="modal-dialog modal-dialog-scrollable">
                        //         <div class="modal-content">
                        //         <div class="modal-header">
                        //             <h5 class="modal-title" id="exampleModalLabel">{"Create Application"}</h5>
                        //             <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        //         </div>
                        //         <div class="modal-body" style="font-size: 14px;">
                        //             <div class="mb-4">
                        //             <label for="basic-url" class="form-label fw-bold">{"Name"}</label>
                        //             <div class="input-group mb-2">
                        //                 <input type="text" class="form-control" placeholder="My App" id="basic-url" aria-describedby="basic-addon3" />
                        //             </div>
                        //             <label class="form-label text-muted">{"This is your application name"}</label>
                        //             </div>
                        //             <div class="mb-4">
                                    
                                    

                        //             <div class="MuiFormGroup-root"><label class="MuiFormLabel-root">{"Choose an application type"}</label>
                        //                 <div class="jss2 jss386" data-cosmos-key="column-layout">
                        //                 <div
                        //                     class="MuiPaper-root MuiCard-root jss396 jss397 jss387 jss389 jss394 jss391 MuiPaper-outlined MuiPaper-rounded">
                        //                     <div class="jss2 jss398" data-cosmos-key="row-layout">
                        //                     <div class="jss2 jss399 jss279 jss293 jss288 jss298" data-cosmos-key="avatar"><img
                        //                         data-cosmos-key="image"
                        //                         src="https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/native.svg" class="jss280"/>
                        //                     </div>
                        //                     <h6 class="MuiTypography-root MuiTypography-subtitle2" data-cosmos-key="text">{"Native"}</h6>
                        //                     <p class="MuiTypography-root MuiTypography-body2" data-cosmos-key="text">{"Mobile, desktop, CLI and
                        //                         smart device apps running natively"}</p>
                        //                     <p class="MuiTypography-root MuiTypography-body2 MuiTypography-colorTextSecondary"
                        //                         data-cosmos-key="text">{"e.g.: iOS, Electron, Apple TV apps"}</p>
                        //                     </div>
                        //                 </div>
                        //                 <div
                        //                     class="MuiPaper-root MuiCard-root jss396 jss400 jss387 jss389 jss391 MuiPaper-outlined MuiPaper-rounded">
                        //                     <div class="jss2 jss398" data-cosmos-key="row-layout">
                        //                     <div class="jss2 jss402 jss279 jss293 jss288 jss298" data-cosmos-key="avatar"><img
                        //                         data-cosmos-key="image" src="https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/spa.svg"
                        //                         class="jss280"/></div>
                        //                     <h6 class="MuiTypography-root MuiTypography-subtitle2" data-cosmos-key="text">{"Single Page Web
                        //                         Applications"}</h6>
                        //                     <p class="MuiTypography-root MuiTypography-body2" data-cosmos-key="text">{"A JavaScript front-end
                        //                         app that uses an API."}</p>
                        //                     <p class="MuiTypography-root MuiTypography-body2 MuiTypography-colorTextSecondary"
                        //                         data-cosmos-key="text">{"e.g.: Angular, React, Vue"}</p>
                        //                     </div>
                        //                 </div>
                        //                 <div
                        //                     class="MuiPaper-root MuiCard-root jss396 jss403 jss387 jss389 jss391 MuiPaper-outlined MuiPaper-rounded">
                        //                     <div class="jss2 jss398" data-cosmos-key="row-layout">
                        //                     <div class="jss2 jss405 jss279 jss293 jss288 jss298" data-cosmos-key="avatar"><img
                        //                         data-cosmos-key="image"
                        //                         src="https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/regular_web.svg" class="jss280"/>
                        //                     </div>
                        //                     <h6 class="MuiTypography-root MuiTypography-subtitle2" data-cosmos-key="text">{"Regular Web
                        //                         Applications"}</h6>
                        //                     <p class="MuiTypography-root MuiTypography-body2" data-cosmos-key="text">{"Traditional web app using
                        //                         redirects."}</p>
                        //                     <p class="MuiTypography-root MuiTypography-body2 MuiTypography-colorTextSecondary"
                        //                         data-cosmos-key="text">{"e.g.: Node.js Express, ASP.NET, Java, PHP"}</p>
                        //                     </div>
                        //                 </div>
                        //                 <div
                        //                     class="MuiPaper-root MuiCard-root jss396 jss406 jss387 jss389 jss391 MuiPaper-outlined MuiPaper-rounded">
                        //                     <div class="jss2 jss398" data-cosmos-key="row-layout">
                        //                     <div class="jss2 jss408 jss279 jss293 jss288 jss298" data-cosmos-key="avatar"><img
                        //                         data-cosmos-key="image"
                        //                         src="https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/non_interactive.svg"
                        //                         class="jss280"/></div>
                        //                     <h6 class="MuiTypography-root MuiTypography-subtitle2" data-cosmos-key="text">{"Machine to Machine
                        //                         Applications"}</h6>
                        //                     <p class="MuiTypography-root MuiTypography-body2" data-cosmos-key="text">{"CLIs, daemons or services
                        //                         running on your backend."}</p>
                        //                     <p class="MuiTypography-root MuiTypography-body2 MuiTypography-colorTextSecondary"
                        //                         data-cosmos-key="text">{"e.g.: Shell script"}</p>
                        //                     </div>
                        //                 </div>
                        //                 </div>
                        //             </div>


                        //             <label class="form-label text-muted">{"Ex: Web Application"}</label>
                        //             </div>
                        //         </div>
                        //         <div class="modal-footer">
                        //             <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                        //             <button type="button" class="btn btn-primary">{"Create"}</button>
                        //         </div>
                        //         </div>
                        //     </div>
                        //     </div>

                        </div>


                <div
                    class=format!("modal fade {}", if self.show_modal_create {"show"} else {""})
                    // id="exampleModal"
                    tabindex="-1"
                    // aria-labelledby="exampleModalLabel"
                    // aria-hidden="true"
                    // style=format!("display: {};", if self.show_modal_create {"block; transition: display .15s linear .15s"} else {"none; transition: display .15s linear .15s"})
                    style="display: block;"
                    aria-modal={ if self.show_modal_create {"true"} else {"false"} }
                    role={ if self.show_modal_create {"dialog"} else {""} }
                    aria-hidden={ if self.show_modal_create {"false"} else {"true"} }
                    // onclick=self.link.callback(|_| {Msg::ShowModalCreate(false)})   
                >
                    <div
                        class="modal-dialog modal-dialog-scrollable" 
                    >
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="exampleModalLabel">{"New APP"}</h5>
                                <button
                                    type="button"
                                    class="btn-close"
                                    data-bs-dismiss="modal"
                                    aria-label="Close"
                                    onclick=self.link.callback(|_| {Msg::ShowModalCreate(false)})
                                ></button>
                            </div>
                            <div class="modal-body" style="font-size: 14px;">
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Name"}</label>
                                    <div class="input-group mb-2">
                                        <input
                                            type="text"
                                            class="form-control"
                                            id="basic-url"
                                            aria-describedby="basic-addon3"
                                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, DataAppCreate::Name))
                                        />
                                    </div>
                                    <label class="form-label text-muted">{"This is your Application Name"}</label>
                                </div>

                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Application Type"}</label>
                                    <select
                                        class="form-select mb-2"
                                        aria-label="Default select example"
                                        onchange=self.link.callback(|e| {
                                            if let ChangeData::Select(select) = e {
                                                let value = select.value();
                                                Msg::Input(value, DataAppCreate::AppType)
                                            } else {
                                                Msg::Input(String::from("no value"), DataAppCreate::AppType)
                                            }
                                        })
                                    >
                                        <option value="spa">{"Single Page Application"}</option>
                                        <option value="regular_web">{"Regular Web Application"}</option>
                                        <option value="native">{"Native Appication"}</option>
                                        <option value="non_interactive">{"Non Interactive"}</option>
                                    </select>
                                    <label class="form-label text-muted">{"Choose an application type"}</label>
                                </div>
                            </div>
                            <div class="modal-footer">
                                <button
                                    type="button"
                                    class="btn btn-secondary"
                                    data-bs-dismiss="modal"
                                    onclick=self.link.callback(|_| {Msg::ShowModalCreate(false)})
                                >{"Cancel"}</button>
                                <button
                                    type="button"
                                    class=format!("btn {} btn-primary position-relative", if self.loading_create_app {"loading"} else {""} )
                                    onclick=self.link.callback(|_| Msg::Create)
                                    disabled={ if self.loading_create_app {true} else {false} }
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
                                if self.error_app_create.is_some() {
                                    html! {
                                        <div class="modal-footer">
                                            <div class="alert alert-warning" role="alert">
                                                <i class="bi bi-exclamation-triangle me-2"></i>
                                                { self.error_app_create.clone().unwrap() }
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
                    class=format!("modal-backdrop fade {}", if self.show_modal_create {"show"} else {""})
                />


                    </div>
                </div>
        </>
        }
    }
}




impl ApplicationHome {
    fn view_app_list(&self) -> Vec<Html> {
        type Anchor = RouterAnchor<AppRoute>;
        let tenant_id = &self.tenant_id;
        self.app_list.iter().map(|app| {
            html! {
                <>
                    <div>
                            <div
                                class="d-flex border-bottom border-1 list-hover"
                            >
                            <div
                                class="p-3 d-flex"
                                style="width: 40%;"
                            >
                                <div
                                    style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                                    class="d-flex justify-content-center align-items-center rounded me-3"
                                >
                                    <img
                                        src={"https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg"} style=" color: transparent;
                                        width: 100%;
                                        height: 100%;
                                        object-fit: cover;
                                        text-align: center;
                                        text-indent: 10000px;"
                                    />
                                </div>

                                <div
                                    class="d-grid"
                                    style="min-width: 40px;"
                                >
                                    <Anchor route=AppRoute::ApplicationSettings { tenant_id: tenant_id.clone(), app_id: app.client_id.clone() } >
                                            <a
                                                class="fw-bold mb-0"
                                                style=" white-space: nowrap;
                                                        text-overflow: ellipsis;
                                                        overflow: hidden;
                                                        font-size: 14px;
                                                        text-decoration: none;" 
                                                href="#">
                                                { &app.name }
                                            </a>
                                        </Anchor>
                                    <p
                                        class="mb-0 text-muted"
                                        style=" white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;"
                                    >
                                        { &app.app_type }
                                    </p>
                                </div>
                            </div>

                            <div
                                class="p-3 d-flex flex-fill align-items-center text-muted"
                            >
                                <span
                                    style="font-size: 14px; margin-right: 8px; white-space: nowrap;"
                                >
                                {"Client ID:"}
                                </span>
                                <div
                                    class="rounded"
                                    style=" background-color: #eff0f2;
                                            white-space: nowrap;
                                            text-overflow: ellipsis;
                                            overflow: hidden;
                                            font-size: 14px;
                                            padding: 2px 6px;
                                            font-family: 'Roboto Mono', monospace;"
                                >
                                    { &app.client_id }
                                </div>
                                <i
                                    class="bi bi-files ms-1"
                                >
                                </i>
                            </div>

                            <div
                                class="p-3 d-flex align-items-center dropdown"
                            >
                                <button
                                    type="button"
                                    style="flex: 0 0 auto; width: 30px; height: 30px;"
                                    class="btn d-flex justify-content-center align-items-center rounded border" role="button"
                                    id="dropdownMenuButton1"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    <i
                                        class="bi bi-three-dots"
                                    >
                                    </i>
                                </button>
                                <ul
                                    class="dropdown-menu"
                                    aria-labelledby="dropdownMenuButton1"
                                >
                                    <li>
                                        <Anchor route=AppRoute::ApplicationSettings { tenant_id: tenant_id.clone(), app_id: app.client_id.clone() }classes="dropdown-item fs-7">
                                            {"Settings"}
                                        </Anchor>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </>
            }
        })
        .collect()
    }

    fn view_app_list_empty(&self) -> Html {
        html! {
            <div style="
                display: flex;
                text-align: center;
                align-items: center;
                flex-direction: column;
                margin-top: 60px;
                padding: 40px;
                border-radius: 6px;
                border: 1px solid #e3e4e6;"
            >

                <img width="150" height=""
                    src="https://assets-global.website-files.com/60058af53d79fbd8e14841ea/602e971e34a1e12c00b8c9ab_sso.svg"
                />

                <h4
                    style="padding-top: 20px;"
                >
                    {"You don't have any App yet."}
                </h4>
                <button
                    type="button"
                    class="btn btn-primary mt-3 d-flex align-items-center"
                    // data-bs-toggle="modal"
                    // data-bs-target="#exampleModal"
                    // onclick=self.link.callback(|_| {Msg::ShowModalCreate(true)})
                >
                    <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                    <span>{"Create APP"}</span>
                </button>
            </div>
        }
    }
}

