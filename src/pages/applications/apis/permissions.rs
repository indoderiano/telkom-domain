use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
    }
};
use crate::types::{
	api::{ ApiDetails, ResponseApiDetails },
	ResponseMessage,
};
use crate::configs::server::API_URL;


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct PermissionsProps {
    pub api_details: ApiDetails,
}
pub struct Permissions {
    api_details: ApiDetails,
}

pub enum Msg {}

impl Component for Permissions {
    type Message = Msg;
    type Properties = PermissionsProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ConsoleService::info(&format!("Permissions props, api details = {:?}", props.api_details));

        Permissions {
            api_details: props.api_details,
        }
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

                <div class="mb-4">

                    <div>
                        <div
                            class="fs-4 fw-bold"
                        >
                            {"Add a Permissions (Scope)"}
                        </div>
                        <p>{"Define the permissions (scopes) that this API uses."}</p>
                    </div>

                    <ul class="list-group list-group-flush">
                        <li class="list-group-item border-bottom-0">
                            <div
                                class="d-flex"
                            >
                                <div
                                    class="flex-fill fw-bold"
                                >
                                    {"Permission"}
                                </div>
                                <div
                                    class="flex-fill fw-bold"
                                >
                                    {"Description"}
                                </div>
                                <div
                                    class="flex-shrink-1"
                                    style="min-width: 81px;"
                                >
                                </div>
                            </div>
                        </li>

                        <li class="list-group-item">
                            <div
                                class="d-flex"
                            >
                                <div
                                    class="flex-fill pe-2"
                                >
                                    <input
                                        type="text"
                                        class="form-control"
                                    />
                                </div>
                                <div
                                    class="flex-fill pe-2"
                                >
                                    <input
                                        type="text"
                                        class="form-control"
                                    />
                                </div>
                                <div
                                    class="flex-shrink-1"
                                    style="min-width: 81px;"
                                >
                                    <button type="button" class="btn btn-outline-secondary">
                                        <i class="bi bi-plus-lg me-1"></i>
                                        <span>{ "Add" }</span>
                                    </button>
                                </div>
                            </div>
                        </li>
                        
                    </ul>
                </div>
                
                <div>
                    <div>
                        <div
                            class="fs-4 fw-bold"
                        >
                            {"List of Permissions (Scopes)"}
                        </div>
                        <p>{"These are all the permissions (scopes) that this API uses."}</p>
                    </div>
            
                    <ul class="list-group list-group-flush">
                        <li class="list-group-item">
                            <div
                                class="d-flex"
                            >
                                <div
                                    class="flex-fill fw-bold"
                                >
                                    {"Permission"}
                                </div>
                                <div
                                    class="flex-fill fw-bold"
                                >
                                    {"Description"}
                                </div>
                                <div
                                    class="flex-shrink-1"
                                    style="min-width: 81px;"
                                >
                                </div>
                            </div>
                        </li>
                        { self.view_list() }
                        
                    </ul>
                </div>

            </div>
        }
    }
}

impl Permissions {
    fn view_list (&self) -> Vec<Html> {
        self.api_details.scopes.clone().iter().map(|scope| {
            html! {
                <li class="list-group-item">
                    <div
                        class="d-flex"
                    >
                        <div
                            class="flex-fill"
                        >
                            { scope.value.clone() }
                        </div>
                        <div
                            class="flex-fill"
                        >
                            { scope.description.clone() }
                        </div>
                        <div
                            class="flex-shrink-1"
                            style="min-width: 81px;"
                        >
                            <button type="button" class="btn btn-outline-secondary">
                                <i class="bi bi-trash"></i>
                            </button>
                        </div>
                    </div>
                </li>
            }
        })
        .collect()
    }
}