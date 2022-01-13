use yew::prelude::*;

mod nodejs;
use nodejs::Nodejs;

pub struct Quickstart {}

pub enum Msg {}

impl Component for Quickstart {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Quickstart {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div
                style="font-size: 14px;"
            >
                <div
                    class="fw-bold mb-1"
                >
                    {"1. Choose a JWT Library"}
                </div>
                <p>
                    {"As your API will be parsing JWT formatted access tokens, you will need to setup these capabilities on your API."}
                </p>
                <p>
                    {"You can navigate to "}
                    <a
                        href="https://jwt.io/#libraries" target="_blank" rel="noopener noreferrer">
                        {"jwt.io"}
                    </a>
                    {"and choose from there. Remember to pick a library that support your selected signing algorithm."}
                </p>
                <div
                    class="fw-bold mb-1"
                >
                    {"2. Configuring your API to accept RS256 signed tokens"}
                </div>
                <p>
                    {"Configure the library that will validate the "}
                    <code
                        class="rounded text-muted"
                        style="
                            background-color: #eff0f2;
                            white-space: nowrap;
                            text-overflow: ellipsis;
                            overflow: hidden;
                            font-size: 14px;
                            padding: 2px 6px;
                        ">
                        {"access tokens"}
                    </code>
                    {"in your API. Validating a token means that you are certain you can trust its contents."}
                </p>

                <ul class="nav nav-tabs">
                    <li class="nav-item">
                        <a class="nav-link active" aria-current="page" href="#">{"Node JS"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="#">{"C#"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="#">{"PHP"}</a>
                    </li>
                </ul>
                <Nodejs/>

                <div
                    class="fw-bold mb-1 mt-3"
                >
                    {"3. Define Permissions and manage Authorization Policies"}
                </div>
                <p>
                    {"On the Permissions tab, you can define what scopes this API may accept. On the Settings tab, you can enable or disable Authorization Policy Enforcement for your API. If enabled, only Roles and Permissions assigned to your user will be included in the token."}
                </p>
                <p>
                    {"That's it!"}
                </p>
            </div>
        }
    }
}
