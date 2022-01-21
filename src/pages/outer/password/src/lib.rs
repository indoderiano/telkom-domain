use yew::prelude::*;

pub struct RequestPassPage {}

pub enum Msg {}

impl Component for RequestPassPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        RequestPassPage {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style="background-color: #dee2e6; min-height:100vh;">
                <div class="login-page form-signin border">
                    
                    <div class="text-center align-middle" style="background-color: white; margin-top: 7vh">
                        
                        <div class="logo-image">
                            <img src="https://www.telkom.co.id/data/image_upload/page/1594112895830_compress_PNG%20Icon%20Telkom.png" 
                            alt="telAuth"
                            style="width: 80px; height: 80px; margin-top: 10px"/>
                        </div>

                        <h1 class="h3 mb-3 fw-normal">{"Enter your password"}</h1>

                        <div class="form-floating m-auto w-75 d-flex justify-content-center mt-4">
                            <input type="email" class="d-flex form-control" id={"floatingInput"} placeholder="name@example.com"/>
                            <label for="floatingInput">{"Email Address"}</label>
                        </div>
                    
                        <div class="form-floating m-auto w-75 d-flex justify-content-center mt-4">
                            <input type="password" class="d-flex form-control" id={"floatingInput"}/>
                            <label for="floatingInput">{"Password"}</label>
                        </div>

                        <p class="mt-3 text-start" style="margin-left:55px;"><a href="">{"Forgot your password?"}</a></p>

                        <button class="w-75 btn btn-lg btn-primary mt-2 mb-5 fs-6" type="submit">{"Continue"}</button>



                    </div>
                
                </div>    
            </div> 
        }
    }
}
