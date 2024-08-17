use crate::monthlist::MonthList;
use crate::reminder::ReminderProps;
use gloo_net::http::Request;
use monthlist::MonthListProps;
use yew::prelude::*;

mod monthlist;
mod reminder;

const SERVER_URL: &str = "https://"; // CHANGE TO TAKE ARGUMENTS OR ENV

#[function_component]
fn App() -> Html {
    let hook = use_state(|| None);
    {
        let hook = hook.clone();
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get(SERVER_URL)
                    .send()
                    .await
                    .expect("failed to initalize months");
                let data: MonthListProps = response
                    .json()
                    .await
                    .expect("deserialization of months failed");
                hook.set(Some(data));
            })
        })
    }

    let props = (*hook).clone().expect("none got from response");

    html! {
        <MonthList ..props/>
    }
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
