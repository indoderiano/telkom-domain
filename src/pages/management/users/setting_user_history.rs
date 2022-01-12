use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew::services::ConsoleService;
use crate::components::loading2::Loading2;
use crate::configs::server::API_URL;
use crate::types::users::{UserLogDetails, ResponseUserLogLists};


pub struct UserTabHistory {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    loading_get_user_logs: bool,
    user_log_events: Vec<UserLogDetails>, 
    error_user_log_list: Option<String>,
}


pub enum StateError {
    UserLogList
}

pub enum Msg {
    DefaultState,
    RequestUserLogEvents,
    GetUserLogEvents(Vec<UserLogDetails>),
    ResponseError(String, StateError),
}

impl UserTabHistory {
    fn view_user_log_history(&self) -> Vec<Html> {
        self.user_log_events.iter().map(|user_event|{
            html! {
                <tr>
                    <th scope="row"><svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#13a688" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg></th>
                    <td><a href="">{&user_event.description}</a></td>
                    <td>{"12 minutes ago"}</td>
                    <td>{&user_event.client_name}</td>
                    <td>{&user_event.connection}</td>
                    <td>
                        <p class="m-0">{&user_event.ip}</p>
                        <p class="m-0">
                        {format!("{:?}, {:?}", user_event.location_info.city_name.clone(), user_event.location_info.country_name.clone() )}
                        </p>
                    </td>
                </tr>
            }
        })
        .collect()
    }
}

impl Component for UserTabHistory {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        // let user_log_events = UserLogDetails::new();
        
        UserTabHistory {
            link,
            fetch_task: None,
            loading_get_user_logs: false,
            user_log_events: Vec::new(),
            error_user_log_list: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestUserLogEvents);
        }
    }



    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DefaultState => {
                self.loading_get_user_logs = false;
                true
            }
            Msg::RequestUserLogEvents => {
                ConsoleService::info("ini di request user logevent"); 
                let request = Request::get(format!("{}/users/tenantid/users/:id/logs", API_URL))
                    .header("access_token", "telkomidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<UserLogDetails>, anyhow::Error>>>| {
                        let Json(dataok) = response.into_body();
                        ConsoleService::info(&format!("{:?}", dataok));    
                        match dataok {
                            Ok(dataok) => Msg::GetUserLogEvents(dataok), 
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::UserLogList)
                            }
                        }   
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_user_log_list = None;
                self.loading_get_user_logs = true;
                true
            }
            Msg::GetUserLogEvents(data) => {
                ConsoleService::info("ini di get user log events");
                self.user_log_events = data;
                self.loading_get_user_logs = false;
                self.fetch_task = None;
                true
            }
            Msg::ResponseError(message, state) => {
                ConsoleService::info("ini di info response error ");
                match state {
                    StateError::UserLogList => {
                        self.loading_get_user_logs = false;
                        self.error_user_log_list = Some(message);
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
                    <p>
                        {"Max. Log Storage: 2 days"}
                    </p>
                </div>
                
                <div class="mt-4 table-responsive">
                    <table class="table">
                        <thead>
                            <tr>
                                <th scope="col"></th>
                                <th scope="col">{"Event"}</th>
                                <th scope="col">{"When"}</th>
                                <th scope="col">{"App"}</th>
                                <th scope="col">{"Identity Provider"}</th>
                                <th scope="col">{"From"}</th>
                            </tr>
                        </thead>
                        <tbody>

                            {
                                if !self.loading_get_user_logs && !self.error_user_log_list.is_some() {
                                    html! {
                                        <>
                                            {self.view_user_log_history()}
                                        </>
                                    } 
                                } else {
                                    html! {}
                                }
                            }



                        </tbody>     
                    </table>
                        
                    {
                        if self.loading_get_user_logs {
                            html! {
                                <div style="position: relative; margin-top:4rem;">
                                    <Loading2 width = 45 />
                                </div>
                            }
                        } else if self.error_user_log_list.is_some() {
                            html!{
                                <div class="alert alert-warning mb-5" role="alert">
                                    <i class="bi bi-exclamation-triangle me-2"></i>
                                    { self.error_user_log_list.clone().unwrap() }
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                    
                </div>
                

                <div class="mt-4">
                        <div class="row">
                            <div class="col d-flex justify-content-start">
                                <button type="button" class="btn btn-primary" disabled=true>
                                    <i class="bi bi-arrow-left"></i>
                                    <span>{"NEWER"}</span>
                                </button>
                            </div>
                            <div class="col d-flex justify-content-center">
                                <p>{"Page 1"}</p>
                            </div>
                            <div class="col d-flex justify-content-end">
                                <button type="button" class="btn btn-primary" disabled=true>
                                    <i class="bi bi-arrow-right"></i>
                                    <span>{"OLDER"}</span>
                                </button>
                            </div>
                        </div>
                </div>

            </>
        }
    }
}
