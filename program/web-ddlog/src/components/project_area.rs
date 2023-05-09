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

    let new_name = use_state(|| -> Option<String>{ None });
    let new_name2 = new_name.clone();
    let new_name3 = new_name.clone();
    let new_name4 = new_name.clone();
    let new_name5 = new_name.clone();

    let new_cb = dispatch.reduce_mut_callback(move |state| {
        if let Some(name) = &*new_name4 {
            state.add_empty_module(&name);
            new_name5.set(None)
        }
    });

     
    html! {
        <div>
            <ul class="nav">
                {for tabs}
                <li class="nav-item" >
                    if new_name.is_none() {
                        <a class="nav-link active" onclick={move |_| new_name3.set(Some(String::new()))}>{"+"}</a>
                    }
                    if let Some(name) = &*new_name {
                        <div>
                            <input type="text" value={name.clone()} onchange={
                                move |ev: Event| {ev.target().and_then(
                                    |trg| trg.dyn_into::<HtmlInputElement>().ok()
                                ).map(
                                    |input| new_name2.set(Some(input.value()))
                                );}
                            }/>
                            <button onclick={&new_cb}>{"+"}</button>
                            <button onclick={move |_| new_name.set(None)}>{"-"}</button>
                        </div>
                    }
                </li>
            </ul>
            if let Some(tab) = state.get_current_module() { <Editor name={tab} /> }
        </div>
    }
}