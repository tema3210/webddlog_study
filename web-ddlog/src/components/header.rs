use yew::prelude::*;
use yewdux::prelude::*;
use stylist::yew::styled_component;

use crate::{store::State};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(Header)]
pub fn header(props: &Props) -> Html {
    html! {
        <div class={"fixed-top"}>
            <div class={css!("background-color: gray;")}>
                <p class={css!("color: lightgreen;")}>
                    <span class={css!("
                        font-size: 1.3rem;
                        padding-left: 2rem;
                    ")}>
                        {"Web DDlog interpreter"} 
                    </span>
                </p>
            </div>
        </div>
    } 
}