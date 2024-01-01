use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, EventTarget, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::{prelude::*, dispatch};
use stylist::yew::styled_component;

use crate::{store::{State, Program}, lang::lexer::lex};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String
}

#[styled_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let (state, dispatch) = use_store::<State>();

    let name = props.name.clone();
    let cb = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        e.target().and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok()).map(|input| {
            state.update_program(&name, |_| Program::from_string(&input.value()));
        });
    });

    if let Some(program) = state.get_program(&props.name) {
        html! {
            <div class={"row"}>
                <textarea name={props.name.clone()} rows="4" cols="50" oninput={cb} value={program.get_text()} />
                <p>{
                    format!("{:?}",lex(&program.get_text()).collect::<Vec<_>>())
                }</p>
            </div>
        }
    } else {
        html! {
            <p></p>
        }
    }
}