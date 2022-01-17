use yew::{
    function_component, 
    events::Event,
};
use yew::prelude::*;


use web_sys::{console, EventTarget, HtmlInputElement};

use wasm_bindgen::{JsValue, JsCast};

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub value: Callback<String>,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {

    let target_input_value = |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input.value()
    };

    let on_change = {
        let cb = props.value.clone();
        move |e: Event| cb.emit(target_input_value(e))
    };

    html! {
        <input
            class={classes!("todoInput")} 
            type="text" 
            placeholder="Igresa el titulo"
            onchange={on_change}
        />
    }
}