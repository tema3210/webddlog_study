use yew::prelude::*;
use yewdux::prelude::*;
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*};


use crate::{Store, lang::lexer::lex, components::editor::Editor};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(ProjectArea)]
pub fn project_area(props: &Props) -> Html {
    let (state, dispatch) = use_store::<Store>();

    let tabs = state.list_tabs().into_iter().map(|s| {
        let cb = dispatch.reduce_mut_callback(move |state| state.set_current_tab(s.clone()));
        html! {
            <li class="nav-item">
                <a class="nav-link active" aria-current="page" onclick={&cb}>{"Active"}</a>
            </li>
        }
    });

    let tab = state.get_current_tab();

    
    html! {
        <div>
            <ul class="nav">
                {for tabs}
                <li class="nav-item" >
                    <a class="nav-link active" >{"+"}</a>
                </li>
            </ul>
            <Editor tab={tab}/>
        </div>
    }
}