use yew::prelude::*;

mod monthlist;
mod reminder;

const SERVER_URL: &str = "https://"; // CHANGE TO TAKE ARGUMENTS OR ENV

#[function_component]
fn App() -> Html {
    html! {<button></button>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}

//let reminders = use_state(|| vec![]);
//
//    let reminders_clone = reminders.clone();
//    let reminders = wasm_bindgen_futures::spawn_local(async move {
//        let fetched_reminders: Vec<Reminder> = Request::get(SERVER_URL)
//            .send()
//            .await
//            .unwrap()
//            .json()
//            .await
//            .unwrap();
//        reminders_clone.set(fetched_reminders);
//    });
