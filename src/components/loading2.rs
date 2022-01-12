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
                class="position-relative"
            >
                <div
                    style=format!("
                        width: {}px;
                        height: {}px;
                        padding: 0px;
                        position: absolute;
                        top: 50%;
                        left: 50%;
                        transform: translate(-50%,-50%);
                        position: relative;
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
                        src="/assets/logo/lock3.png"
                        style=format!("
                            width: {}px;
                            position: absolute;
                            top: calc(50% + {}px);
                            left: calc(50% + {}px);
                            transform: translate(-50%,-50%);
                            opacity: .5;
                        ", self.width*5/10, 0/22, 0/22)
                    />
                </div>
            </div>
        }
    }
}