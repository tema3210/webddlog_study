use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*};
use yewdux::prelude::*;

use crate::{Store, components::home::Home, lang::lexer::lex};

#[function_component(App)]
pub fn app() -> Html {
    let (state, dispatch) = use_store::<Store>();
    let lexed: Vec<_> = lex(
        &state.get_program()
    ).collect();
    let onclick_add = dispatch.reduce_mut_callback(Store::inc);
    let onclick_minus = dispatch.reduce_mut_callback(Store::dec);

    let onchange = Callback::from(move |ev: Event| {
        ev.target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            .map(
                |input| dispatch.reduce_mut(
                    |state| state.set_program(input.value())
                )
            );
    });

    html! {
        <div class={"container"}>
            <div class={"row"}>
                <button class={"col btn btn-primary"} onclick={onclick_add}>{"+1"}</button>
                <button class={"col btn"} onclick={onclick_minus}>{"-1"}</button>
                <button class={"col btn"} onclick={|ev| log::info!("{:?}",ev)}>{"oooo"}</button>
                <input type={"text"} onchange={onchange} value={state.get_program()} />
            </div>
            {format!("{:?}", lexed)}
            <Home />
        </div>
    }
}