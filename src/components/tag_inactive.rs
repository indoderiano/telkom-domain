use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TagInactive {}

pub enum Msg {}

impl Component for TagInactive {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TagInactive {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <span
                class="position-absolute top-0 translate-middle-y badge rounded-pill bg-warning"
                style="left: 10px;"    
            >
                { "Inactive" }
            </span>
        }
    }
}
