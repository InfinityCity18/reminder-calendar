use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::SERVER_URL;

#[derive(PartialEq, Clone, Deserialize, Serialize, Properties)]
pub struct ReminderProps {
    pub name: String,
    pub month: u8,
    pub day: u8,
    pub checked: bool,
}

#[function_component]
pub fn Reminder(props: &ReminderProps) -> Html {
    let is_checked = use_state(|| props.checked);
    let props_clone = props.clone();

    let on_checking = Callback::from(move |_| {
        let is_checked_clone = is_checked.clone();
        let reminder_month = props_clone.month.clone();
        let reminder_name = props_clone.name.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let post_json = CheckboxPostData {
                checked: *is_checked_clone,
                reminder_month,
                reminder_name,
            };

            let post_json = serde_json::to_value(post_json).unwrap();

            Request::post(SERVER_URL)
                .json(&post_json)
                .expect("sending checkbox state failed");
        });
    });

    html! {
        <div>
        <label for={props.name.clone()}>{props.name.clone()}</label>
      <input onclick={on_checking.clone()} type="checkbox" name={props.name.clone()} id={props.name.clone()} checked={props.checked} />
        </div>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct CheckboxPostData {
    checked: bool,
    reminder_name: String,
    reminder_month: u8,
}
