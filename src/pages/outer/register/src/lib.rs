use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use serde::{
    Deserialize,
    Serialize,
};
use configs::server::API_URL;
use types::ResponseMessage;


pub enum RegisterData {
    Email,
    Password,
    Name,
    AccountType,
    Region,
    TenantName,
}

#[derive(Serialize, Debug, Clone)]
struct RegisterInput {
    email: String,
    password: String,
    name: String,
    account_type: String,
    region: String,
    tenant_name: String,
}

pub struct RegisterPage {
    register_input: RegisterInput,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    loading_register: bool,
}

pub enum Msg {
    Register,
    Input(String, RegisterData),
    RegisterResponse(Result<ResponseMessage, anyhow::Error>),
}

impl Component for RegisterPage {
    type Message = Msg;
    type Properties = ();

    fn create( _: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        let register_input = RegisterInput {
            email: "".into(),
            password: "".into(),
            name: "".into(),
            account_type: "".into(),
            region: "".into(),
            tenant_name: "".into(),
        };
        
        RegisterPage {
            register_input,
            link,
            fetch_task: None,
            loading_register: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Register => {
                ConsoleService::info("register");
                ConsoleService::info(&format!("data register is {:?}", self.register_input.clone()));
                // ConsoleService::info(&format!("email is {}", self.email.clone()));
                // ConsoleService::info(&format!("password is {}", self.password.clone()));


                // let request = Request::post(format!("{}/register", API_URL))
                //     .header("Content-Type", "application/json")
                //     .body(Json(&self.register_input))
                //     .expect("Could not build request.");
                // let callback = self.link.callback(
                //     |response: Response<Json<Result<Vec<String>, anyhow::Error>>>| {
                //         let Json(data) = response.into_body();
                //         // ConsoleService::info(&format!("{:?}", &data.unwrap()));
                //         // Msg::GetAppList(data.unwrap())
                //         match data {
                //             Ok(dataok) => {
                //                 ConsoleService::info(&format!("{:?}", &dataok));
                //                 // Msg::GetAppList(dataok)
                //             } 
                //             Err(error) => {
                //                 ConsoleService::info(&format!("error = {}", error.to_string()));
                //                 // Msg::ResponseError(error.to_string(), StateError::AppList)
                //             }
                //         }
                //     },
                // );
                // let task = FetchService::fetch(request, callback).expect("failed to start request");
                // self.fetch_task = Some(task);

                let request = Request::post(format!("{}/register", API_URL))
                    .header("Content-Type", "application/json")
                    .body(Json(&self.register_input))
                    .expect("Could not build request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<ResponseMessage, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::RegisterResponse(data)
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.loading_register = true;

                true
            }
            Msg::Input(input, data) => {
                match data {
                    RegisterData::Email => {
                        self.register_input.email = input;
                    }
                    RegisterData::Password => {
                        self.register_input.password = input;
                    }
                    RegisterData::Name => {
                        self.register_input.name = input;
                    }
                    RegisterData::AccountType => {
                        self.register_input.account_type = input;
                    }
                    RegisterData::Region => {
                        self.register_input.region = input;
                    }
                    RegisterData::TenantName => {
                        self.register_input.tenant_name = input;
                    }
                }
                true
            }
            Msg::RegisterResponse(response) => {
                // ConsoleService::info(&format!("{}", message));
                match response {
                    Ok(data) => {
                        ConsoleService::info(&format!("response = {:?}", data));
                    }
                    Err(error) => {
                        ConsoleService::info(&format!("error = {}", error.to_string()));
                    }
                }
                self.loading_register = false;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <div style="background-color: #dee2e6; min-height: 100vh;">
            <div class="login-page form-signin border">
                <div class="text-center align-middle" style="background-color: white; margin-top: 7vh">
                <div class="logo-image">
                    <img src="https://www.telkom.co.id/data/image_upload/page/1594112895830_compress_PNG%20Icon%20Telkom.png" 
                    alt="telAuth"
                    style="width: 80px; height: 80px;"/>
                </div>  
                <div
                    // onsubmit=self.link.callback(|_| {Msg::Register})
                >
                    <h1 class="h3 mb-3 fw-normal">{"TelAuth"}</h1>
                    <h1 class="h5 mb-2 fw-normal fs-6">{"Sign Up to TelAuth"}</h1>
            
                    <div class="form-floating m-auto w-75 d-flex justify-content-center mt-4">
                        <input
                            type="email"
                            class="d-flex form-control"
                            id={"floatingInput"}
                            placeholder="name@example.com"
                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, RegisterData::Email))
                            value={self.register_input.email.clone()}
                        />
                        <label for="floatingInput">{"Email Address"}</label>
                    </div>
                    
                    <div class="form-floating m-auto w-75 d-flex justify-content-center mt-4">
                        <input
                            type="username"
                            class="d-flex form-control"
                            id={"floatingInput"}
                            placeholder="ex: John doe"
                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, RegisterData::Name))
                            value={self.register_input.name.clone()}
                        />
                        <label for="floatingInput">{"Username"}</label>
                    </div>

                    <div class="form-floating m-auto w-75 d-flex justify-content-center mt-4">
                        <input
                            type="password"
                            class="d-flex form-control"
                            id={"floatingInput"}
                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, RegisterData::Password))
                            value={self.register_input.password.clone()}
                        />
                        <label for="floatingInput">{"Password"}</label>
                    </div>

                    <div class="form-floating m-auto w-75 d-flex justify-content-center mt-4">
                        <input
                            class="d-flex form-control"
                            id={"floatingInput"}
                            placeholder="Personal"
                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, RegisterData::AccountType))
                            value={self.register_input.account_type.clone()}
                        />
                        <label for="floatingInput">{"Account Type"}</label>
                    </div>

                    <div class="form-floating m-auto w-75 d-flex justify-content-center mt-4">
                        <input
                            class="d-flex form-control"
                            id={"floatingInput"}
                            placeholder="Asia"
                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, RegisterData::Region))
                            value={self.register_input.region.clone()}
                        />
                        <label for="floatingInput">{"Region"}</label>
                    </div>

                    <div class="form-floating m-auto w-75 d-flex justify-content-center mt-4">
                        <input
                            class="d-flex form-control"
                            id={"floatingInput"}
                            placeholder="Your Company"
                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, RegisterData::TenantName))
                            value={self.register_input.tenant_name.clone()}
                        />
                        <label for="floatingInput">{"Tenant Name"}</label>
                    </div>

                    <div class="mt-3">
                        <h1 class="h5 mb-2 fw-normal fs-6">{"By clicking any of the Sign Up buttons,"}</h1>
                        <h1 class="h5 mb-2 fw-normal fs-6">{"I agree to the"} <a class="ms-2" href="">{"terms of service"}</a> </h1>
                    </div>

                    <button
                        type="button"
                        class=format!("btn btn-lg btn-primary w-75 mt-3 fs-6 position-relative {}", if self.loading_register {"loading"} else {""} )
                        onclick=self.link.callback(|_| Msg::Register)
                        disabled={ if self.loading_register {true} else {false} }
                    >
                        <div class="telkom-label">
                            {"Sign Up"}
                        </div>
                        <div class="telkom-spinner telkom-center">
                            <div class="spinner-border spinner-border-sm" role="status"/>
                        </div>
                    </button>
        
                    // <button
                    //     class="w-75 btn btn-lg btn-primary mt-3 fs-6"
                    //     type="submit"
                    //     onclick=self.link.callback(|_| {Msg::Register})
                    // >
                    //     {"Sign Up"}
                    // </button>



                    // <h1 class="h5 mt-3 mb-1 fw-normal" style="font-family: fakt-web, Helvetica Neue, Helvetica, sans-serif;">{"or"}</h1>
                    // <button class="w-75 btn btn-lg btn-outline-dark mt-3 fs-6" type="submit">
                    //     <div class="text-start">
                    //         <span>
                    //             <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-linkedin" viewBox="0 0 16 16">
                    //                 <path d="M0 1.146C0 .513.526 0 1.175 0h13.65C15.474 0 16 .513 16 1.146v13.708c0 .633-.526 1.146-1.175 1.146H1.175C.526 16 0 15.487 0 14.854V1.146zm4.943 12.248V6.169H2.542v7.225h2.401zm-1.2-8.212c.837 0 1.358-.554 1.358-1.248-.015-.709-.52-1.248-1.342-1.248-.822 0-1.359.54-1.359 1.248 0 .694.521 1.248 1.327 1.248h.016zm4.908 8.212V9.359c0-.216.016-.432.08-.586.173-.431.568-.878 1.232-.878.869 0 1.216.662 1.216 1.634v3.865h2.401V9.25c0-2.22-1.184-3.252-2.764-3.252-1.274 0-1.845.7-2.165 1.193v.025h-.016a5.54 5.54 0 0 1 .016-.025V6.169h-2.4c.03.678 0 7.225 0 7.225h2.4z"/>
                    //             </svg>
                    //         </span>
                    //         <span class="ms-4">{"Sign Up with LinkedIn"}</span>
                    //     </div>
                    // </button>
        
                    // <button class="w-75 btn btn-lg btn-outline-dark mt-3 fs-6" type="submit">
                    //     <div class="text-start">
                    //         <span>
                    //             <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-github" viewBox="0 0 16 16">
                    //                 <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
                    //             </svg>
                    //         </span>
                    //         <span class="ms-4">{"Sign Up with Github"}</span>
                    //     </div>
                    // </button>
        
                    // <button class="w-75 btn btn-lg btn-outline-dark mt-3 mb-3 fs-6" type="submit">
                    //     <div class="text-start">
                    //         <span>
                    //             <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-google" viewBox="0 0 16 16">
                    //                 <path d="M15.545 6.558a9.42 9.42 0 0 1 .139 1.626c0 2.434-.87 4.492-2.384 5.885h.002C11.978 15.292 10.158 16 8 16A8 8 0 1 1 8 0a7.689 7.689 0 0 1 5.352 2.082l-2.284 2.284A4.347 4.347 0 0 0 8 3.166c-2.087 0-3.86 1.408-4.492 3.304a4.792 4.792 0 0 0 0 3.063h.003c.635 1.893 2.405 3.301 4.492 3.301 1.078 0 2.004-.276 2.722-.764h-.003a3.702 3.702 0 0 0 1.599-2.431H8v-3.08h7.545z"/>
                    //             </svg>
                    //         </span>
                    //         <span class="ms-4">{"Sign Up with Google"}</span>
                    //     </div>
                    // </button>
                </div>
            </div>
        </div>
        </div> 
        }
    }
}
