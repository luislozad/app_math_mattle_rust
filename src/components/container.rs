use yew::{classes, html, Children, Component, Html, Properties, Context};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub children: Children,
}

pub struct Container;

impl Component for Container {
    type Message = ();
    type Properties = Props;
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("container")}>
                { ctx.props().children.clone() }
            </div>
        }
     }
}