use yew::prelude::*;

pub struct EmailTry {}

pub enum Msg {}

impl Component for EmailTry {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        EmailTry {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
           <div class="p-2" style="font-size: 14px;">
                
                <div class="mt-2 mb-3">
                    <div class="alert alert-primary d-flex align-items-center" role="alert">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2" viewBox="0 0 16 16" role="img" aria-label="Warning:">
                            <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/>
                        </svg>
                        <div>
                            {"You need to enable one application at least in order to be able to try this connection."}
                        </div>
                    </div>    
                </div>

                <div class="mb-3">
                    <p class="text-muted">{"Try this connection by specifying an application and a recipient."}</p>
                </div>

                <div class="mb-3">
                    <label class="form-label text-muted">{"Application"}</label>
                    <select class="form-select" aria-label="Disabled select example" disabled=true>
                        <option selected=true>{""}</option>
                        <option value="1">{""}</option>
                        <option value="2">{""}</option>
                        <option value="3">{""}</option>
                    </select>
                    <div class="pt-1">
                        <p class="text-muted">{"The application on which you want to try this connection."}</p>
                    </div>
                </div>


                <div class="mb-3">
                    <label class="form-label text-muted">{"Email recipient"}</label>
                    <input type="text" id="disabledTextInput" class="form-control" placeholder="yourmail@mail.com" disabled=true />
                    <div class="pt-1">
                        <p class="text-muted">{"The email address which will receive the test email."}</p>
                    </div>
                </div>


                <div class="mb-3">
                    <label class="form-label text-muted">{"Mode"}</label>
                    <select class="form-select text-muted" aria-label="Disabled select example" disabled=true>
                        <option selected=true value="code">{"code"}</option>
                        <option value="1">{""}</option>
                        <option value="2">{""}</option>
                        <option value="3">{""}</option>
                    </select>
                    <div class="pt-1">
                        <p class="text-muted">{"Specify whether you want to receive a magic link or a code."}</p>
                    </div>
                </div>



           </div>
        
        }
    }   

}
