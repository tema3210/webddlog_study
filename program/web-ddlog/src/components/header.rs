use yew::prelude::*;
use yewdux::prelude::*;
use stylist::yew::styled_component;

use crate::{store::State};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(Header)]
pub fn header(props: &Props) -> Html {
    let (state, _) = use_store::<State>();
    
    html! {
        <div class={"row"}>
            <p class={css!("color: red;")}>{"Home "} {state.get_count()} </p>
        </div>
    }
}