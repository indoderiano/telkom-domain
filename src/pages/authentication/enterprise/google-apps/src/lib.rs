use yew::prelude::*;
use yew_router::components::RouterAnchor;
// use crate::app::AppRoute;
use router::AppRoute;

pub struct EnterpriseGoogle {
    // link: ComponentLink<Self>
}

pub enum Msg {}

impl Component for EnterpriseGoogle {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        EnterpriseGoogle {
            // link
        }
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
            <div
                class="py-5 px-4 m-auto"
                style="max-width: 1048px; font-size:14px;"
            >
                <Anchor
                    route=AppRoute::EnterpriseHome
                    classes="text-decoration-none domain-link-dark"
                >
                    <i class="bi bi-arrow-left me-2"></i>
                    {"Back to Enterprise Connections"}
                </Anchor>

                <div
                    class="d-flex mb-4 mt-3"
                >
                    <div
                        class="d-flex flex-column"
                    >
                        <h2>{"Google Workspace"}</h2>
                    </div>
                </div>

                <div style="
                    display: flex;
                    text-align: center;
                    align-items: center;
                    flex-direction: column;
                    padding: 40px;
                    border-radius: 6px;
                    border: 1px solid #e3e4e6;
                    "
                >

                    <i class="bi bi-briefcase text-color-secondary" style="font-size:150px; opacity:.5;"></i>

                    <div>
                        {"No items have been added to this section."}
                    </div>
                    <button
                        class="btn btn-primary"
                        style=" color: #fff;
                                background-color: #635dff;
                                box-shadow: none;
                                border-radius: 4px;
                                padding: 8px 16px;
                                margin: 20px"
                    >
                        <Anchor
                            route=AppRoute::EnterpriseGoogleCreate
                            classes="text-decoration-none text-light px-2 link-primary pe-auto"
                            >
                                {"+ Create Connection"}
                        </Anchor>
                    </button>
                    <a
                        href="https://auth0.com/docs/sso/single-sign-on"
                        target="_blank"
                        style="text-decoration: none;"
                    >{"Learn More"}</a>
                </div>

            </div>
        }
    }
}
