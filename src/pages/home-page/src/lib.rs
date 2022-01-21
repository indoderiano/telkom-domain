use yew::prelude::*;
use yewdux::prelude::WithDispatch;
use landing_page_navtop::LandingPageNavTop;
use yew::html::Classes;
use css_in_rust::Style;

pub struct HomePage {
    style: Style,
}

pub fn parser(style: Style) -> Classes {
    Classes::from(style.to_string())
}

pub enum Msg {}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = Style::create("home_page", include_str!("home_page.scss"))
            .expect("An error occured while creating the style.");
        HomePage {
            style
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
            <div class=parser(self.style.clone())>
                <WithDispatch<LandingPageNavTop>/>
                <div class="container">
                    <div class="row">
                        <div class="col">
                            <div class="col-content">
                                {"We're protecting your digital life"}
                            </div>
                            <div class="col-content">
                                {"The cybersecurity at your hand itself"}
                            </div>
                        </div>
                        <div class="col">
                            <div class="landing-page-logo">
                            <a
                                class="app-logo"
                                target="_blank" 
                            >
                            </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
