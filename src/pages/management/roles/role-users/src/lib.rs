use yew::prelude::*;

pub struct TabUsers {}

pub enum Msg {}

impl Component for TabUsers {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TabUsers {}
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
                            {"User that have this role directly assigned"}
                        </p>
                    </div>
                    <div class="col-3 d-flex justify-content-end">
                        <button type="button" class="btn btn-primary">{"Add Users"}</button>
                    </div>
                </div>
                <div class="mt-4">
                    <table class="table">
                        <thead>
                            <tr>
                                <th scope="col">{"Users"}</th>
                                <th scope="col"></th>
                            </tr>
                        </thead>
                    </table>
                    <div class="mt-2">
                        <div class="alert alert-secondary text-center" role="alert">
                            {"There are no users assigned to this role"}
                        </div>
                    </div>
                </div>
            </div>

            </>
        }
    }
}
