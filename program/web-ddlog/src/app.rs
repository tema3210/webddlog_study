use yew::prelude::*;
use yewdux::prelude::*;

use crate::{Store, components::home::Home, lang::lexer::lex};

#[function_component(App)]
pub fn app() -> Html {
    let (state, dispatch) = use_store::<Store>();
    let lexed: Vec<_> = lex("asdadsa + 11.21 + as* true + -22 -22.7").collect();
    let onclick_add = dispatch.reduce_mut_callback(Store::inc);
    let onclick_minus = dispatch.reduce_mut_callback(Store::dec);

    html! {
        <div class={"container"}>
            <p> {"Hello "}{ state.get_count() }</p>
            <div class={"row"}>
                <button class={"col btn btn-primary"} onclick={onclick_add}>{"+1"}</button>
                <button class={"col btn"} onclick={onclick_minus}>{"-1"}</button>
                <button class={"col btn"} onclick={|ev| log::info!("{:?}",ev)}>{"oooo"}</button>
            </div>
            {format!("{:?}", lexed)}
            <Home />
        </div>
    }
}