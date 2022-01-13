use yew::prelude::*;

pub struct Nodejs {}

pub enum Msg {}

impl Component for Nodejs {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Nodejs {}
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
                    class="rounded p-3 text-light"
                    style="
                        background-color: rgb(47, 56, 61);
                        white-space: nowrap;
                        text-overflow: ellipsis;
                        overflow: hidden;
                        font-size: 14px;
                        padding: 2px 6px;
                        font-family: 'Roboto Mono', monospace;
                    "
                >
                    <span class="code-keyword">{"var"}</span>{" express = "}<span class="code-require">{"require"}</span>{"("}<span class="code-string">{"'express'"}</span>{");"}
                    <br/>
                    <span class="code-keyword">{"var"}</span>{" app = express();"}
                    <br/>
                    <span class="code-keyword">{"var"}</span>{" jwt = "}<span class="code-require">{"require"}</span>{"("}<span class="code-string">{"'express-jwt'"}</span>{");"}
                    <br/>
                    <span class="code-keyword">{"var"}</span>{" jwks = "}<span class="code-require">{"require"}</span>{"("}<span class="code-string">{"'jwks-rsa'"}</span>{");"}
                    <br/>

                    <br/>
                    <span class="code-keyword">{"var"}</span>{" port = process.env.PORT || "}<span class="code-number">{"8080"}</span>{";"}
                    <br/>
                    <br/>
                    
                    <span class="code-keyword">{"var"}</span>{" jwtCheck = jwt({"}
                        <br/>
                        <span class="tab-1">{"secret"}</span>{": jwks.expressJwtSecret({"}
                            <br/>
                            <span class="tab-2">{"cache"}</span>{": "}<span class="code-literal">{"true"}</span>{","}
                            <br/>
                            <span class="tab-2">{"rateLimit"}</span>{": "}<span class="code-literal">{"true"}</span>{","}
                            <br/>
                            <span class="tab-2">{"jwksRequestsPerMinute"}</span>{": "}<span class="code-number">{"5"}</span>{","}
                            <br/>
                            <span class="tab-2">{"jwksUri"}</span>{": "}<span class="code-string">{"'https://dev-r5y8heyf.au.auth0.com/.well-known/jwks.json'"}</span>
                            <br/>
                        <span class="tab-1">{"}),"}</span>
                        <br/>
                        <span class="tab-1">{"audience"}</span>{": "}<span class="code-string">{"'https://test-api/'"}</span>{","}
                        <br/>
                        <span class="tab-1">{"issuer"}</span>{": "}<span class="code-string">{"'https://dev-r5y8heyf.au.auth0.com/'"}</span>{","}
                        <br/>
                        <span class="tab-1">{"algorithms"}</span>{": ["}<span class="code-string">{"'RS256'"}</span>{"]"}
                        <br/>
                    {"});"}
                    <br/>
                    
                    <br/>
                    {"app.use(jwtCheck);"}
                    <br/>
                    <br/>
                    
                    {"app.get("}<span class="code-string">{"'/authorized'"}</span>{", "}<span class="hljs-function"><span class="code-keyword">{"function"}</span>{" ("}<span class="code-params">{"req, res"}</span>{") "}</span>{"{"}
                        <br/>
                        <span class="tab-1">{"res.send("}</span><span class="code-string">{"'Secured Resource'"}</span>{");"}
                        <br/>
                    {"});"}
                    <br/>
                    
                    <br/>
                    {"app.listen(port);"}
                </div>
            </div>
        }
    }
}
