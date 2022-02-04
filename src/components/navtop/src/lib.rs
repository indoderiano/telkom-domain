use yew::prelude::*;
use logo::Logo;
use yewdux::dispatch::Dispatcher;
use store::{
    AppDispatch,
    DataAccountAction,
    // DataAccount,
};
// use crate::types::{
//     ResponseLogin,
// };
use yewtil::NeqAssign;
use yew::services::{
    ConsoleService,
    storage::{ StorageService, Area },
};
use yew_router::components::RouterAnchor;
// use crate::types::LOCALSTORAGE_KEY;
use router::AppRoute;
use types::LOCALSTORAGE_KEY;

pub struct Navtop {
    dispatch: AppDispatch,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Logout,
}

impl Component for Navtop {
    type Message = Msg;
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navtop {
            dispatch,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Logout => {
                ConsoleService::info("logout");

                // RESET REDUCER
                self.dispatch.send(DataAccountAction::Logout);
                
                // REMOVE LOCALSTORAGE
                let mut storage = StorageService::new(Area::Local).expect("storage was disabled");
                storage.remove(LOCALSTORAGE_KEY);
                
                false
            }
        }
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        html! {
            <div class="d-flex justify-content-between px-4 py-2 bg-dark"
                style="font-size: 14px; height: 64px;"
            >
                <ul class="nav text-light">
                    <li class="nav-item justify-content-center my-auto">
                        <div class="me-3">
                            <Logo width=40 />
                        </div>
                        // <div class="bg-white p-1 pt-0 rounded me-3 navtop-logo">
                        //     <img
                        //         src="https://i.stack.imgur.com/3Stuq.png"
                        //         style="width: 23px;"
                        //     />
                        // </div>
                    </li>
                    <div
                        class="nav-item justify-content-center my-auto bg-light me-3"
                        style="width: 1px; height: 24px; opacity: .8;"
                    >
                    </div>
                    <li class="nav-item px-2"
                        style="min-width: 64px;"
                    >
                        <p class="fw-bolder mb-1">{"user-asdfbd"}</p>
                        <span>{"Development"}</span>
                    </li>
                </ul>
                <ul class="nav justify-content-end"
                    style="flex: 1 1 0%;"
                >
                    <li class="nav-item my-auto me-3">
                        <button type="button" class="btn btn-dark navtop-hover">
                            <i class="bi bi-search"></i>
                        </button>
                    </li>
                    <li class="nav-item my-auto me-3">
                        <button type="button" class="btn btn-outline-light btn-sm">{"Discuss your needs"}</button>
                    </li>
                    <li class="nav-item my-auto me-3">
                        <button type="button" class="btn btn-dark navtop-hover">
                            <i class="bi bi-book me-2"></i> {"Docs"}
                        </button>
                    </li>
                    <li class="nav-item my-auto me-3">
                        <button type="button" class="btn btn-dark navtop-hover">
                            <i class="bi bi-bell"></i>
                        </button>
                    </li>
                    <li class="nav-item justify-content-center my-auto">
                        <div
                            style="cursor: pointer;"
                            class="bg-white rounded-circle p-1 navtop-logo dropdown">
                            <img
                                src="https://cdn0.iconfinder.com/data/icons/set-ui-app-android/32/8-512.png"
                                style="width: 23px;"
                                id="dropdownMenuButton1"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            />
                            <ul
                                class="dropdown-menu p-3"
                                style="font-size: 14px;"
                                aria-labelledby="dropdownMenuButton1"
                            >
                                <li
                                    class="d-flex px-1"
                                >
                                    <div
                                        class="d-flex border-bottom border-1 py-2"
                                        // style="width: 40%;"
                                    >
                                        <div
                                            style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                                            class="d-flex justify-content-center align-items-center rounded-circle me-3"
                                        >
                                            // <img
                                            //     src={"https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg"} style=" color: transparent;
                                            //     width: 100%;
                                            //     height: 100%;
                                            //     object-fit: cover;
                                            //     text-align: center;
                                            //     text-indent: 10000px;"
                                            // />
                                            <i class="bi bi-person-circle fs-5"></i>
                                        </div>

                                        <div
                                            class="d-block"
                                            style="min-width: 40px;white-space:nowrap;"
                                        >
                                            <p
                                                class="mb-0"
                                            >
                                                { "indo halim" }
                                            </p>
                                            // <Anchor route=AppRoute::ApplicationSettings { tenant_id: tenant_id.clone(), app_id: app.client_id.clone() } >
                                            //     <a
                                            //         class="fw-bold mb-0"
                                            //         style=" white-space: nowrap;
                                            //                 text-overflow: ellipsis;
                                            //                 overflow: hidden;
                                            //                 font-size: 14px;
                                            //                 text-decoration: none;" 
                                            //         href="#">
                                            //         { &app.name }
                                            //     </a>
                                            // </Anchor>
                                            <p
                                                class="mb-0 text-muted"
                                                style=" white-space: nowrap;
                                                        text-overflow: ellipsis;
                                                        overflow: hidden;
                                                        font-size: 14px;"
                                            >
                                                { "mde50526@gmail.com" }
                                            </p>
                                        </div>
                                    </div>
                                </li>

                                <li
                                    class="pt-2 px-1"
                                >  
                                    <div class="border-bottom border-1 pb-2">
                                        <div
                                            class="list-hover dropdown-item py-1 px-2 rounded align-middle"
                                            style="width: calc(100% + 18px);margin-left: -9px;"
                                        >
                                            <Anchor
                                                route=AppRoute::Profile { tenant_id: String::from("id-tenant-id") }
                                                classes="text-decoration-none text-color-primary"
                                            >
                                                <i class="bi bi-person me-3 fs-5"></i>
                                                <span
                                                    style="vertical-align: text-bottom;"
                                                >
                                                    {"Your profile"}
                                                </span>
                                            </Anchor>
                                        </div>
                                    </div>
                                </li>

                                <li
                                    onclick=self.link.callback(|_| Msg::Logout)
                                    class="pt-2 px-1"
                                >
                                    <div>
                                        <div
                                            class="list-hover dropdown-item py-1 px-2 rounded align-middle"
                                            style="width: calc(100% + 18px);margin-left: -9px;"
                                        >
                                            <i class="bi bi-box-arrow-right me-3 fs-5"></i>
                                            <span
                                                style="vertical-align: text-bottom;"
                                            >
                                                {"Log out"}
                                            </span>
                                        </div>
                                    </div>
                                </li>
                            </ul>
                        </div>
                    </li>
                </ul>
            </div>
        }
    }
}
