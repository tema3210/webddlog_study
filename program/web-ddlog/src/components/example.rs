use yew::prelude::*;
use yewdux::prelude::*;
use stylist::yew::styled_component;

use crate::store::State;

#[styled_component(Home)]
pub fn home() -> Html {
    let (state, _) = use_store::<State>();
    html! {
        <p class={css!("color: red;")}>{"Home "} {state.get_count()}</p>
    }
}