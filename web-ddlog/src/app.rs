use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*};

use crate::{
    components::header::Header, 
    components::footer::Footer,
    components::menu::Menu,
    components::interaction::Interactions,
    components::project_area::ProjectArea
};

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <div class={"position-relative"}>
            <Header />
            <div class={css!("
                margin-top: 2rem;
                max-height: calc(100% - 4rem);
            ")}>
                <div class={"container"}>
                    <div class={"row"}>
                        <div class={"col-3"}>
                            <Menu />
                        </div>
                        <div class={"col-6"}>
                            <ProjectArea />
                        </div>
                        <div class={"col-3"}>
                            <Interactions />
                        </div>
                    </div>
                </div>
            </div>
            <Footer />
        </div>
        
    }
}