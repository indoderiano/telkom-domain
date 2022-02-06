use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{ FetchService, FetchTask, Request, Response },
        storage::{ Area, StorageService },
        ConsoleService,
    },
};
use yew_router::components::RouterAnchor;
use router::AppRoute;
use types::{
    roles::Role,
    LocalStorage,
    LOCALSTORAGE_KEY,
};
use configs::server::API_URL;
use loading::Loading;


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ProfileProps {
    pub tenant_id: String,
}

pub enum StateError {
    RequestProfile,
}

pub struct Profile {
    access_token: String,
    link: ComponentLink<Self>,
    tenant_id: String,
    fetch_task: Option<FetchTask>,
    profile: String,
    loading_request_profile: bool,
    error_request_profile: Option<String>,
}

pub enum Msg {
    RequestProfile,
    GetProfile(String),
    ResponseError(String, StateError),
}

impl Component for Profile {
    type Message = Msg;
    type Properties = ProfileProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        
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

        let mut access_token = String::from("");

        if let Some(token) = localstorage_data.token {
            access_token = token;
        } else {}
        
        Profile {
            access_token,
            link,
            tenant_id: props.tenant_id,
            fetch_task: None,
            profile: String::from("PROFILE"),
            loading_request_profile: false,
            error_request_profile: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // self.link.send_message(Msg::RequestProfile);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestProfile => {
                // let request = Request::get(format!("{}/api/v2/roles/{}", API_URL, self.role_id.clone()))
                //     .header("access_token", self.access_token.clone())
                //     .body(Nothing)
                //     .expect("Could not build request.");
                // let callback =
                //     self.link
                //         .callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                //             let Json(data) = response.into_body();
                //             match data {
                //                 Ok(dataok) => {
                //                     Msg::GetProfile(dataok)
                //                 }
                //                 Err(error) => Msg::ResponseError(
                //                     error.to_string(),
                //                     StateError::RequestProfile,
                //                 ),
                //             }
                //         });
                // let task = FetchService::fetch(request, callback).expect("failed to start request");
                // self.fetch_task = Some(task);
                // self.error_request_profile = None;
                // self.loading_request_profile = true;
                true
            }
            Msg::GetProfile(data) => {
                self.profile = data;
                self.fetch_task = None;
                self.loading_request_profile = false;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::RequestProfile => {
                        self.loading_request_profile = false;
                        self.error_request_profile = Some(message);
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
        html! {
            <div>
                <div class="domain-content">

                    <div class="mt-2">
                        <p class="fw-bold fs-2">{ "Profile" }</p>
                    </div>

                    {
                        if self.loading_request_profile {
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
                        } else if self.error_request_profile.is_some() {
                            html! {
                                <div class="alert alert-warning mb-5" role="alert">
                                    <i class="bi bi-exclamation-triangle me-2"></i>
                                    { self.error_request_profile.clone().unwrap() }
                                </div>
                            }
                        } else {
                            self.view_content()
                        }
                    }

                </div>
            </div>


        }
    }
}

impl Profile {
    fn view_content(&self) -> Html {
        html! {
            <>
                <div class="card p-3 mt-3">
                    <div class="card-body container">
                        <div class="row">
                            <div
                                style="width: 160px;"
                                class="mb-4"
                            >
                                <div
                                    style="flex: 0 0 auto; width: 128px; height: 128px; background-color: #eff0f2;"
                                    class="d-flex justify-content-center align-items-center rounded-circle"
                                >
                                    <i class="bi bi-person-circle" style="font-size: 45px;"></i>
                                </div>
                            </div>
                            <div class="col-md-9 px-3">
                                <div class="row">
                                    <div class="col-12 col-md-12 col-lg-4 mb-2">
                                        <p class="text-muted mb-1">{"Name"}</p>
                                        <p class="mb-1">{"Indo Halim"}</p>
                                    </div>
                                    <div class="col-12 col-md-12 col-lg-4 mb-2">
                                        <p class="text-muted mb-1 ">{"Email"}</p>
                                        <p class="mb-1">{"mde50526@gmail.com"}</p>
                                    </div>
                                    <div class="col-12 col-md-12 col-lg-4 mb-2">
                                        <p class="text-muted mb-1">{"Nickname"}</p>
                                        <p class="mb-1">{"mde50526"}</p>
                                    </div>
                                </div>
                                <div class="row">
                                    <div class="col-12 col-md-12 col-lg-4 mb-2">
                                        <p class="text-muted mb-1">{"Provider"}</p>
                                        <p class="mb-1">{"Google / Gmail"}</p>
                                    </div>
                                    <div class="col-12 col-md-12 col-lg-4 mb-2">
                                        <p class="text-muted mb-1">{"Default Tenant"}</p>
                                        <select
                                            class="form-select"
                                            // onchange=self.link.callback(|e| {
                                            //     if let ChangeData::Select(select) = e {
                                            //         let value = select.value();
                                            //         Msg::InputText(value, Data::SigningAlg)
                                            //     } else {
                                            //         Msg::InputText("No value".to_string(), Data::SigningAlg)
                                            //     }
                                            // })
                                        >
                                            <option
                                                value="dev-ofzd5p1b"
                                                // selected={if signing_alg == "RS256".to_string() {true} else {false}}
                                            >
                                                {"dev-ofzd5p1b (au)"}
                                            </option>
                                            <option
                                                value="dev-skls3k4d"
                                                // selected={if signing_alg == "HS256".to_string() {true} else {false}}
                                            >
                                                {"dev-skls3k4d (au)"}
                                            </option>
                                        </select>
                                    </div>
                                </div>
                
                            </div>
            
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
