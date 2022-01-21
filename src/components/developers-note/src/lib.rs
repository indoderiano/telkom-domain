use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Note {
    #[prop_or(String::from("Create your note warnings here"))]
    pub message: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct DevelopersNote {
    message: String,
}

pub enum Msg {}

impl Component for DevelopersNote {
    type Message = Msg;
    type Properties = Note;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        DevelopersNote {
            message: props.message,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.message == props.message {
            false
        } else {
            self.message = props.message;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <div
                class="pt-2"
            >
                <div
                    class="alert alert-warning position-relative"
                    role="alert"
                    style="font-size: 14px;"
                >
                    <i
                        class="bi bi-exclamation-triangle-fill me-2"
                        style="font-size: 18px;"    
                    />
                    { self.message.clone() }
                    <span
                        class="position-absolute top-0 translate-middle-y badge rounded-pill bg-warning"
                        style="left: 10px;"    
                    >
                        { "For Developers" }
                    </span>
                </div>
            </div>
        }
    }
}