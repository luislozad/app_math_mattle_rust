use yew::{
    functional::use_state,
};

use yew::prelude::*;

mod components;
mod types;

use components::{ 
    Container, 
    Content, 
    Home,
    Header,
};

use types::Rate;

type TodoList = Vec<TodoItem>;

#[derive(Default)]
struct TodoItem {
    text: String,
    completed: bool,
}

#[function_component(App)]
fn app() -> Html {
    // let state_input = use_state(|| String::from(""));

    // let handle_input = {
    //     let state = state_input.clone();
    //     Callback::from(move |title: String| state.set(title))
    // };

    let rate = Rate{ pt: 0.0, hidden: false };

    html! {
        <Container>
            <Content>
                <Header {rate} />
                <Home />
            </Content>
        </Container>
    }
}

fn main() {
    yew::start_app::<App>();
}