use yew::prelude::*;

mod monthlist;
mod remainder;

const SERVER_URL: &str = "https://"; // CHANGE TO TAKE ARGUMENTS OR ENV

#[function_component]
fn App() -> Html {
    html! {<button></button>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
