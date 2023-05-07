use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*};
use yewdux::prelude::*;

use crate::{
    components::header::Header, 
    components::footer::Footer,
    components::menu::Menu,
    components::interaction::Interactions,
    components::project_area::ProjectArea
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={"container"}>
            <div class={"row"}>
                <Header />
            </div>
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
            <div class={"row"}>
                <Footer />
            </div>
        </div>
    }
}