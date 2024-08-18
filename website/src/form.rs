use crate::SERVER_URL;
use gloo_net::http::Request;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::hash_set::HashSet;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component]
pub fn Form(props: &FormProps) -> Html {
    let set = use_state(|| HashSet::<i32>::new());
    let reminder_name = use_state(|| String::new());
    let reminder_day = use_state(|| 0);

    let month_checkboxes = props.months.iter().zip(0..=11).map(|(month_name, i)| {
        let set = set.clone();
        let on_check = Callback::from(move |_| {
            if (*set).contains(&i) {
                let mut hsh = (*set).clone();
                hsh.remove(&i);
                set.set(hsh);
            } else {
                let mut hsh = (*set).clone();
                hsh.insert(i);
                set.set(hsh);
            }
        });
        html! {
            <>
            <label class="form-months" for={month_name.clone()}>{month_name.clone()}</label>
            <input onclick={on_check} type="checkbox" name={month_name.clone()} />
            <br/>
            </>
        }
    });

    let reminder_name_clone = reminder_name.clone();
    let on_name_input = Callback::from(move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            reminder_name_clone.set(input.value());
        }
    });

    let reminder_day_clone = reminder_day.clone();
    let on_day_input = Callback::from(move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            reminder_day_clone.set(input.value().parse().unwrap());
        }
    });

    let reminder_name = reminder_name.clone();
    let reminder_day = reminder_day.clone();
    let set = set.clone();

    let send_reminder = Callback::from(move |_| {
        let reminder_name = reminder_name.clone();
        let reminder_day = reminder_day.clone();
        let set = set.clone();
        debug!("SENDING REMINDER TO ADD");
        wasm_bindgen_futures::spawn_local(async move {
            let post_json = ReminderAddData {
                day: *reminder_day,
                name: (*reminder_name).clone(),
                months: (*set).clone(),
            };

            let post_json = serde_json::to_value(post_json).unwrap();
            debug!("POST JSON ADD REMINDER : {:?}", &post_json);

            Request::post(&(SERVER_URL.to_owned() + "/add"))
                .json(&post_json)
                .expect("sending add reminder post failed")
                .send()
                .await
                .unwrap();
        });
    });

    html! {
        <div class="reminderform">
            <label class="form-name-input" for="remindername">{"Name of the reminder:"}<br/></label>
            <input onchange={on_name_input} type="text" name="remindername" />
            <br/><br/>

            <label class="form-day-input" for="reminderday">{"Day of the reminder:"}<br/></label>
            <input onchange={on_day_input} type="number" placeholder="Deadline day" min="1" max="28" name="reminderday" />
            <br/><br/>

            { for month_checkboxes }

            <br/>
            <input type="button" onclick={send_reminder} value={"Send"} />
        </div>
    }
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct FormProps {
    pub months: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReminderAddData {
    pub day: i32,
    pub name: String,
    pub months: HashSet<i32>,
}
