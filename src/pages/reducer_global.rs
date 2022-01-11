// use yew::prelude::*;
// use yewdux::prelude::*;
// use yewtil::NeqAssign;
// // use yew::services::ConsoleService;

// use store::reducer::{
//     AppDispatch,
//     Action,
//     Counter
// };

// pub struct ReducerGlobal {
//     dispatch: AppDispatch,
// }

// impl Component for ReducerGlobal {
//     type Message = ();
//     type Properties = AppDispatch;

//     fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
//         // Magically increment counter for this example.
//         dispatch.send(Action::Increment);

//         Self { dispatch }
//     }

//     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
//         false
//     }

//     fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
//         self.dispatch.neq_assign(dispatch)
//     }

//     fn view(&self) -> Html {
//         let count = self.dispatch.state().count;
//         let name = self.dispatch.state().name.clone();
//         let increment = self.dispatch.callback(|_| Action::Increment);
//         let update = self.dispatch.callback(|_| {
//             // ConsoleService::info(&data.name);
//             let newdata = Counter {
//                 count: 0,
//                 name: String::from("Batman")
//             };
//             Action::Update(newdata)
//         });
//         html! {
//             <>
//                 // <p>{"Show Reducer State"}</p>
//                 <h1>{ count }</h1>
//                 <h1>{ name }</h1>
//                 <button onclick=increment>{"+1"}</button>
//                 <button onclick=update>{"update"}</button>
//             </>
//         }
//     }
// }