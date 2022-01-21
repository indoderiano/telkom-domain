use yew::prelude::*;
use yew_router::components::RouterAnchor;
use router::AppRoute;

pub struct DatabaseHome {
    learn_more: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    LearnMore,
    HideDetails
}

impl Component for DatabaseHome {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        DatabaseHome {
            learn_more: false,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LearnMore => {
                self.learn_more = true;
                true
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
        type Anchor = RouterAnchor<AppRoute>;
        html! {
            <div
                class="mx-auto pt-5 pb-5 px-4"
                style="max-width: 1048px;"
            >
                <div
                    class="mb-3"
                >
                    <div
                        class="d-flex flex-row mb-3"
                    >
                        <div
                            class="flex-fill fs-3 fw-bold"
                        >
                            {"Database Connections"}
                        </div>

                        <div
            
                        >
                            <Anchor
                                route=AppRoute::DbCreate
                                classes="btn btn-primary d-flex align-items-center"
                                
                            >
                                <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                                <span>{"Create DB Connection"}</span>
                            </Anchor>
                        </div>
                    </div>
                    <p>
                        {"Securely store and manage username / password credentials either in an Auth0 Database or in your own store."}
                        {
                            if self.learn_more == true {
                                html! {
                                    <a
                                        href="javascript: void(0)"
                                        class="text-decoration-none"
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
                                        class="text-decoration-none"
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
                                        {"WITH DATABASE CONNECTIONS YOU CAN"}
                                    </div>
                                    <div
                                        class="d-inline-flex flex-row w-50"
                                    >
                                        <i class="bi bi-info-circle-fill me-4"></i>
                                        <p
                                            class="pe-5"
                                        >
                                            {"Securely store and manage username / password credentials."}
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
                                            {"Let your users signup and login with username and password."}
                                        </p>
                                    </div>
                                    <div
                                        class="d-inline-flex flex-row w-50"
                                    >
                                        <i class="bi bi-info-circle-fill me-4"></i>
                                        <p
                                            class="pe-5"
                                        >
                                            {"Connect to an existing store or database through a Javascript script that runs on Domain server on every authentication."}
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
                                            {"Migrate an existing legacy credentials database to Auth0 gradually as users authenticate. Eliminates the need for users to reset passwords manually when migrating."}
                                        </p>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>

                     <div>

                    <div
                        class="d-flex border-bottom border-1 list-hover"
                    >
                        <div
                            class="p-3 d-flex"
                            style="width: 40%;"
                        >
                            <div
                                style="flex: 0 0 auto; width: 40px; height: 40px;"
                                class="d-flex justify-content-center align-items-center rounded me-3 border"
                            >
                                <i class="bi bi-files"></i>
                            </div>

                            <div
                                class="d-grid"
                                style="min-width: 40px;"
                            >
                                <Anchor
                                    route=AppRoute::DatabaseSettings
                                    classes="text-decoration-none fw-bold mb-0"
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
                                        {"Username-Password-Authentication"}
                                    </span>
                                </Anchor>
                                <p
                                    class="mb-0 text-muted"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                    "
                                >
                                    {"Database"}
                                </p>
                            </div>
            
                        </div>

                        <div
                            class="p-3 d-flex flex-fill align-items-center text-muted"
                        >
                            
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
                                    <Anchor route=AppRoute::DatabaseSettings classes="dropdown-item fs-7">
                                        {"Settings"}
                                    </Anchor>
                                </li>
                            </ul>
                        </div>

                    </div>

                </div>
                
            </div>
        }
    }
}
