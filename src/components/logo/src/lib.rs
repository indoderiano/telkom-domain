use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct LogoState {
    #[prop_or(70)]
    pub width: u16,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Logo {
    width: u16,
}

pub enum Msg {}

impl Component for Logo {
    type Message = Msg;
    type Properties = LogoState;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Logo {
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
                        display: inline-block;
                        padding: 0px;
                        border-radius: {}px;
                        background: white;
                        border: {}px solid rgba(0,0,0,.78);
                        overflow: hidden;
                        position: relative;
                    ", self.width, self.width, self.width, 0/22)
            >
                <img
                    src="/assets/logo/shield.png"
                    style=format!("
                        width: {}px;
                        position: absolute;
                        top: 53%;
                        left: 50%;
                        transform: translate(-50%,-50%);
                        opacity: .9;
                    ", self.width*19/20)
                />
                <img
                    src="/assets/logo/lock3.png"
                    style=format!("
                        width: {}px;
                        position: absolute;
                        top: 55%;
                        left: 50%;
                        transform: translate(-50%,-50%);
                        filter: grayscale(100%);
                        opacity: 1;
                    ", self.width*4/10)
                />
                <div
                    style=format!("
                        width: {}px;
                        height: {}px;
                        position: absolute;
                        bottom: 50%;
                        left:50%;
                        transform-origin: 0% 100%;
                        transform: rotate(-90deg) translate(-50%,0);
                        background: rgba(0,0,0,.0);
                    ", self.width*5/4, self.width*5/4)
                />
            </div>
        }
    }
}