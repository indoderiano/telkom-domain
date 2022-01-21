use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_router::components::RouterAnchor;
use yew::services::{
    ConsoleService,
    storage::{ StorageService, Area },
};
use router::AppRoute;
use types::{
    api::{ ApiTitle, ApiCreate },
    ResponseMessage,
    LocalStorage,
    LOCALSTORAGE_KEY,
};
use loading::Loading;
use configs::server::API_URL;


// #[derive(Clone, Debug, Eq, PartialEq, Properties)]
// pub struct ApisProps {
//     pub tenant_id: String,
// }

pub enum StateError {
    ApiList,
    ApiCreate,
}

pub enum DataApiCreate {
    Name,
    Identifier,
    SignAlg,
}

pub struct ApisHome {
    // tenant_id: String,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    loading_get_api: bool,
    api_list: Vec<ApiTitle>,
    error_api_list: Option<String>,
    show_modal_create: bool,
    loading_create_api: bool,
    api_create: ApiCreate,
    error_api_create: Option<String>,
    access_token: String
}

pub enum Msg {
    DefaultState,
    RequestApiList,
    GetApiList(Vec<ApiTitle>),
    Input(String, DataApiCreate),
    ShowModalCreate(bool),
    Create,
    ResponseError(String, StateError),
}

impl Component for ApisHome {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // ConsoleService::info(&format!("Apis home props, tenant id = {}", props.tenant_id));

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
        
        let mut access_token = String::from("");
        if let Some(_) = localstorage_data.token {
            access_token = localstorage_data.token.unwrap();
        } else {

        }

        let api_create = ApiCreate::new();
        ApisHome {
            // tenant_id: props.tenant_id,
            fetch_task: None,
            link,
            loading_get_api: false,
            api_list: Vec::new(),
            error_api_list: None,
            show_modal_create: false,
            loading_create_api: false,
            api_create,
            error_api_create: None,
            access_token,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info("This is first render");
            self.link.send_message(Msg::RequestApiList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DefaultState => {
                self.show_modal_create = false;
                self.loading_get_api = false;
                // self.error_api_list: None,
                self.loading_create_api = false;
                // self.error_api_create: None,
                self.api_create.name = String::from("");
                self.api_create.identifier = String::from("");
                self.api_create.signing_alg = String::from("");
                true
            }
            Msg::RequestApiList => {
                // let request = Request::get(format!("{}/api/v2/resource-servers/tenantid", API_URL))
                let request = Request::get("https://evening-cliffs-55855.herokuapp.com/api/v2/resource-server")
                    // .header("Content-Type", "application/json")
                    .header("access_token", self.access_token.clone())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<ApiTitle>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", &dataok));
                                Msg::GetApiList(dataok)
                            }
                            Err(error) => {
                                ConsoleService::info(&format!("{:?}", &error));
                                Msg::ResponseError(error.to_string(), StateError::ApiList)
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_api_list = None;
                self.loading_get_api = true;
                true
            }
            Msg::GetApiList(data) => {
                self.api_list = data;
                self.loading_get_api = false;
                self.fetch_task = None;
                true
            }
            Msg::Input(input, data) => {
                match data {
                    DataApiCreate::Name => {
                        self.api_create.name = input;
                    }
                    DataApiCreate::Identifier => {
                        self.api_create.identifier = input;
                    }
                    DataApiCreate::SignAlg => {
                        self.api_create.signing_alg = input;
                    }
                }
                true
            }
            Msg::ShowModalCreate(state) => {
                self.show_modal_create = state;
                true
            }
            Msg::Create => {
                ConsoleService::info(&format!("{:?}", self.api_create));
                let request = Request::post("https://evening-cliffs-55855.herokuapp.com/api/v2/resource-server")
                    .header("Content-Type", "application/json")
                    .header("access_token", self.access_token.clone())
                    .body(Json(&self.api_create))
                    .expect("Could not build request.");
                let callback = 
                    self.link.batch_callback(|response: Response<Json<Result<ApiTitle, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("ini response berhasil{:?}", dataok));
                                vec![Msg::DefaultState, Msg::RequestApiList]
                            }
                            Err(error) => {
                                ConsoleService::info(&error.to_string());
                                vec![Msg::ResponseError(error.to_string(), StateError::ApiCreate)]
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_create_api = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::ApiList => {
                        self.loading_get_api = false;
                        self.error_api_list = Some(message);
                    }
                    StateError::ApiCreate => {
                        self.loading_create_api = false;
                        self.error_api_create = Some(message);
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
        // type Anchor = RouterAnchor<AppRoute>;
        // ConsoleService::info(&format!("fetch task: {:?}", &self.fetch_task));
        html! {
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
                            {"APIs"}
                        </div>
                        <div>
                            <button
                                type="button"
                                class="btn btn-primary d-flex align-items-center"
                                // data-bs-toggle="modal"
                                // data-bs-target="#exampleModal"
                                onclick=self.link.callback(|_| {Msg::ShowModalCreate(true)})
                            >
                                <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                                <span>{"Create API"}</span>
                            </button>
                        </div>
                    </div>
                    <p>{"Define APIs that you can consume from your authorized applications."}</p>
                </div>


                {
                    if self.loading_get_api {
                        html! {
                            <div
                                style="
                                    position: relative;
                                    margin-top: 8rem;
                                "
                            >
                                <Loading width=45 />
                            </div>
                        }
                    } else if self.error_api_list.is_some() {
                        html! {
                            <div class="alert alert-warning mb-5" role="alert">
                                <i class="bi bi-exclamation-triangle me-2"></i>
                                { self.error_api_list.clone().unwrap() }
                            </div>
                        }
                    } else if self.api_list.len() == 0 {
                        html! {
                            <>
                                { self.view_api_list_empty() }
                            </>
                        }
                    } else {
                        html! {
                            <>
                                { self.view_api_list() }
                            </>
                        }
                    }
                }



                // MODAL CREATE APIs
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
                                <h5 class="modal-title" id="exampleModalLabel">{"New API"}</h5>
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
                                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, DataApiCreate::Name))
                                        />
                                    </div>
                                    <label class="form-label text-muted">{"A friendly name for the API"}</label>
                                </div>
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Identifier"}</label>
                                    <div class="input-group mb-2">
                                        <input
                                            type="text"
                                            class="form-control"
                                            id="basic-url"
                                            aria-describedby="basic-addon3"
                                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, DataApiCreate::Identifier))
                                        />
                                    </div>
                                    <label class="form-label text-muted">{"A logical identifier for this API. We recommend using a URL but note that this doesn’t have to be a publicly available URL, Auth0 will not call your API at all.This field cannot be modified."}</label>
                                </div>
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Signing Algorithm"}</label>
                                    <select
                                        class="form-select mb-2"
                                        aria-label="Default select example"
                                        onchange=self.link.callback(|e| {
                                            if let ChangeData::Select(select) = e {
                                                let value = select.value();
                                                Msg::Input(value, DataApiCreate::SignAlg)
                                            } else {
                                                Msg::Input(String::from("no value"), DataApiCreate::SignAlg)
                                            }
                                        })
                                    >
                                        <option value="RS256">{"RS256"}</option>
                                        <option value="HS256">{"HS256"}</option>
                                    </select>
                                    <label class="form-label text-muted">{"Algorithm to sign the tokens with. When selecting RS256 the token will be signed with Auth0's private key."}</label>
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
                                    class=format!("btn {} btn-primary position-relative", if self.loading_create_api {"loading"} else {""} )
                                    onclick=self.link.callback(|_| Msg::Create)
                                    disabled={ self.loading_create_api }
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
                                if self.error_api_create.is_some() {
                                    html! {
                                        <div class="modal-footer">
                                            <div class="alert alert-warning" role="alert">
                                                <i class="bi bi-exclamation-triangle me-2"></i>
                                                { self.error_api_create.clone().unwrap() }
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
                
        }
    }
}


impl ApisHome {
    fn view_api_list (&self) -> Vec<Html> {
        type Anchor = RouterAnchor<AppRoute>;
        // let tenant_id = &self.tenant_id;
        self.api_list.iter().map(|api| {
            html! {
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
                                <i class="bi bi-gear"></i>
                            </div>

                            <div
                                class="d-grid"
                                style="min-width: 40px;"
                            >
                                <p
                                    class="m-0"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                        text-decoration: none;
                                    "
                                >
                                    <Anchor
                                        route=AppRoute::ApisSettings { resource_server_id: api.resource_server_id.clone() }
                                        classes="text-decoration-none fw-bold mb-0"
                                    >
                                            // {"Auth0 Management API"}
                                            { &api.name }
                                    </Anchor>
                                </p>
                                <p
                                    class="mb-0 text-muted"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                    "
                                >
                                    {
                                        if api.is_system {"System API"} else {"Custom Api"}
                                    }
                                </p>
                            </div>
            
                        </div>

                        <div
                            class="p-3 d-flex flex-fill align-items-center text-muted"
                        >
                            <span
                                style="font-size: 14px; margin-right: 8px; white-space: nowrap;"
                            >
                                {"API Audience:"} 
                            </span>
                            <div
                                class="rounded"
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
                                // {"https://dev-r5y8heyf.au.auth0.com/api/v2/"}
                                { &api.identifier }
                            </div>
                            <i class="bi bi-files ms-1"></i>
                        </div>

                        <div
                            class="p-3 d-flex align-items-center dropdown"
                        >
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
                                    <Anchor route=AppRoute::ApisSettings { resource_server_id: api.resource_server_id.clone() } classes="dropdown-item fs-7">
                                        {"Settings"}
                                    </Anchor>
                                </li>
                            </ul>
                        </div>

                    </div>
                </div>
            }
        })
        .collect()
    }

    fn view_api_list_empty (&self) -> Html {
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
                    {"You don't have any SSO integrations yet."}
                </h4>
                <button
                    type="button"
                    class="btn btn-primary mt-3 d-flex align-items-center"
                    // data-bs-toggle="modal"
                    // data-bs-target="#exampleModal"
                    onclick=self.link.callback(|_| {Msg::ShowModalCreate(true)})
                >
                    <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                    <span>{"Create API"}</span>
                </button>
            </div>
        }
    }
}