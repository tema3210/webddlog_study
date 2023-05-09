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

    let tabs = state.list_modules().into_iter().map(|s| {
        let s2 = s.clone();
        let cb = dispatch.reduce_mut_callback(move |state| state.set_current_module(&s));
        html! {
            <li class="nav-item">
                <a class="nav-link active" aria-current="page" onclick={&cb}>{s2}</a>
            </li>
        }
    });

    let new_cb = dispatch.reduce_mut_callback(|state| {
        state.add_empty_module("new")
    });

    
    html! {
        <div>
            <ul class="nav">
                {for tabs}
                <li class="nav-item" >
                    <a class="nav-link active" onclick={&new_cb}>{"+"}</a>
                </li>
            </ul>
            if let Some(tab) = state.get_current_module() { <Editor name={tab} /> }
        </div>
    }
}