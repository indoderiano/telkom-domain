use yew::prelude::*;
use yew_router::components::RouterAnchor;
// use crate::app::AppRoute;
use router::AppRoute;
use social_applications::SocialApplications;
use social_tab_settings::TabSettings;

pub enum Content {
    Settings,
    Applications
}

pub struct SocialSettings {
    content: Content,
    link: ComponentLink<Self>
}

pub enum Msg {
    ChangeContent(Content)
}

impl Component for SocialSettings {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SocialSettings {
            content: Content::Settings,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeContent(content) => {
                self.content = content;
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
                        style="flex: 0 0 auto; width: 64px; height: 64px;"
                        class="d-flex justify-content-center align-items-center rounded me-4 border"
                    >
                        <img
                            src="/assets/icons/google-avatar.png"
                            class="w-50"
                        />
                    </div>

                    <div
                        class="d-flex flex-column"
                    >
                        <h2>{"google-oauth2"}</h2>
                        <div
                            class="text-muted"
                        >
                            <span
                                class="me-4"
                            >
                                {"Google / Gmail"}
                            </span>
                            <span>
                                {"Identifier"}
                            </span>
                            <span
                                class="rounded ms-2"
                                style="
                                    background-color: #eff0f2;
                                    white-space: nowrap;
                                    text-overflow: ellipsis;
                                    overflow: hidden;
                                    font-size: 14px;
                                    padding: 2px 6px;
                                    font-family: 'Roboto Mono', monospace;
                                "
                            >
                                {"con_qgZPycWvQ4BBRzcY"}
                            </span>
                        </div>
                    </div>
                </div>

                <div
                    class="mb-4"
                >
                    <ul class="nav nav-tabs">
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Settings))
                            class="nav-item"
                        >
                        <a
                            // class="nav-link active"
                            class={
                                match self.content {
                                    Content::Settings => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            aria-current="page"
                            href="#"
                        >
                            {"Settings"}</a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Applications))
                            class="nav-item">
                        <a
                            // class="nav-link"
                            class={
                                match self.content {
                                    Content::Applications => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            href="#">{"Applications"}</a>
                        </li>
                    </ul>
                </div>

                // <Quickstart/>
                // <TabSettings/>

                {
                    match self.content {
                        Content::Applications => html! { <SocialApplications/> },
                        Content::Settings => html! { <TabSettings/> }
                    }
                }
            </div>
        }
    }
}
