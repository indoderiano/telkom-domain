use yew::prelude::*;
use yew_router::components::RouterAnchor;
// use crate::app::AppRoute;
use router::AppRoute;

pub struct EnterpriseHome {
    learn_more: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    LearnMore,
    HideDetails
}

impl Component for EnterpriseHome {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        EnterpriseHome {
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
                class="domain-content"
            >
                <div
                    class="mb-3"
                >
                    <div
                        class="d-flex flex-row mb-3"
                    >
                        <div
                            class="flex-fill title"
                        >
                            {"Enterprise Connections"}
                        </div>
                    </div>
                    <p>
                        {"Configure Enterprise Connections like AD, SAML, Google Workspace and others so that you can let your users login with them. "}
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
                                        {"With enterprise connection you can"}
                                    </div>
                                    <div
                                        class="d-inline-flex flex-row w-50"
                                    >
                                        <i class="bi bi-info-circle-fill me-4"></i>
                                        <p
                                            class="pe-5"
                                        >
                                            {"Let your users use their enterprise credentials to login to your app."}
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
                                            {"Use Auth0's AD Connector to let Auth0 access your AD or LDAP enterprise IdP securely for validating user credentials."}
                                        </p>
                                    </div>
                                    <div
                                        class="d-inline-flex flex-row w-50"
                                    >
                                        <i class="bi bi-info-circle-fill me-4"></i>
                                        <p
                                            class="pe-5"
                                        >
                                            {"Use Auth0's beautiful Login Box Lock to let your users choose how to authenticate."}
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
                                            {"Implement single sign on in your applications with a flip of a switch."}
                                        </p>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>


                <div class="alert alert-warning mb-5" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    {"This feature is not included in your current plan. Upgrade your Subscription Enterprise connections in production."}
                </div>

                <div>
                    <Anchor
                        route=AppRoute::EnterpriseGoogle
                        classes="d-flex border-bottom border-1 list-hover justify-content-between pe-auto text-decoration-none"
                    >
                        <div
                            class="p-3 d-flex"
                            style="width: 40%;"
                        >
                            <div
                                style="flex: 0 0 auto; width: 40px; height: 40px;"
                                class="d-flex justify-content-center align-items-center rounded me-3 border"
                            >
                                <img
                                    src="/assets/icons/google-avatar.png"
                                    class="w-50"
                                />
                            </div>

                            <div
                                class="d-grid align-items-center"
                                style="min-width: 40px;"
                            >
                                <span
                                    class="fw-bold mb-0"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                    "
                                >
                                    {"Google Workspace"}
                                </span>
                            </div>
            
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
                                <i class="bi bi-plus"></i>
                            </button>
                            <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                                <li>
                                    <Anchor route=AppRoute::EnterpriseGoogle classes="dropdown-item fs-7">
                                        {"Settings"}
                                    </Anchor>
                                </li>
                            </ul>
                        </div>

                    </Anchor>

                    <Anchor
                        route=AppRoute::EnterpriseGoogle
                        classes="d-flex border-bottom border-1 list-hover justify-content-between pe-auto text-decoration-none"
                    >
                        <div
                            class="p-3 d-flex"
                            style="width: 40%;"
                        >
                            <div
                                style="flex: 0 0 auto; width: 40px; height: 40px;"
                                class="d-flex justify-content-center align-items-center rounded me-3 border"
                            >
                                <img
                                    src="/assets/icons/azure-avatar.png"
                                    class="w-50"
                                />
                            </div>

                            <div
                                class="d-grid align-items-center"
                                style="min-width: 40px;"
                            >
                                <span
                                    class="fw-bold mb-0"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                    "
                                >
                                    {"Microsoft Azure AD"}
                                </span>
                            </div>
            
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
                                <i class="bi bi-plus"></i>
                            </button>
                            <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                                <li>
                                    <Anchor route=AppRoute::EnterpriseGoogle classes="dropdown-item fs-7">
                                        {"Settings"}
                                    </Anchor>
                                </li>
                            </ul>
                        </div>

                    </Anchor>

                </div>
                
            </div>
        }
    }
}
