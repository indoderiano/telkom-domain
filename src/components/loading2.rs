use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct LoadingState {
    #[prop_or(70)]
    pub width: u16,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Loading2 {
    width: u16,
}

pub enum Msg {}

impl Component for Loading2 {
    type Message = Msg;
    type Properties = LoadingState;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Loading2 {
            width: props.width,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.width == props.width {
            false
        } else {
            self.width = props.width;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <div
                style=format!("
                    width: {}px;
                    height: {}px;
                    position: absolute;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%,-50%);
                ", self.width, self.width)
            >
                <div
                    style=format!("
                        width: {}px;
                        height: {}px;
                        border: {}px solid rgba(0,0,0,.2);
                        border-radius: 50%;
                        border-top: {}px solid rgba(0,0,0,.6);
                        -webkit-animation: spin .75s linear infinite;
                        animation: spin .75s linear infinite;
                    ", self.width, self.width, self.width/22, self.width/22)
                />
                <img
                    src="/assets/logo/lock.png"
                    style=format!("
                        width: {}px;
                        position: absolute;
                        top: 50%;
                        left: 50%;
                        transform: translate(-50%,-50%);
                        opacity: .3;
                    ", self.width*9/10)
                />
            </div>
        }
    }
}