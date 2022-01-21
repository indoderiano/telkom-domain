use yew::prelude::*;

pub struct Activity {}

pub enum Msg {}

impl Component for Activity {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Activity {}
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
                <div class="flex-fill fs-3 fw-bold mt-3"> {"Activity"}</div>
                
                <div class="card mt-3 me-2">
                    <div class="card-body">
                        <div class="container">
                            <div class="row">
                                <div class="col">
                                    <p class="text-muted">{"Total Users"}</p>
                                    <p class="fw-bold">{"0"}</p>
                                </div>
                                <div class="col">
                                    <p class="text-muted">{"Application"}</p>
                                    <p class="fw-bold">{"8"}</p>
                                </div>
                                <div class="col">
                                    <p class="text-muted">{"APIs"}</p>
                                    <p class="fw-bold">{"3"}</p>
                                </div>
                                <div class="col">
                                    <p class="text-muted">{"Connnection"}</p>
                                    <p class="fw-bold">{"0"}</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

            </>
        }
    }
}
