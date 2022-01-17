use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct PropsProgressBar {
    pub total: f64,
}

#[function_component(ProgressBar)]
pub fn progress_bar(props: &PropsProgressBar) -> Html {
    html! {
        <div class={classes!("playProgressBar")}>
        </div> 
     }    
}