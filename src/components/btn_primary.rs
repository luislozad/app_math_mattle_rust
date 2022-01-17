use yew::{
    function_component,
};
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct PropsButtonPrimary {
    pub text: String,
    pub onclick: Callback<()>,
}

#[function_component(ButtonPrimary)]
pub fn button_primary(props: &PropsButtonPrimary) -> Html {
    let onclick = {
        let handle = props.onclick.clone();
        move |_| handle.emit(())
    };

    html! {
        <button 
            class={classes!("playBtnPrimary", "text_light", "effect-btn-click")}
            onclick={onclick}
        >
            { props.text.clone() }
        </button>
     }
}