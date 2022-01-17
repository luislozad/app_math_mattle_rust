use yew::{
    function_component,
};
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub children: Children,
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    html! {
       <div class={classes!("playContent")}>
        { props.children.clone() }
       </div> 
    }
}