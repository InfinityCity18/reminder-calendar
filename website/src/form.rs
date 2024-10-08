use crate::SERVER_URL;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::hash_set::HashSet;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

#[function_component]
pub fn Form(props: &FormProps) -> Html {
    let form_hidden = use_state(|| true);
    let set = use_state(|| HashSet::<i32>::new());
    let reminder_name = use_state(|| String::new());
    let reminder_day = use_state(|| 0);
    let delete_hidden = use_state(|| true);
    let deletion_input = use_state(|| String::new());
    let uncheck_state = use_state(|| false);
    let deadline_remind_state = use_state(|| 0);

    let form_hidden_clone = form_hidden.clone();
    let change_form_vis = Callback::from(move |_| {
        form_hidden_clone.set(!(*form_hidden_clone));
    });

    let delete_hidden_clone = delete_hidden.clone();
    let change_delete_vis = Callback::from(move |_| {
        delete_hidden_clone.set(!(*delete_hidden_clone));
    });

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

    let deletion_input_clone = deletion_input.clone();
    let on_delete_name_input = Callback::from(move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            deletion_input_clone.set(input.value());
        }
    });

    let deadline_remind_state_clone = deadline_remind_state.clone();
    let on_deadline_input = Callback::from(move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            deadline_remind_state_clone.set(input.value().parse().unwrap());
        }
    });

    let reminder_name = reminder_name.clone();
    let reminder_day = reminder_day.clone();
    let set = set.clone();
    let form_hidden_clone = form_hidden.clone();

    let send_reminder = Callback::from(move |_| {
        let reminder_name = reminder_name.clone();
        let reminder_day = reminder_day.clone();
        let set = set.clone();
        let deadline_remind_state_clone = deadline_remind_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let post_json = ReminderAddData {
                day: *reminder_day,
                name: (*reminder_name).clone(),
                months: (*set).clone(),
                deadline_remind: *deadline_remind_state_clone,
            };

            let post_json = serde_json::to_value(post_json).unwrap();

            Request::post(&(SERVER_URL.to_owned() + "/add"))
                .json(&post_json)
                .expect("sending add reminder post failed")
                .send()
                .await
                .unwrap();
            if let Some(w) = web_sys::window() {
                w.location().reload().unwrap();
            }
        });
        form_hidden_clone.set(true);
    });

    let delete_hidden_clone = delete_hidden.clone();
    let send_deletion = Callback::from(move |_| {
        let deletion_input_clone = deletion_input.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let post_json = ReminderDeleteData {
                name: (*deletion_input_clone).clone(),
            };

            let post_json = serde_json::to_value(post_json).unwrap();

            Request::post(&(SERVER_URL.to_owned() + "/delete"))
                .json(&post_json)
                .expect("sending delete reminder post failed")
                .send()
                .await
                .unwrap();
            if let Some(w) = web_sys::window() {
                w.location().reload().unwrap();
            }
        });
        delete_hidden_clone.set(true);
    });

    let uncheck_state_clone = uncheck_state.clone();
    let send_uncheck = Callback::from(move |_| {
        if !(*uncheck_state_clone) {
            uncheck_state_clone.set(true);
            return;
        }

        wasm_bindgen_futures::spawn_local(async move {
            let post_json = UncheckData { uncheck: true };

            let post_json = serde_json::to_value(post_json).unwrap();

            Request::post(&(SERVER_URL.to_owned() + "/uncheck"))
                .json(&post_json)
                .expect("sending delete reminder post failed")
                .send()
                .await
                .unwrap();
            if let Some(w) = web_sys::window() {
                w.location().reload().unwrap();
            }
        });
    });

    html! {
        <>
        <div class="topbar">
        <input class="toggle-form-button" type="button" onclick={change_form_vis} value={"Add reminder"} />
        <input class="delete-reminder-button" type="button" onclick={change_delete_vis} value={"Delete reminder"} />
        <input class="uncheck-reminder-button" type="button" onclick={send_uncheck} value={"Uncheck all, double click"} />
        </div>
        <div class="reminderform" hidden={*form_hidden}>
            <label class="form-name-input" for="remindername">{"Name of the reminder:"}<br/></label>
            <input onchange={on_name_input} type="text" name="remindername" />
            <br/><br/>

            <label class="form-day-input" for="reminderday">{"Day of the reminder:"}<br/></label>
            <input onchange={on_day_input} type="number" min="1" max="28" name="reminderday" />
            <br/><br/>

            <label class="deadline-input" for="deadline_remind">{"Days to send notification before deadline:"}<br/></label>
            <input onchange={on_deadline_input} type="number" min="1" max="28" name="deadline_remind" />
            <br/><br/>

            { for month_checkboxes }

            <br/>
            <input class="send-form-button" type="button" onclick={send_reminder} value={"Send"} />
        </div>
        <div class="deleteform" hidden={*delete_hidden}>
            <label class="delete-name-input" for="deleteremindername">{"Name of the reminder:"}<br/></label>
            <input onchange={on_delete_name_input} type="text" name="deleteremindername" />
            <br/><br/>

            <input class="send-delete-button" type="button" onclick={send_deletion} value={"Send"} />
        </div>
        </>
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
    pub deadline_remind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReminderDeleteData {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UncheckData {
    pub uncheck: bool,
}
