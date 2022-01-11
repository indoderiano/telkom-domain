// use yew::prelude::*;
// use yewdux::prelude::*;
// use yewtil::NeqAssign;
// // use yew::services::ConsoleService;

// use store::reducer_account::{
//     AppDispatch,
//     DataAccountAction,
//     // DataAccount,
// };
// use types::{
//     ResponseLogin,
// };

// pub struct ReducerAccountView {
//     dispatch: AppDispatch,
// }

// impl Component for ReducerAccountView {
//     type Message = ();
//     type Properties = AppDispatch;

//     fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
//         Self { dispatch }
//     }

//     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
//         false
//     }

//     fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
//         self.dispatch.neq_assign(dispatch)
//     }

//     fn view(&self) -> Html {
//         let name = self.dispatch.state().username.clone();
//         let update = self.dispatch.callback(|_| {
//             // ConsoleService::info(&data.name);
//             let newdata = ResponseLogin {
//                 username: String::from("Batman"),
//                 email: String::from("bat@mail.com"),
//                 token: String::from("testingtoken"),
//             };
//             DataAccountAction::Update(newdata)
//         });
//         if let Some(ref namestr) = name {
//             html! {
//                 <>
//                     <p>{"Show Reducer Account"}</p>
//                     <h1>{namestr}</h1>
//                     <button onclick=update>{"update"}</button>
//                 </>
//             }
//         } else {
//             html! {
//                 <>
//                     <p>{"Show Reducer Account"}</p>
//                     <h1>{"not logged in"}</h1>
//                     <button onclick=update>{"update"}</button>
//                 </>
//             }
//         }
//     }
// }