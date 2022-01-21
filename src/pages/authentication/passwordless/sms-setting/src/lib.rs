use yew::prelude::*;

pub struct SmsSetting {}

pub enum Msg {}

impl Component for SmsSetting {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SmsSetting {}
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
                <p>{ "Twilio SMS connection is a service that allows your users to use one-time password authentication via text messages. Learn more about integrating " }<a href="">{"SMS connections."}</a></p>
                <div class="mt-2 mb-3">
                    <label class="form-label">{"Connections"}</label>
                    <input class="form-control" type="text" value="sms" aria-label="Disabled input example" disabled=true readonly=true/>
                    <p>{"If you are triggering a login manually, this is the identifier you would use on the connection parameter"}</p>
                </div>
                <div class="mb-3">
                    <label class="form-label">{"Twilio SID"}</label>
                    <input class="form-control" type="text" placeholder="YOUR_TWILIO_SID" aria-label="twilio input example"/>
                    <p>{"Learn more: "}<a href="">{"How to create an Application SID."}</a></p>
                </div>
                <div class="mb-3">
                    <label class="form-label d-block">{"SMS Source"}</label>
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="radio" name="inlineRadioOptions" id="smsSourceUms" value="Use Messaging Service"/>
                        <label class="form-check-label" for="smsSourceUms">{"Use Messaging Service"}</label>
                    </div>
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="radio" name="inlineRadioOptions" id="smsSourceUseFrom" value="Use From"/>
                        <label class="form-check-label" for="smsSourceUseFrom">{"Use From"}</label>
                    </div>
                </div>
                <div class="mb-3">
                    <label class="form-label">{"Messaging Service SID"}</label>
                    <input class="form-control" type="text" placeholder="YOUR_TWILIO_MESSAGING_SERVICE_SID" aria-label="Messaging service SID example" disabled=true/>
                    <p>{"Learn more: "}<a href="">{"Sending Messages with the Messaging Service."}</a></p>
                </div>
                <div class="mb-3">
                    <label class="form-label">{"From"}</label>
                    <input class="form-control" type="text" placeholder="+15555555" aria-label="default input example"/>
                </div>
                <div class="mb-3">
                    <label class="form-label d-block">{"SMS Syntax"}</label>
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="radio" name="inlineRadioOptions" id="smsSyntaxLiquid" value="Liquid"/>
                        <label class="form-check-label" for="smsSyntaxLiquid">{"Use Messaging Service"}</label>
                    </div>
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="radio" name="inlineRadioOptions" id="smsSyntaxMarkdown" value="Markdown"/>
                        <label class="form-check-label" for="smsSyntaxMarkdown">{"Use From"}</label>
                    </div>
                </div>

                <div class="mb-3">
                    <label class="form-label d-block">{"Message"}</label>
                    <div class="form-floating">
                        <textarea class="form-control text-light pt-1" placeholder="Your verification code is: @@password@@" style="background-color:rgb(47,56,61); height:100px;" id="floatingTextarea2"></textarea>
                    </div>
                    <p>{"You can use @@password@@ as a placeholder of where the password value should be placed."}</p>
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
                    <div class="pt-1">{"The length of the resulting one-time password."}</div>
                </div>
                <div class="mb-3">
                    <label class="form-label">{"Disable Sign Ups"}</label>
                    <div class="form-check form-switch">
                        <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault"/>
                    </div>
                    <p class="text-muted">{"Check this if you want to prevent sign ups to your application. You will still be able to create users with your API credentials."}</p>
                </div>

                <div class="modal-footer">
                    <button type="button" class="btn btn-primary" >{"Save"}</button>
                </div>
            </div>
        }
    }
}
