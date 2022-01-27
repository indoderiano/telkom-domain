use yew::prelude::*;

pub struct TabPermissions {}

pub enum Msg {}

impl Component for TabPermissions {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TabPermissions {}
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
            <div class="mt-4 p-4">
                <div class="row">
                    <div class="col-9">
                        <p>
                            { "Add Permissions to this Role. Users who have this Role will receive all Permissions below that match the API of their login request." }
                        </p>
                    </div>
                    <div class="col-3 d-flex align-items-center justify-content-end">
                        <button type="button" class="btn btn-primary text-center" style="width:80%; height:60%">{"Add Permissions"}</button>
                    </div>
                </div>
                <div class="mt-4">
                    <table class="table">
                        <thead>
                            <tr>
                                <th scope="col">{"Permission"}</th>
                                <th scope="col">{"Description"}</th>
                                <th scope="col">{"API"}</th>
                            </tr>
                        </thead>
                    </table>
                    <div class="mt-2">
                        <div class="alert alert-secondary text-center" role="alert">
                            {"No Permissions assigned"}
                        </div>
                    </div>
                </div>
            </div>
            </>
        }
    }
}
