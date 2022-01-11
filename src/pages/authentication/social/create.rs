use yew::prelude::*;
use yew_router::components::RouterAnchor;
// use crate::app::AppRoute;
use router::AppRoute;


pub struct SocialCreate {
    // link: ComponentLink<Self>
}

pub enum Msg {}

impl Component for SocialCreate {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SocialCreate {
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
                    route=AppRoute::SocialHome
                    classes="text-decoration-none domain-link-dark"
                >
                    <i class="bi bi-arrow-left me-2"></i>
                    {"Social Connections"}
                </Anchor>

                <div
                    class="d-flex mb-5 mt-3"
                >

                    <div
                        class="d-flex flex-column"
                    >
                        <h2>{"New Social Connection"}</h2>
                        
                    </div>
                </div>
                
                <div class="input-group mb-5">
                    <span class="input-group-text"><i class="bi bi-search"></i></span>
                    <input
                        type="text"
                        class="form-control"
                        style="font-size: 14px;"
                        placeholder="Search for social connection"
                        aria-label="Username"
                        aria-describedby="basic-addon1"
                    />
                </div>

                <div class="row">


                    <div class="col-sm-6 mb-4">
                        <div class="card card-hover" style="cursor: pointer;">
                            <div class="card-body p-4">

                                <div
                                    class="d-flex mb-3"
                                >
                                    <div
                                        style="flex: 0 0 auto; width: 40px; height: 40px;"
                                        class="d-flex justify-content-center align-items-center me-3"
                                    >
                                        <img
                                            src="/assets/icons/google-avatar.png"
                                            class="w-75"
                                        />
                                    </div>

                                    <div
                                        class="d-grid"
                                        style="min-width: 40px;"
                                    >
                                        <div
                                            class="text-decoration-none fw-bold mb-0"
                                        >
                                            <span
                                                class="fw-bold"
                                                style="
                                                    white-space: nowrap;
                                                    text-overflow: ellipsis;
                                                    overflow: hidden;
                                                    font-size: 16px;
                                                    text-decoration: none;
                                                "
                                            >
                                                {"Google / Gmail"}
                                            </span>
                                        </div>
                                        <p
                                            class="mb-0 text-muted"
                                            style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 12px;
                                                text-transform: uppercase;
                                                letter-spacing: 1px;
                                            "
                                        >
                                            {"social connection"}
                                        </p>
                                    </div>
                    
                                </div>
                            <p class="card-text">{"Allow your users to login with their Google Account"}</p>
                            </div>
                        </div>
                    </div>



                    <div class="col-sm-6 mb-4">
                        <div class="card card-hover" style="cursor: pointer;">
                            <div class="card-body p-4">

                                <div
                                    class="d-flex mb-3"
                                >
                                    <div
                                        style="flex: 0 0 auto; width: 40px; height: 40px;"
                                        class="d-flex justify-content-center align-items-center me-3"
                                    >
                                        <img
                                            src="/assets/icons/facebook-avatar.png"
                                            class="w-75"
                                        />
                                    </div>

                                    <div
                                        class="d-grid"
                                        style="min-width: 40px;"
                                    >
                                        <div
                                            class="text-decoration-none fw-bold mb-0"
                                        >
                                            <span
                                                class="fw-bold"
                                                style="
                                                    white-space: nowrap;
                                                    text-overflow: ellipsis;
                                                    overflow: hidden;
                                                    font-size: 16px;
                                                    text-decoration: none;
                                                "
                                            >
                                                {"Facebook"}
                                            </span>
                                        </div>
                                        <p
                                            class="mb-0 text-muted"
                                            style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 12px;
                                                text-transform: uppercase;
                                                letter-spacing: 1px;
                                            "
                                        >
                                            {"social connection"}
                                        </p>
                                    </div>
                    
                                </div>
                            <p class="card-text">{"Allow your users to login with their Google Account"}</p>
                            </div>
                        </div>
                    </div>


                    <div class="col-sm-6 mb-4">
                        <div class="card card-hover" style="cursor: pointer;">
                            <div class="card-body p-4">

                                <div
                                    class="d-flex mb-3"
                                >
                                    <div
                                        style="flex: 0 0 auto; width: 40px; height: 40px;"
                                        class="d-flex justify-content-center align-items-center rounded me-3 border"
                                    >
                                        <img
                                            src="/assets/icons/facebook-avatar.png"
                                            class="w-50"
                                        />
                                    </div>

                                    <div
                                        class="d-grid"
                                        style="min-width: 40px;"
                                    >
                                        <div
                                            class="text-decoration-none fw-bold mb-0"
                                        >
                                            <span
                                                class="fw-bold"
                                                style="
                                                    white-space: nowrap;
                                                    text-overflow: ellipsis;
                                                    overflow: hidden;
                                                    font-size: 16px;
                                                    text-decoration: none;
                                                "
                                            >
                                                {"Facebook"}
                                            </span>
                                        </div>
                                        <p
                                            class="mb-0 text-muted"
                                            style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 12px;
                                                text-transform: uppercase;
                                                letter-spacing: 1px;
                                            "
                                        >
                                            {"social connection"}
                                        </p>
                                    </div>
                    
                                </div>
                            <p class="card-text">{"Allow your users to login with their Google Account"}</p>
                            </div>
                        </div>
                    </div>




                </div>

            </div>
        }
    }
}
