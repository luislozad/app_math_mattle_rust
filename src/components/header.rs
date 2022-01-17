use yew::prelude::*;

use crate::types::Rate;

#[derive(PartialEq, Properties, Clone)]
pub struct PropsHeader {
    pub rate: Rate,
}

#[function_component(Header)]
pub fn header(props: &PropsHeader) -> Html {
    html! {
        <div class={classes!("playHeader")}>
            <div class={classes!("playHeader__content")}>
                {"ssss"}
            </div>
        </div> 
     }
}