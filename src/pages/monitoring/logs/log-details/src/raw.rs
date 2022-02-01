use yew::prelude::*;

pub struct Raw {
    letter: String,
}

pub enum Msg {}

impl Component for Raw {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Raw {
            letter: String::from("")
        }
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
                style="font-size: 14px; border-radius: 3px; height: 300px; overflow-x: scroll;"
            >
                <div
                    class="p-3 text-light"
                    style="
                        background-color: rgb(47, 56, 61);
                        text-overflow: ellipsis;
                        overflow: hidden;
                        font-size: 14px;
                        padding: 2px 6px;
                        font-family: 'Roboto Mono', monospace;
                    "
                >
                    {"{"}
                    <div class="ms-3">
                        <span class="code-number">{"''date''"}</span>{": "}{"2022-01-31T12:35:50.341Z"}
                        <br/>
                        <span class="code-number">{"type"}</span>{": "}{"mgmt_api_read"}
                        <br/>
                        <span class="code-number">{"description"}</span>{": "}{"Get a client"}
                        <br/>
                        <span class="code-number">{"client_id"}</span>{": "}{"MrlpRDQKGK9ENLgHd89jWTMkKvf0O7t9"}
                        <br/>
                        <span class="code-number">{"client_name"}</span>{": "}{"indo"}
                        <br/>
                        <span class="code-number">{"ip"}</span>{": "}{"35.166.202.113"}
                        <br/>
                        <span class="code-number">{"user_agent"}</span>{": "}{"Chrome 97.0.4692 / Windows 10.0.0"}
                        <br/>
                        <span class="code-number">{"details"}</span>{": {"}
                        <div class="ms-3">
                            <span class="code-number">{"accessedSecrets"}</span>{": ["}
                            <div class="ms-3">
                                {"client_secret"}
                            </div>
                            {"],"}
                            <span class="code-number">{"request"}</span>{": {"}
                            <div class="ms-3">
                                <span class="code-number">{"method"}</span>{": "}{"get"}{","}
                                <br/>
                                <span class="code-number">{"path"}</span>{": "}{"/api/v2/clients/6XbLBxnKtMOeFfPpcqVJj6e8CmxvBgMb"}{","}
                                <br/>
                                <span class="code-number">{"query"}</span>{": "}{"{}"}{","}
                                <br/>
                                <span class="code-number">{"userAgent"}</span>{": "}{"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36"}{","}
                                <br/>
                                <span class="code-number">{"body"}</span>{": "}<span class="code-literal">{"null"}</span>{","}
                                <br/>
                                <span class="code-number">{"channel"}</span>{": "}{"https://manage.auth0.com/"}{","}
                                <br/>
                            </div>
                            {"}"}
                        </div>
                        {"}"}
                    </div>
                    {"}"}





                    // <span class="code-keyword">{"var"}</span>{" express = "}<span class="code-require">{"require"}</span>{"("}<span class="code-string">{"'express'"}</span>{");"}
                    // <br/>
                    // <span class="code-keyword">{"var"}</span>{" app = express();"}
                    // <br/>
                    // <span class="code-keyword">{"var"}</span>{" jwt = "}<span class="code-require">{"require"}</span>{"("}<span class="code-string">{"'express-jwt'"}</span>{");"}
                    // <br/>
                    // <span class="code-keyword">{"var"}</span>{" jwks = "}<span class="code-require">{"require"}</span>{"("}<span class="code-string">{"'jwks-rsa'"}</span>{");"}
                    // <br/>

                    // <br/>
                    // <span class="code-keyword">{"var"}</span>{" port = process.env.PORT || "}<span class="code-number">{"8080"}</span>{";"}
                    // <br/>
                    // <br/>
                    
                    // <span class="code-keyword">{"var"}</span>{" jwtCheck = jwt({"}
                    //     <br/>
                    //     <span class="tab-1">{"secret"}</span>{": jwks.expressJwtSecret({"}
                    //         <br/>
                    //         <span class="tab-2">{"cache"}</span>{": "}<span class="code-literal">{"true"}</span>{","}
                    //         <br/>
                    //         <span class="tab-2">{"rateLimit"}</span>{": "}<span class="code-literal">{"true"}</span>{","}
                    //         <br/>
                    //         <span class="tab-2">{"jwksRequestsPerMinute"}</span>{": "}<span class="code-number">{"5"}</span>{","}
                    //         <br/>
                    //         <span class="tab-2">{"jwksUri"}</span>{": "}<span class="code-string">{"'https://dev-r5y8heyf.au.auth0.com/.well-known/jwks.json'"}</span>
                    //         <br/>
                    //     <span class="tab-1">{"}),"}</span>
                    //     <br/>
                    //     <span class="tab-1">{"audience"}</span>{": "}<span class="code-string">{"'https://test-api/'"}</span>{","}
                    //     <br/>
                    //     <span class="tab-1">{"issuer"}</span>{": "}<span class="code-string">{"'https://dev-r5y8heyf.au.auth0.com/'"}</span>{","}
                    //     <br/>
                    //     <span class="tab-1">{"algorithms"}</span>{": ["}<span class="code-string">{"'RS256'"}</span>{"]"}
                    //     <br/>
                    // {"});"}
                    // <br/>
                    
                    // <br/>
                    // {"app.use(jwtCheck);"}
                    // <br/>
                    // <br/>
                    
                    // {"app.get("}<span class="code-string">{"'/authorized'"}</span>{", "}<span class="hljs-function"><span class="code-keyword">{"function"}</span>{" ("}<span class="code-params">{"req, res"}</span>{") "}</span>{"{"}
                    //     <br/>
                    //     <span class="tab-1">{"res.send("}</span><span class="code-string">{"'Secured Resource'"}</span>{");"}
                    //     <br/>
                    // {"});"}
                    // <br/>
                    
                    // <br/>
                    // {"app.listen(port);"}


                </div>
            </div>
        }
    }
}
