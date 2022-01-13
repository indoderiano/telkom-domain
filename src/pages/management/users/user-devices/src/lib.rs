use yew::prelude::*;

pub struct UserTabDevices {}

pub enum Msg {}

impl Component for UserTabDevices {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        UserTabDevices {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="mt-4">
                    <p>{"These are the devices being used by this particular user. If you click on Unlink, the refresh token will be revoked, forcing the user to re-login on the application."}</p>
                </div>
                <div class="mt-4 table-responsive">
                    <table class="table">
                        <thead>
                            <tr>
                                <th scope="col">{"Client"}</th>
                                <th scope="col">{"Devices"}</th>
                                <th scope="col">{"Number of Refresh Tokens"}</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <th scope="row">{"AppleTvApp"}</th>
                                <td>{"Chrome"}</td>
                                <td>{"1"}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </>
        }
    }
}
