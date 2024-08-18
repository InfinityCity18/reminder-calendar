use crate::SERVER_URL;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::hash_set::HashSet;
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
            <label for={month_name.clone()}>{month_name.clone()}</label>
            <input onclick={on_check} type="checkbox" name={month_name.clone()} />
            </>
        }
    });

    let reminder_name_clone = reminder_name.clone();
    let on_name_input = Callback::from(move |e: InputEvent| {
        let data: Option<String> = e.data();
        if let Some(user_input) = data {
            reminder_name_clone.set(user_input);
        }
    });

    let reminder_day_clone = reminder_day.clone();
    let on_day_input = Callback::from(move |e: InputEvent| {
        let data: Option<String> = e.data();
        if let Some(user_input) = data {
            reminder_day_clone.set(user_input.parse().unwrap());
        }
    });

    let reminder_name = reminder_name.clone();
    let reminder_day = reminder_day.clone();
    let set = set.clone();

    let send_reminder = Callback::from(move |_| {
        let reminder_name = reminder_name.clone();
        let reminder_day = reminder_day.clone();
        let set = set.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let post_json = ReminderAddData {
                day: *reminder_day,
                name: (*reminder_name).clone(),
                months: (*set).clone(),
            };

            let post_json = serde_json::to_value(post_json).unwrap();

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
            <form>
                <label for="remindername"><br/></label>
                <input oninput={on_name_input} type="text" placeholder="Name of the reminder" name="remindername" required=true/>

                <label for="reminderday"><br/></label>
                <input oninput={on_day_input} type="number" placeholder="Deadline day" min="1" max="28" name="reminderday" required=true/>

                { for month_checkboxes }

                <button onclick={send_reminder}/>
            </form>
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
