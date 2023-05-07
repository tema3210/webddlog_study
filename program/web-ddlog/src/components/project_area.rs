use yew::prelude::*;
use yewdux::prelude::*;
use stylist::yew::styled_component;

use crate::{store::State};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(ProjectArea)]
pub fn project_area(props: &Props) -> Html {
    let (state, _) = use_store::<State>();
    
    html! {
        <div class={"row"}>
            <p class={css!("color: red;")}>{"header "} {state.get_count()} </p>
        </div>
    }
}