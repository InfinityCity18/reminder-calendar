use crate::monthlist::MonthList;
use gloo_net::http::Request;
use log::{debug, info};
use monthlist::MonthListProps;
use yew::prelude::*;

mod monthlist;
mod reminder;

const SERVER_URL: &str = "http://localhost:2137/reminders"; // CHANGE TO TAKE ARGUMENTS OR ENV

#[function_component]
fn App() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    info!("TEST");
    let hook = use_state(|| None);
    {
        let hook = hook.clone();
        info!("TEST");
        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::get(SERVER_URL)
                .send()
                .await
                .expect("failed to initalize months");
            debug!("help me : {:?}", response.text().await);
            let data: MonthListProps = response
                .json()
                .await
                .expect("deserialization of months failed");
            hook.set(Some(data));
        });
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
