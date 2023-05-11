use yew::prelude::*;
use yewdux::prelude::*;
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*};


use crate::{Store, lang::lexer::lex, components::editor::Editor, validation};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(ProjectArea)]
pub fn project_area(props: &Props) -> Html {
    let (state, dispatch) = use_store::<Store>();

    let tabs = state.list_modules().into_iter().map(|s| {
        let s2 = s.clone();
        let cb = dispatch.reduce_mut_callback(move |state| state.set_current_module(&s));
        html! {
            <li class="nav-item">
                <a class="nav-link active" aria-current="page" onclick={&cb}>{s2}</a>
            </li>
        }
    });

    let new_name = use_state(|| -> Option<Result<String,(String,validation::project_area::Error)>>{ None });
    
    let new_name4 = new_name.clone();
    let new_cb = dispatch.reduce_mut_callback(move |state| {
        if let Some(Ok(name)) = &*new_name4 {
            state.add_empty_module(&name);
            new_name4.set(None)
        }
    });

     
    html! {
        <div>
            <ul class="nav">
                {for tabs}
                <li class="nav-item" >
                    if new_name.is_none() {
                        <a class="nav-link active" onclick={
                            let new_name = new_name.clone();
                            move |_| new_name.set(Some(Ok(String::new())))
                        }>{"+"}</a>
                    }
                    if let Some(name) = &*new_name {
                        <p>
                            <input type="text" 
                                value={
                                    match name {
                                        Ok(s) => s.clone(),
                                        Err((s,_)) => s.clone()
                                    }
                                } 
                                oninput={
                                    let new_name = new_name.clone();
                                    move |ev: InputEvent| {ev.target().and_then(
                                        |trg| trg.dyn_into::<HtmlInputElement>().ok()
                                    ).map(
                                        |input| {
                                            let val = input.value();
                                            if let Some(err) = validation::project_area::validate(&val) {
                                                new_name.set(Some(Err((val,err))))
                                            } else {
                                                new_name.set(Some(Ok(val)))
                                            }
                                        }
                                    );}
                                }
                            />
                            if let Err((_,err)) = name {
                                <span>{format!("{:?}",err)}</span>
                            }
                            <button class="btn" onclick={&new_cb}>{"+"}</button>
                            <button class="btn" onclick={move |_| new_name.set(None)}>{"-"}</button>
                        </p>
                    }
                </li>
            </ul>
            if let Some(tab) = state.get_current_module() { <Editor name={tab} /> }
        </div>
    }
}