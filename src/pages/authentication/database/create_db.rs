use yew::prelude::*;
use yew_router::components::RouterAnchor;
// use crate::app::AppRoute;
use router::AppRoute;


pub struct DbCreate {
    // link: ComponentLink<Self>
}

pub enum Msg {}

impl Component for DbCreate {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        DbCreate {
            // link
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        html! {
            <div class="py-5 px-4 m-auto" style="max-width: 1048px; font-size:14px;">
                <Anchor route=AppRoute::DatabaseHome classes="text-decoration-none domain-link-dark">
                    <i class="bi bi-arrow-left me-2"></i>
                    {"Back to Database Connection"}
                </Anchor>
                <div class="d-flex mb-5 mt-3">
                    <div class="d-flex flex-column">
                        <h2>{"New Database Connection"}</h2>
                    </div>
                </div>
                <div>
                    <div class="border rounded p-4 d-flex flex-column mb-5" style="font-size: 14px;">
                        <div class="d-flex flex-row ">
                            <div style="width: 100%;">
                                <div class="mb-4 ">
                                    <p class="mb-2 fw-bold">
                                        {"Name"}
                                    </p>
                                    <div class="input-group mb-2">
                                        <input type="text" class="form-control bg-input-grey"
                                            aria-label="Dollar amount (with dot and two decimal places)" placeholder="Connection name" />
                                    </div>
                                    <p class="text-color-disabled">
                                        {"Must start and end with an alphanumeric character and can only contain alphanumeric characters and
                                        '-'. Can't have more than 35 characters."}
                                    </p>
                                </div>
                                <div class="card db-input my-2">
                                    <div class="card-body-db-input p-2 px-4">
                                        <div class="card p-2 m-2">
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
                                                                    <input type="number" class="form-control" min="1" value="1" width="50px"/>
                                                                </div>
                                                                <div class="d-grid m-2">
                                                                    <span class="fw-bold m-2" style="
                                                                        font-size: 14px;
                                                                        ">
                                                                    {"Max"}
                                                                    </span>
                                                                    <input type="number" class="form-control" min="1" value="15" width="50px"/>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="card db-input mt-4 mx-2">
                                            <div class="card-body-db-input py-2 px-2">
                                                <div class="d-flex list-hover justify-content-between align-items-center">
                                                    <div class="p-4 d-flex" style="width: 40%;">
                                                        <div class="d-grid " style="min-width: 40px;">
                                                            <span class="fw-bold " style="
                                                                white-space: nowrap;
                                                                text-overflow: ellipsis;
                                                                overflow: hidden;
                                                                font-size: 14px;
                                                                text-decoration: none;
                                                                ">
                                                            {"Disable Sign Ups"}
                                                            </span>
                                                            <p class="mb-0 text-muted" style="
                                                                width: 200%;
                                                                font-size: 14px;
                                                                ">
                                                                {"Prevent new user sign ups to your application. You will still be able to create users with
                                                                your API credentials or from the dashboard."}
                                                            </p>
                                                        </div>
                                                    </div>
                                                    <div class="form-check form-switch fs-4">
                                                        <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault" />
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="card db-input mb-1 mt-4 mx-0 border-0">
                                            <div class="card-body-db-input p-2 px-4 ">
                                                <div class="d-flex list-hover justify-content-start align-items-center">
                                                    <div class="btn btn-primary d-flex align-items-center mx-2">
                                                    <span>{"Create"}</span>
                                                    </div>
                                                    <div class="btn btn-secondary d-flex align-items-center mx-2">
                                                    <span>{"Cancel"}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
