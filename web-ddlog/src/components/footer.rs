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
        <footer class={"fixed-bottom"}>
            <div>
                <p class={css!("color: black;")}>
                    <span class={css!("
                        font-size: 1.3rem;
                        padding-left: 2rem;
                    ")}>
                        {"
                            No rights reserved
                        "} 
                    </span>
                </p>
            </div>
        </footer>
        
    }
}