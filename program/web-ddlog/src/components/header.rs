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
        <div class={"row"}>
            <p class={css!("color: red;")}>{"Web DDlog interpreter"} </p>
        </div>
    }
}