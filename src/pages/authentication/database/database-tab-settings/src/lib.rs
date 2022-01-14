use yew::prelude::*;
pub struct TabSettings {}
pub enum Msg {}

impl Component for TabSettings {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TabSettings {}
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
                <p class="p-2 m-2">
                {"Securely store and manage username / password credentials either in an Auth0 Database or in your own store."}
                </p>
                
                <div class="card p-2 m-4">
                <div class="d-flex border-bottom border-1 list-hover justify-content-between align-items-center">
                    <div class="p-4 d-flex border-bottom" style="width: 40%;">
                    <div class="d-grid " style="min-width: 40px;">
                        <span class="fw-bold " style="
                                                                            width: 200%;
                                                                            font-size: 14px;
                                                                            ">
                        {"Requires Username"}
                        </span>
                        <p class="mb-0 text-muted" style="
                                                                            width: 200%;
                                                                            font-size: 14px;
                                                                            ">
                        {"Requires the user to provide a username in addition to email."}
                        </p>
                    </div>
                    </div>
                    <div class="form-check form-switch fs-4">
                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault" />
                    </div>
                </div>
                <div class="d-flex list-hover justify-content-between align-items-center">
                    <div class="p-4 d-flex" style="width: 40%;">
                    <div class="d-grid " style="min-width: 40px;">
                        <span class="fw-bold " style="
                                                                            width: 200%;
                                                                            font-size: 14px;
                                                                            ">
                        {"Username Length"}
                        </span>
                        <p class="mb-0 text-muted" style="
                                                                            width: 200%;
                                                                            font-size: 14px;
                                                                            ">
                        {"Set the minimum and maximum values allowed for a user to have as username."}
                        </p>
                    </div>
                    </div>
                </div>
                <div class="card db-input mx-0 p-0 border-0">
                    <div class="card-body-db-input pb-2 ">
                    <div class="d-flex list-hover justify-content-start align-items-center">
                        <div class="d-flex list-hover justify-content-between align-items-center">
                        <div class="px-2 d-flex">
                            <div class="d-grid m-2" style="min-width: 40px;">
                            <span class="fw-bold m-2" style="
                                                                                        font-size: 14px;
                                                                                        ">
                                {"Min"}
                            </span>
                            <input type="number" class="form-control" min="1" value="1" width="50px" />
                            </div>
                            <div class="d-grid m-2">
                            <span class="fw-bold m-2" style="
                                                                                        font-size: 14px;
                                                                                        ">
                                {"Max"}
                            </span>
                            <input type="number" class="form-control" min="1" value="15" width="50px" />
                            </div>
                        </div>
                        </div>
                    </div>
                    </div>
                </div>
                </div>
                
                <div class="card p-2 m-4">
                <div class="d-flex border-1 list-hover justify-content-between align-items-center">
                    <div class="p-4 d-flex" style="width: 40%;">
                    <div class="d-grid " style="min-width: 40px;">
                        <span class="fw-bold " style="
                                                                            width: 200%;
                                                                            font-size: 14px;
                                                                            ">
                        {"Import Users to Domain"}
                        </span>
                        <p class="mb-0 text-muted" style="
                                                                            width: 200%;
                                                                            font-size: 14px;
                                                                            ">
                        {"Gradually migrate your legacy user store to the Domain user store."}
                        </p>
                    </div>
                    </div>
                    <div class="form-check form-switch fs-4">
                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault" />
                    </div>
                </div>
                </div>

                <div class="card p-2 m-4">
                <div class="d-flex border-1 list-hover justify-content-between align-items-center">
                    <div class="p-4 d-flex" style="width: 40%;">
                    <div class="d-grid " style="min-width: 40px;">
                        <span class="fw-bold " style="
                                                                            width: 200%;
                                                                            font-size: 14px;
                                                                            ">
                        {"Disable Sign Ups"}
                        </span>
                        <p class="mb-0 text-muted" style="
                                                                            width: 200%;
                                                                            font-size: 14px;
                                                                            ">
                        {"Prevent new user sign ups to your application. You will still be able to create users with your API
                        credentials or from the dashboard."}
                        </p>
                    </div>
                    </div>
                    <div class="form-check form-switch fs-4">
                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault" />
                    </div>
                </div>
                </div>

                <div class="card p-2 m-4">
                <div class="d-flex border-1 list-hover justify-content-between align-items-center">
                    <div class="p-4 d-flex" style="width: 40%;">
                    <div class="d-grid " style="min-width: 40px;">
                        <span class="fw-bold " style="
                                                                            width: 200%;
                                                                            font-size: 14px;
                                                                            ">
                        {"Sync user profile attributes at each login"}
                        </span>
                        <p class="mb-0 text-muted" style="
                                                                            width: 200%;
                                                                            font-size: 14px;
                                                                            ">
                        {"Update the user profile upon each login for custom databases where import mode is disabled."}
                        </p>
                    </div>
                    </div>
                    <div class="form-check form-switch fs-4">
                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault" />
                    </div>
                </div>
                </div>

                <span class="fw-bold p-2 m-4" style="
                                                                            width: 200%;
                                                                            font-size: 20px;
                                                                            ">
                {"Danger Zone"}
                </span>

                <div class="card p-2 m-4 alert alert-danger">
                <div class="d-flex border-1 list-hover justify-content-between align-items-center">
                    <div class="p-2 d-flex" style="width: 40%;">
                    <div class="d-grid " style="min-width: 40px;">
                        <span class="fw-bold py-2" style="
                                                                            width: 200%;
                                                                            font-size: 16px;
                                                                            ">
                        {"Sync user profile attributes at each login"}
                        </span>
                        <p class="mb-0" style="
                                                                            width: 200%;
                                                                            font-size: 16px;
                                                                            ">
                        {"Update the user profile upon each login for custom databases where import mode is disabled."}
                        </p>
                    </div>
                    </div>
                    <button type="button" class="btn btn-danger p-2 mx-2">{"Delete"}</button>
                </div>
                </div>
                
            </>
        }
    }
}