use gloo_net::http::Request;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::SERVER_URL;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Properties)]
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
    let is_checked_clone = is_checked.clone();
    info!("BEFORE SETTING {:?}", *is_checked);

    let on_checking = Callback::from(move |_| {
        is_checked_clone.set(!(*is_checked_clone));
        info!("AFTER SETTING {:?}", *is_checked_clone);
        let reminder_month = props_clone.month.clone();
        let reminder_name = props_clone.name.clone();
        let is_checked_clone_async = is_checked_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let post_json = CheckboxPostData {
                checked: !*is_checked_clone_async,
                reminder_month,
                reminder_name,
            };
            debug!("HI {:?}", &post_json);

            let post_json = serde_json::to_value(post_json).unwrap();

            Request::post(&(SERVER_URL.to_owned() + "/checkbox"))
                .json(&post_json)
                .expect("sending checkbox state failed")
                .send()
                .await
                .unwrap();
        });
    });

    html! {
        <div class="reminder">
        <label for={props.name.clone() + &props.month.to_string()}>{props.day.clone()}{" - "}{props.name.clone()}</label>
      <input class="remindercheckbox" onclick={on_checking.clone()} type="checkbox" name={props.name.clone()} id={props.name.clone() + &props.month.to_string()} checked={*is_checked} />
        </div>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct CheckboxPostData {
    checked: bool,
    reminder_name: String,
    reminder_month: u8,
}
