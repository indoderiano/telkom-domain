use yew::prelude::*;
use crate::components::{
    // logo::Logo,
    logo2::Logo2,
};
use yewdux::dispatch::Dispatcher;
use crate::store::reducer_account::{
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
// use crate::types::LOCALSTORAGE_KEY;
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
        html! {
            <div class="d-flex justify-content-between px-4 py-2 bg-dark"
                style="font-size: 14px; height: 64px;"
            >
                <ul class="nav text-light">
                    <li class="nav-item justify-content-center my-auto">
                        <div class="me-3">
                            <Logo2 width=40 />
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
                            <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                                <li
                                    onclick=self.link.callback(|_| Msg::Logout)
                                    class="dropdown-item fs-7"
                                    style="cursor: pointer;"
                                >
                                    {"Log out"}
                                </li>
                            </ul>
                        </div>
                    </li>
                </ul>
            </div>
        }
    }
}
