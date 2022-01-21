use yew::prelude::*;

pub struct EmailSettings {}

pub enum Msg {}

impl Component for EmailSettings {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        EmailSettings {}
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
                <div class="mb-3">
                    <label class="form-label">{"Connection"}</label>
                    <input class="form-control" type="text" value="email" aria-label="Disabled input example" disabled=true readonly=true/>
                    <p>{"If you are triggering a login manually, this is the identifier you would use on the connection parameter"}</p>
                </div>

                <div class="mb-3">
                    <label class="form-label">{"From"}</label>
                    <input class="form-control" type="text" placeholder="myname@mycompany.com" aria-label="from input example"/>
                </div>

                <div class="mb-3">
                    <label class="form-label">{"Subject"}</label>
                    <input class="form-control" type="text" placeholder="Welcome to {{application name}}" aria-label="app subject input example"/>
                </div>

                <div class="mb-3 w-75">
                    <label class="form-label">{"Body"}</label>
                    <p>{"The content of the email your users are going to receive."}</p>
                    <div class="mb-2 row 2-50">
                        <div class="col">
                        <button  type="button" class="text-color-primary btn btn-secondary btn-default:hover" style="background-color:#96989d !important; font-size:12px; ">{"Reset to last saved"}</button>
                        </div>
                        <div class="col">
                        <button type="button" class="text-color-primary btn btn-secondary btn-default:hover" style="background-color:#96989d !important; font-size:12px;">{"Reset to default"}</button>
                        </div>
                    </div>
                </div>

                <div class="mb-3">
                    <label class="form-label">{"Authentication Parameter"}</label>
                    <div class="form-floating">
                        <textarea class="form-control text-light pt-1" placeholder="Your verification code is: @@password@@" style="background-color:rgb(47,56,61); height:100px;" id="floatingTextarea2"></textarea>
                    </div>
                    <p>{"Query string parameters to be included as part of the generated link.

                    "}</p>
                </div>


                 <div class="mb-3">
                    <label class="form-label">{"OTP Expiry"}</label>
                    <div class="input-group mb-3">
                        <input type="number" class="form-control" min="1" aria-label="Recipient's username" aria-describedby="otpExpiry"/>
                        <span class="input-group-text" id="otpExpiry">{"seconds"}</span>
                    </div>
                    <p>{"The time step, in seconds, between new passwords."}</p>
                </div>

                <div class="mb-3">
                    <label class="form-label">{"OTP Length"}</label>
                    <div class="input-group mb-3">
                        <input type="number" class="form-control" min="1" aria-label="Recipient's username" aria-describedby="otpLength"/>
                        <span class="input-group-text" id="otpLength">{"seconds"}</span>
                    </div>
                    <p>{"The length of the resulting one-time password."}</p>
                </div>

                <div class="mb-3">
                    <label class="form-label">{"Disable Sign Ups"}</label>
                    <div class="form-check form-switch">
                        <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault"/>
                    </div>
                    <p class="text-muted">{"Check this if you want to prevent sign ups to your application. You will still be able to create users with your API credentials."}</p>
                </div>

                <div class="modal-footer">
                    <button type="button" class="btn btn-primary">{"Save"}</button>
                </div>

            </div>
        }
    }
}
