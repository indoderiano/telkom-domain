use yew::{
    format::{
        Json,
    },
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
    events::{ KeyboardEvent }
};
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;
use yewtil::NeqAssign;
use crate::store::reducer_account::{
    AppDispatch,
    DataAccountAction,
    // DataAccount,
};
use crate::configs::server::API_URL;
use crate::types::{
    ResponseLogin,
    LocalStorage,
    LOCALSTORAGE_KEY,
};
use yewdux::dispatch::Dispatcher;
// use crate::app::AppRoute;
use yew_router::service::RouteService;
// use yew_router
use yew::services::{
    ConsoleService,
    storage::{ StorageService, Area },
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestLogin {
    email: String,
    password: String,
}

pub struct LoginPage {
    fetch_task: Option<FetchTask>,
    form_data: RequestLogin,
    error: Option<String>,
    // user: Option<User>,
    link: ComponentLink<Self>,
    dispatch: AppDispatch,
    route_service: RouteService,
}

pub enum Msg {
    Login,
    LoginResponse(Result<ResponseLogin, anyhow::Error>),
    EditEmail(String),
    EditPassword(String),
    KeyPress(String),
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, link: ComponentLink<Self>) -> Self {
        let form_data = RequestLogin {
            email: String::from(""),
            password: String::from(""),
        };
        LoginPage {
            fetch_task: None,
            form_data,
            error: None,
            // user: None,
            link,
            dispatch,
            route_service: RouteService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            Login => {
                // ConsoleService::info(&self.form_data.email);
                let request = Request::post(format!("{}/login", API_URL))
                    .header("Content-Type", "application/json")
                    .body(Json(&self.form_data))
                    // .body(Nothing)
                    .expect("Could not build request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<ResponseLogin, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::LoginResponse(data)
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            LoginResponse(response) => {
                match response {
                    Ok(data) => {
                        ConsoleService::info("response ok");
                        ConsoleService::info(&format!("{:?}", data));
                        // ConsoleService::info(&data.email.clone());
                        // self.user = Some(data.clone());

                        // UPDATE REDUCER
                        let newdata = ResponseLogin {
                            email: String::from(data.email.clone()),
                            username: String::from(data.username.clone()),
                            token: String::from(data.token.clone()),
                        };
                        self.dispatch.send(DataAccountAction::Update(newdata));


                        // SET LOCALSTORAGE
                        let mut storage = StorageService::new(Area::Local).expect("storage was disabled");
                        let user_data = LocalStorage {
                            email: Some(data.email),
                            username: Some(data.username),
                            token: Some(data.token),
                        };
                        let localstorage_data = Json(&user_data);

                        // // let localstorage_data: Result<String, anyhow::Error> = Ok(String::from("tokendata_telkomdomain"));
                        storage.store(LOCALSTORAGE_KEY, localstorage_data);


                        // REDIRECT ROUTE
                        // let router = RouteService::new();
                        // self.route_service.set_route("/apis", ());
                        // router.set_route(AppRoute::ApisHome, );
                        
                        // yew_router::push_route(AppRoute::ApisHome);

                    }
                    Err(error) => {
                        ConsoleService::info("response error");
                        ConsoleService::info(&error.to_string());
                        self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                true
            }
            EditEmail(email) => {
                self.form_data.email = email;
                true
            }
            EditPassword(password) => {
                self.form_data.password = password;
                true
            }
            KeyPress(key) => {
                match key.as_str() {
                    "Enter" => {
                        self.link.send_message(Msg::Login);
                        false
                    }
                    _ => {
                        // do nothing
                        false
                    }
                }
            }
        }
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        let is_fetching:bool = self.fetch_task.is_some();
        let onkeypress = self.link.callback(|e: KeyboardEvent| Msg::KeyPress(e.key()));
        html! {
            <div style="background-color: #dee2e6; min-height: 100vh;">
                <div class="login-page form-signin border">
                    <div class="text-center" style="background-color: white; margin-top: 7vh;">
                        <div class="logo-image">
                            <img src="https://www.telkom.co.id/data/image_upload/page/1594112895830_compress_PNG%20Icon%20Telkom.png" 
                            alt="telAuth"
                            style="width: 80px; height: 80px;"/>
                        </div>  
                        <div>
                            <h1 class="h3 mb-3 fw-normal">{"TelAuth"}</h1>
                            <h1 class="h5 mb-2 fw-normal fs-6">{"Login to TelAuth to continue"}</h1>
                    
                            <div
                                class="form-floating m-auto w-75 d-flex justify-content-center mt-4"
                            >
                                <input
                                    type="email"
                                    class="d-flex form-control"
                                    id={"floatingInput"}
                                    placeholder="name@example.com"
                                    value=self.form_data.email.clone()
                                    oninput=self.link.callback(|data: InputData| Msg::EditEmail(data.value))
                                />
                                <label for="floatingInput">{"Email address"}</label>
                            </div>

                            <div
                                class="form-floating m-auto w-75 d-flex justify-content-center mt-4"
                            >
                                <input
                                    type="password"
                                    class="d-flex form-control"
                                    id={"floatingInput"}
                                    placeholder="password"
                                    value=self.form_data.password.clone()
                                    oninput=self.link.callback(|data: InputData| Msg::EditPassword(data.value))
                                    onkeypress=onkeypress
                                />
                                <label for="floatingInput">{"Password"}</label>
                            </div>
                
                            <button
                                type="button"
                                onclick=self.link.callback(|_| Msg::Login)
                                class="w-75 btn btn-lg btn-primary mt-3 fs-6"
                                // class={
                                //     if is_fetching {

                                //     } else {
                                //         "w-75 btn btn-lg btn-primary mt-3 fs-6"
                                //     }
                                // }
                                disabled={ if is_fetching {true} else {false} }
                            >
                                {
                                    if is_fetching {
                                        html! {
                                            <div class="d-flex justify-content-center">
                                                <div class="spinner-border" role="status">
                                                    <span class="visually-hidden">{"Loading..."}</span>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        html! {
                                            "Continue"
                                        }
                                    }
                                }
                            </button>

                            <button
                                type="button"
                                class="w-75 btn btn-lg btn-secondary mt-3 fs-6"
                            >
                                <Anchor
                                route=AppRoute::RegisterPage
                                classes="text-decoration-none text-light px-2 link-primary pe-auto"
                                >
                                    {"Register"}
                                </Anchor>
                            </button>
                            {
                                if let Some(ref error) = self.error {
                                    html! {
                                        <p class="mt-3 text-danger">
                                            { error.clone() }
                                        </p>
                                    }
                                } else {
                                    html! { }
                                }
                            }


                            
                            // { self.view_user() }
                            
                            <h1 class="h3 mt-3 mb-1 fw-normal" style="font-family: fakt-web, Helvetica Neue, Helvetica, sans-serif;">{"or"}</h1>

                            <button class="w-75 btn btn-lg btn-outline-dark mt-3 fs-6" type="submit">
                                <div class="text-start">
                                    <span>
                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-linkedin" viewBox="0 0 16 16">
                                            <path d="M0 1.146C0 .513.526 0 1.175 0h13.65C15.474 0 16 .513 16 1.146v13.708c0 .633-.526 1.146-1.175 1.146H1.175C.526 16 0 15.487 0 14.854V1.146zm4.943 12.248V6.169H2.542v7.225h2.401zm-1.2-8.212c.837 0 1.358-.554 1.358-1.248-.015-.709-.52-1.248-1.342-1.248-.822 0-1.359.54-1.359 1.248 0 .694.521 1.248 1.327 1.248h.016zm4.908 8.212V9.359c0-.216.016-.432.08-.586.173-.431.568-.878 1.232-.878.869 0 1.216.662 1.216 1.634v3.865h2.401V9.25c0-2.22-1.184-3.252-2.764-3.252-1.274 0-1.845.7-2.165 1.193v.025h-.016a5.54 5.54 0 0 1 .016-.025V6.169h-2.4c.03.678 0 7.225 0 7.225h2.4z"/>
                                        </svg>
                                    </span>
                                    <span class="ms-4">{"Login with LinkedIn"}</span>
                                </div>
                            </button>
            
                            <button class="w-75 btn btn-lg btn-outline-dark mt-3 fs-6" type="submit">
                                <div class="text-start">
                                    <span>
                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-github" viewBox="0 0 16 16">
                                            <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
                                        </svg>
                                    </span>
                                    <span class="ms-4">{"Login with Github"}</span>
                                </div>
                            </button>
            
                            <button class="w-75 btn btn-lg btn-outline-dark mt-3 mb-3 fs-6" type="submit">
                                <div class="text-start">
                                    <span>
                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-google" viewBox="0 0 16 16">
                                            <path d="M15.545 6.558a9.42 9.42 0 0 1 .139 1.626c0 2.434-.87 4.492-2.384 5.885h.002C11.978 15.292 10.158 16 8 16A8 8 0 1 1 8 0a7.689 7.689 0 0 1 5.352 2.082l-2.284 2.284A4.347 4.347 0 0 0 8 3.166c-2.087 0-3.86 1.408-4.492 3.304a4.792 4.792 0 0 0 0 3.063h.003c.635 1.893 2.405 3.301 4.492 3.301 1.078 0 2.004-.276 2.722-.764h-.003a3.702 3.702 0 0 0 1.599-2.431H8v-3.08h7.545z"/>
                                        </svg>
                                    </span>
                                    <span class="ms-4">{"Login with Google"}</span>
                                </div>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

// fn show_error ()