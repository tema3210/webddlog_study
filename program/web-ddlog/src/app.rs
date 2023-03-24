use yew::prelude::*;
use yewdux::prelude::*;

use crate::{store::State, components::example::Home};

#[function_component(App)]
pub fn app() -> Html {
    let (state, dispatch) = use_store::<State>();
    let onclick_add = dispatch.reduce_mut_callback(State::inc);
    let onclick_minus = dispatch.reduce_mut_callback(State::dec);

    html! {
        <div class={"container"}>
            <p> {"Hello "}{ state.get_count() }</p>
            <div class={"row"}>
                <button class={"col btn btn-primary"} onclick={onclick_add}>{"+1"}</button>
                <button class={"col btn"} onclick={onclick_minus}>{"-1"}</button>
                <button class={"col btn"} onclick={|ev| log::info!("{:?}",ev)}>{"oooo"}</button>
            </div>
            <Home />
        </div>
    }
}