use yew::{
    function_component,
};
use yew::prelude::*;

#[path = "./btn_primary.rs"]
mod btn_primary;
#[path = "./btn_secondary.rs"]
mod btn_secondary;

use btn_primary::{ ButtonPrimary };
use btn_secondary::{ ButtonSecondary };

// #[derive(PartialEq, Properties, Clone)]
// pub struct PropsHome {
//     pub children: Children,
// }

#[function_component(Title)]
fn title() -> Html {
    html! {
        <div class={classes!("item-center", "margin-b-x1", "margin-t-x1")}>
            <p 
                class={classes!("text_center", "text_title", "max-width-child", "text_line-height-title")}>
                { "Math Battle" }
            </p>
        </div>
    }
}

#[function_component(ButtonTop)]
fn btn_top() -> Html {
    html! {
        <div class={classes!("item-center")}>
            <div class={classes!("max-width-child")}>
                <ButtonPrimary text={"Daily challenge"} onclick={Callback::from(|_| {})} />
            </div>
        </div>
    }
}

#[function_component(ButtonClassic)]
fn btn_classic() -> Html {
    html! {
        <div class={classes!("item-center", "margin-b-x1:2", "margin-t-x3")}>
            <div class={classes!("max-width-child")}>
                <ButtonSecondary text={"Classic"} onclick={Callback::from(|_| {})} />
            </div>
        </div>
    }
}

#[function_component(ButtonAdvanced)]
fn btn_advanced() -> Html {
    html! {
        <div class={classes!("item-center", "margin-b-x1:2")}>
            <div class={classes!("max-width-child")}>
                <ButtonSecondary text={"Advanced"} onclick={Callback::from(|_| {})} />
            </div>
        </div>
    }
}

#[function_component(ButtonProfessional)]
fn btn_professional() -> Html {
    html! {
        <div class={classes!("item-center")}>
            <div class={classes!("max-width-child")}>
                <ButtonSecondary text={"Professional"} onclick={Callback::from(|_| {})} />
            </div>
        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={classes!("playHome")}>
            <Title />
            <ButtonTop />
            <ButtonClassic />
            <ButtonAdvanced />
            <ButtonProfessional />
        </div> 
     }
}