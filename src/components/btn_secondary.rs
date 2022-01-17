use yew::{
    function_component,
};
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct PropsButtonSecondary {
    pub text: String,
    pub onclick: Callback<()>,
}

#[function_component(ButtonSecondary)]
pub fn button_secondary(props: &PropsButtonSecondary) -> Html {
    let onclick = {
        let handle = props.onclick.clone();
        move |_| handle.emit(())
    };
    
    html! {
        <button 
            class={classes!("playBtnSecondary", "text_head", "effect-btn-click", "text_line-height-child")}
            onclick={onclick}
        >
            { props.text.clone() }
        </button>
     }
}