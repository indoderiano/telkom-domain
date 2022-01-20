use yew::prelude::*;

pub struct EmailApp {}

pub enum Msg {}

impl Component for EmailApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        EmailApp {}
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
                <p>{ " Email Applications using this connection." }</p>
                <div class="mb-3">
                    <table class="table">
                        <tbody>
                            <tr>
                                <td>
                                    <p class="fw-bold">{"TelAuth Management API (Test Application)"}</p>
                                    <p>{"Machine to Machine"}</p>
                                </td>
                                <td>
                                    <div class="form-check form-switch">
                                    <input class="form-check-input" type="checkbox" id="authManageSwitch"/>
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="fw-bold">{"API Explorer Application"}</p>
                                    <p>{"Machine to Machine"}</p>
                                </td>
                                <td>
                                    <div class="form-check form-switch">
                                    <input class="form-check-input" type="checkbox" id="apiExplorerSwitch"/>
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="fw-bold">{"Placeholder (Test Application)"}</p>
                                    <p>{"Machine to Machine"}</p>
                                </td>
                                <td>
                                    <div class="form-check form-switch">
                                    <input class="form-check-input" type="checkbox" id="placeholderTestSwitch"/>
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="fw-bold">{"Default App"}</p>
                                    <p>{"Generic"}</p>
                                </td>
                                <td>
                                    <div class="form-check form-switch">
                                    <input class="form-check-input" type="checkbox" id="defaultAppSwitch"/>
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="fw-bold">{"Native App"}</p>
                                    <p>{"Generic"}</p>
                                </td>
                                <td>
                                    <div class="form-check form-switch">
                                    <input class="form-check-input" type="checkbox" id="nativeAppSwitch"/>
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="fw-bold">{"RWA"}</p>
                                    <p>{"Regular Web Application"}</p>
                                </td>
                                <td>
                                    <div class="form-check form-switch">
                                    <input class="form-check-input" type="checkbox" id="regWebAppSwitch"/>
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="fw-bold">{"SPWA"}</p>
                                    <p>{"Generic"}</p>
                                </td>
                                <td>
                                    <div class="form-check form-switch">
                                    <input class="form-check-input" type="checkbox" id="singPageAppSwitch"/>
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-primary">{"Save"}</button>
                </div>
            </div>
        }
    }
}
