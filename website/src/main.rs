use crate::monthlist::MonthList;
use gloo_net::http::Request;
use log::{debug, info};
use monthlist::MonthListProps;
use yew::prelude::*;

mod monthlist;
mod reminder;

const SERVER_URL: &str = "http://10.21.37.100:2137"; // CHANGE TO TAKE ARGUMENTS OR ENV

#[function_component]
fn App() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    info!("TEST");
    let hook = use_state(|| MonthListProps {
        months_reminders: Vec::new(),
        months_names: Vec::new(),
    });
    {
        let hook = hook.clone();
        use_effect_with((), move |_| {
            let hook = hook.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get(&(SERVER_URL.to_string() + "/reminders"))
                    .send()
                    .await
                    .expect("failed to initalize months");
                debug!("help me : ");
                let data: MonthListProps = response
                    .json()
                    .await
                    .expect("deserialization of months failed");
                debug!("DATA : {:?}", &data);
                hook.set(data);
                debug!("WHAT");
            });
            || ()
        });
    }
    debug!("HOOK : {:?}", &hook);
    html! {
        <MonthList ..(*hook).clone()/>
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
