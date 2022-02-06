use yew::prelude::*;
use yew_router::components::RouterAnchor;
// use crate::app::AppRoute;
use router::AppRoute;

pub struct SsoHome {}

pub enum Msg {}

impl Component for SsoHome {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SsoHome {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        html! {
            <>
                <div class="col domain-content">
                    <div>
                        <div class="mx-auto pt-5 pb-5 px-4" style="max-width: 1048px;">
                            <div class="mb-5">
                                <div class="d-flex flex-row mb-3">
                                    <div class="flex-fill fs-3 fw-bold">
                                        {"Single Sign On"}
                                    </div>
                                </div>

                                <div style="
                                        display: flex;
                                        text-align: center;
                                        align-items: center;
                                        flex-direction: column;
                                        margin-top: 60px;
                                        padding: 40px;
                                        border-radius: 6px;
                                        border: 1px solid #e3e4e6;
                                    "
                                >

                                    <img width="150" height=""
                                        src="https://assets-global.website-files.com/60058af53d79fbd8e14841ea/602e971e34a1e12c00b8c9ab_sso.svg"
                                    />
                    
                                    <h4 
                                        style="padding-top: 20px;"
                                    > 
                                        {"You don't have any SSO integrations yet."}
                                    </h4>
                                    <div>
                                        <p>{"SSO enables users to authenticate at Auth0 with one set of credentials to access any number of service provider applications."}</p>
                                    </div>
                                    <button
                                        style=" color: #fff;
                                                background-color: #635dff;
                                                box-shadow: none;
                                                border-radius: 4px;
                                                padding: 8px 16px;
                                                margin: 20px"
                                    >
                                        <Anchor
                                            route=AppRoute::CreateSso
                                            classes="text-decoration-none text-light px-2 link-primary pe-auto"
                                        >
                                            {"+ Create SSO Integration"}
                                        </Anchor>
                                    </button>
                                    <a href="https://auth0.com/docs/sso/single-sign-on" target="_blank">{"Learn More"}</a>
                                </div>
                            </div>
                        </div>
            
                    </div>
                </div>
            </>
        }
    }
}
