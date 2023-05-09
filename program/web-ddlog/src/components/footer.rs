use yew::prelude::*;
use yewdux::prelude::*;
use stylist::yew::styled_component;

use crate::{store::State};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(Footer)]
pub fn footer(props: &Props) -> Html {
    let (state, _) = use_store::<State>();
    
    html! {
        <footer>
            <div class={"row"}>
                <p class={css!("color: red;")}>{"footer "} </p>
            </div>
        </footer>
        
    }
}