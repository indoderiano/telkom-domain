use yew::prelude::*;
// use super::nodejs::Nodejs;

pub struct SocialApplications {}

pub enum Msg {}

impl Component for SocialApplications {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SocialApplications {}
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
                style="font-size: 14px;"
            >

                <p>{"Applications using this connection."}</p>

                <div
                    class="d-flex border-bottom border-1 list-hover justify-content-between align-items-center"
                >
                    <div
                        class="p-3 d-flex"
                        style="width: 40%;"
                    >
                        <div
                            style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                            class="d-flex justify-content-center align-items-center rounded me-3"
                        >
                            <i class="bi bi-gear"></i>
                        </div>

                        <div
                            class="d-grid"
                            style="min-width: 40px;"
                        >
                            <span
                                class="fw-bold"
                                style="
                                    white-space: nowrap;
                                    text-overflow: ellipsis;
                                    overflow: hidden;
                                    font-size: 14px;
                                    text-decoration: none;
                                "
                            >
                                {"Adobe Sign"}
                            </span>
                            <p
                                class="mb-0 text-muted"
                                style="
                                    white-space: nowrap;
                                    text-overflow: ellipsis;
                                    overflow: hidden;
                                    font-size: 14px;
                                "
                            >
                                {"Generic"}
                            </p>
                        </div>

                    </div>
                    
                    <div class="form-check form-switch fs-4">
                        <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault"/>
                    </div>
                </div>

                <div
                    class="d-flex border-bottom border-1 list-hover justify-content-between align-items-center"
                >
                    <div
                        class="p-3 d-flex"
                        style="width: 40%;"
                    >
                        <div
                            style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                            class="d-flex justify-content-center align-items-center rounded me-3"
                        >
                            <i class="bi bi-gear"></i>
                        </div>

                        <div
                            class="d-grid"
                            style="min-width: 40px;"
                        >
                            <span
                                class="fw-bold"
                                style="
                                    white-space: nowrap;
                                    text-overflow: ellipsis;
                                    overflow: hidden;
                                    font-size: 14px;
                                    text-decoration: none;
                                "
                            >
                                {"Default App"}
                            </span>
                            <p
                                class="mb-0 text-muted"
                                style="
                                    white-space: nowrap;
                                    text-overflow: ellipsis;
                                    overflow: hidden;
                                    font-size: 14px;
                                "
                            >
                                {"Generic"}
                            </p>
                        </div>

                    </div>
                    
                    <div class="form-check form-switch fs-4">
                        <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault"/>
                    </div>
                </div>
            </div>
        }
    }
}
