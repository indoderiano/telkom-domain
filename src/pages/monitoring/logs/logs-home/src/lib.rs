use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
        storage::{ StorageService, Area }
    },
};
use yew_router::components::RouterAnchor;
use router::AppRoute;
use types::{
    users::{ResponseUsersList, UserCreate, UserTitle},
    ResponseMessage,
    LocalStorage,
    LOCALSTORAGE_KEY,
};

use loading::Loading;
use developers_note::DevelopersNote;
use configs::server::API_URL;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct UserProps {
    pub tenant_id: String,
}


pub struct LogsHome {
    tenant_id: String,
    access_token: String,
    fetch_task: Option<FetchTask>,
    learn_more: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    LearnMore,
    InputDate(String),
    HideDetails,
}

impl Component for LogsHome {
    type Message = Msg;
    type Properties = UserProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // ConsoleService::info(&format!("User home props, tenant id= {}", props.tenant_id));

        
        // LOCALSTORAGE RESOURCE
        // https://github.com/yewstack/yew/issues/1287
        // GET LOCALSTORAGE
        // NEED BETTER WAY TO PARSE JSON DATA
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


        let user_create = UserCreate::new();

        LogsHome {
            tenant_id: props.tenant_id,
            access_token,
            fetch_task: None,
            learn_more: false,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // self.link.send_message(Msg::RequestUserList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LearnMore => {
                self.learn_more = true;
                true
            }
            Msg::InputDate(value) => {
                ConsoleService::info(&format!("date is {}", value));
                false
            }
            Msg::HideDetails => {
                self.learn_more = false;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let tenant_id = self.tenant_id.clone();
        type Anchor = RouterAnchor<AppRoute>;
        html! {
            <div>
                <div class="container domain-content">

                    <div class=" d-flex row align-center">
                        <div class="col">
                            <h2 class="title">{"Logs"}</h2>
                        </div>
                        <div class="col d-flex justify-content-end">
                            <button
                                type="button"
                                class="btn btn-primary ms-3 mt-3 mb-3"
                                data-bs-toggle="modal"
                                data-bs-target="#createNewUser"
                            >

                                <i class="bi bi-plus"></i>
                                <span>{"Create User"}</span>
                            </button>
                        </div>
                    </div>

                    <div class="mt-3">
                        <p>
                            {"Storage of log data of both actions taken in the dashboard by the administrators, as well as authentications made by your users."}
                            {
                                if self.learn_more == true {
                                    html!{
                                        <a
                                            href="javascript: void(0);"
                                            class="text-decoration-none ms-1"
                                            onclick=self.link.callback(|_| Msg::HideDetails)
                                        >
                                            <span
                                                style="
                                                    white-space: nowrap;
                                                    text-overflow: ellipsis;
                                                    overflow: hidden;
                                                    font-size: 14px;
                                                    text-decoration: none;
                                                "
                                            >
                                                {"Hide details"}
                                                <i
                                                class="bi bi-arrow-right-short fs-5"
                                                style="vertical-align: -3px; margin-left: -2px;"></i>
                                            </span>
                                        </a>
                                    }
                                } else {
                                    html! {
                                        <a
                                            href="javascript: void(0)"
                                            class="text-decoration-none ms-1"
                                            onclick=self.link.callback(|_| Msg::LearnMore)
                                        >
                                            <span
                                                style="
                                                    white-space: nowrap;
                                                    text-overflow: ellipsis;
                                                    overflow: hidden;
                                                    font-size: 14px;
                                                    text-decoration: none;
                                                "
                                            >
                                                {"Learn more"}
                                                <i
                                                    class="bi bi-arrow-right-short fs-5"
                                                    style="vertical-align: -3px; margin-left: -2px;"></i>
                                            </span>
                                        </a>
                                    }
                                }
                            }
                        </p>
                        {
                            if self.learn_more == true {
                                html! {
                                    <div
                                        class="alert alert-secondary"
                                        role="alert"
                                        style="font-size: 13px;"
                                    >
                                        <div
                                            class="fw-bold mb-3 pb-2"
                                            style="
                                                font-size: 13px;
                                                text-transform: uppercase;
                                                letter-spacing: 1px;
                                                border-bottom: 1px solid rgb(200, 200, 200);
                                            "
                                        >
                                            {"With logs you can"}
                                        </div>
                                        <div
                                            class="d-inline-flex flex-row w-50"
                                        >
                                            <i class="bi bi-info-circle-fill me-4"></i>
                                            <p
                                                class="pe-5"
                                            >
                                                {"Review the logged data of both actions taken in the dashboard by the administrators, as well as authentications made by your users."}
                                            </p>
                                        </div>
                                        <div
                                            class="d-inline-flex flex-row"
                                            style="width: 49%;"
                                        >
                                            <i class="bi bi-info-circle-fill me-4"></i>
                                            <p
                                                class="pe-5"
                                            >
                                                {"Facilitate diagnosis and resolution of authentication issues."}
                                            </p>
                                        </div>
                                        <div
                                            class="d-inline-flex flex-row w-50"
                                        >
                                            <i class="bi bi-info-circle-fill me-4"></i>
                                            <p
                                                class="pe-5"
                                            >
                                                {"Longer Storage of log data for your apps."}
                                            </p>
                                        </div>
                                        <div
                                            class="d-inline-flex flex-row"
                                            style="width: 49%;"
                                        >
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>


                    <div class="mt-5 mb-3">
                        // <DevelopersNote message="Feature Search and Filter are not yet implemented"/>
                        <div class="row" style="opacity: .7;">
                            <div class="col-md col-lg">
                                <div class="input-group flex-nowrap">
                                    <span class="input-group-text" id="addon-wrapping"><i class="bi bi-search"></i></span>
                                    <input
                                        type="text"
                                        class="form-control"
                                        placeholder="Ex: type:'s' AND date:[2019-05-22 TO *]"
                                        aria-label="Username"
                                        aria-describedby="addon-wrapping"
                                    />
                                </div>
                            </div>
                            <div class="col-md-auto col-lg-auto">
                                <div class="input-group mb-3">
                                    <label class="input-group-text" for="inputGroupSelect01">{"Filter"}</label>
                                    <select class="form-select" id="inputGroupSelect01">
                                    <option selected=true>{"User"}</option>
                                    <option value="1">{"Email"}</option>
                                    <option value="2">{"Identity Provider"}</option>
                                    <option value="3">{"Connection"}</option>
                                    <option value="3">{"Connection"}</option>
                                    <option value="3">{"Login Count"}</option>
                                    <option value="3">{"Last Login"}</option>
                                    <option value="3">{"Phone Number"}</option>
                                    <option value="lucene_syntax">{"Lucene Syntax ()"}</option>
                                    </select>
                                </div>
                            </div>
                            <div class="col-md-auto col-lg-auto">
                                // <button type="button" class="btn btn-outline-secondary">
                                //     <i class="bi bi-calendar me-2"></i>
                                //     <span>{"All Dates"}</span>
                                // </button>
                                <input
                                    class="form-control"
                                    type="date"
                                    oninput=self.link.callback(|data: InputData| {
                                        Msg::InputDate(data.value)
                                    })
                                />
                            </div>
                        </div>

                        <div>
                            {"To perform your search, press"}
                            <code
                                style="padding: 2px 6px; font-size: 11px;"
                                class="bg-input-grey text-color-primary rounded ms-2"
                            >
                                {"enter"}
                            </code>
                        </div>
                    </div>

                    // <DevelopersNote message="Error handling (when token is expired) is not yet implemented"/>
                    {
                        if false {
                        // if true {
                            html! {
                                <div
                                    // class="d-flex align-items-center justify-content-center"
                                    style="margin-top: 4rem;"
                                >
                                    <Loading width=45 />
                                </div>
                            }
                        } else if false {
                            html! {
                                <tr>
                                    <div class="alert alert-warning mb-5" role="alert">
                                        <i class="bi bi-exclamation-triangle me-2"></i>
                                        { "Error message" }
                                    </div>
                                </tr>
                            }
                        } else {
                            html! {
                                <>
                                    <div class="mt-2 table-responsive-md table-responsive-lg">
                                        <table class="table">
                                            <thead>
                                                <tr>
                                                    <th scope="col-auto">{""}</th>
                                                    <th scope="col-auto">{"Type"}</th>
                                                    <th scope="col-auto">{"Description"}</th>
                                                    <th scope="col-auto">{"Date"}</th>
                                                    <th scope="col-auto">{"Connection"}</th>
                                                    <th scope="col-auto">{"Application"}</th>
                                                </tr>
                                            </thead>
                                        
                                            <tbody>
                                                {self.view_user_list()}
                                            </tbody>
                                        </table>
                                    </div>
                                </>
                            }
                        }
                    }

                </div>

            </div>
        }
    }
}


impl LogsHome {
    fn view_user_list(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        let tenant_id = self.tenant_id.clone();

        // self.user_list.iter().map(|user| {
            html! {
                <tr
                    class="align-middle"
                >
                    <td>
                        <i class="bi bi-check2-circle text-success"></i>
                    </td>
                    <td scope="row">
                        <div
                            class="pt-2 pb-2"
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
                                    route=AppRoute::LogDetails {tenant_id: tenant_id.clone(), log_id: String::from("log-id") }
                                    classes="text-decoration-none fw-bold mb-0"
                                >
                                    { "API Read Operation" }
                                </Anchor>

                            </p>
                            // <p class="text-muted overflow-hidden m-0">{"API Read Operation"}</p>
                        </div>
                    </td>
                    <td>{"Get a client"}</td>
                    <td>{"2 hours ago"}</td>
                    <td>{"N/A"}</td>
                    <td>
                        {"N/A"}
                        // <button type="button" style="flex: 0 0 auto; width: 30px; height: 30px;" class="btn d-flex justify-content-center align-items-center rounded border" role="button" id="dropdownMenuButton1" data-bs-toggle="dropdown" aria-expanded="false">
                        //     <i class="bi bi-three-dots"></i>
                        // </button>
                        // <ul class="dropdown-menu pt-1" aria-labelledby="dropdownMenuButton1">
                        //     <li class="p-1" style="font-size: 13px;">
                        //         <DevelopersNote message="Not yet implemented"/>
                        //     </li>
                        //     <li class="p-1 text-muted" style="font-size:13px;">
                        //         <Anchor route=AppRoute::UserSettings {tenant_id: tenant_id.clone(), user_id: user.user_id.clone(), id:1 } classes="dropdown-item">
                        //             {"View Details"}
                        //         </Anchor>
                        //     </li>
                        //     <li>
                        //         <hr class="dropdown-divider"/>
                        //     </li>
                        //     <li class="p-1 text-muted">
                        //                 <div class="ms-1 d-flex flex-row inline-block align-items-center" style="font-size:13px;" >
                        //                     <i class="bi bi-person-check"></i>
                        //                     <span data-bs-toggle="modal" data-bs-target="#assignRoles">
                        //                     <a class="dropdown-item" href="#">
                        //                         {"Assign Roles"}
                        //                     </a>
                        //                 </span>
                        //                 </div>
                        //     </li>
                        //     <li class="p-1 text-muted" style="font-size:13px;">
                        //                 <div class="ms-1 d-flex flex-row inline-block align-items-center">
                        //                     <i class="bi bi-check2-square"></i>
                        //                     <span data-bs-toggle="modal" data-bs-target="#assignPermissions">
                        //                         <a class="dropdown-item" href="#" >
                        //                             {"Assign Permissions"}
                        //                         </a>
                        //                     </span>
                        //                 </div>
                        //     </li>
                        //     <li class="p-1 text-muted" style="font-size:13px;">
                        //         <div class="ms-1 d-flex flex-row inline-block align-items-center">
                        //             <i class="bi bi-envelope "></i>
                        //             <span  data-bs-toggle="modal" data-bs-target="#resendConfirmation">
                        //                 <a class="dropdown-item" href="#">
                        //                     {"Send Verification Email "}
                        //                 </a>
                        //             </span>
                        //         </div>
                        //     </li>
                        //     <li>
                        //         <hr class="dropdown-divider"/>
                        //     </li>
                        //     <li class="p-1 text-muted" style="font-size:13px;" data-bs-toggle="modal" data-bs-target="#changeEmail">
                        //         <a class="dropdown-item" href="#" >
                        //             {"Change Email "}
                        //         </a>
                        //     </li>
                        //     <li class="p-1 text-muted" style="font-size:13px;" data-bs-toggle="modal" data-bs-target="#changePassword">
                        //         <a class="dropdown-item" href="#">
                        //             {"Change Password "}
                        //         </a>
                        //     </li>
                        //     <li>
                        //         <hr class="dropdown-divider" />
                        //     </li>
                        //     <li class="p-1" style="font-size:13px;">
                        //         <div class="ms-1 d-flex flex-row text-muted inline-block align-items-center">
                        //             <svg xmlns="http://www.w3.org/2000/svg " width="13" height="13" viewBox="0 0 24 24 " fill="none " stroke="currentColor " stroke-width="2 " stroke-linecap="round " stroke-linejoin="round"><circle cx="12 " cy="12 " r="10 "></circle><line x1="4.93 " y1="4.93 " x2="19.07 " y2="19.07 "></line></svg>
                        //             <span>
                        //                 <a class="dropdown-item" href="#">
                        //                     {"Block "}
                        //                 </a>
                        //             </span>
                        //         </div>
                        //     </li>
                        //     <li class="p-1 text-danger " style="font-size:13px;">
                        //         <div class="ms-1 d-flex flex-row align-items-center">
                        //             <i class="bi bi-trash "></i>
                        //             <span data-bs-toggle="modal" data-bs-target="#deleteUsers">
                        //                 <a class="dropdown-item fs-7" href="#">
                        //                     {"Delete "}
                        //                 </a>
                        //             </span>
                        //         </div>
                        //     </li>
                        // </ul>
                    </td>
                </tr>
            }
        // })
        // .collect()
    }
}