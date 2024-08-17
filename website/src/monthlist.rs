use serde::Deserialize;
use serde::Serialize;
use yew::prelude::*;

use crate::reminder::Reminder;
use crate::reminder::ReminderProps;

#[function_component]
pub fn Month(props: &MonthProps) -> Html {
    let is_hidden = use_state(|| false);

    let is_hidden_clone = is_hidden.clone();
    let on_click_unfold = Callback::from(move |_| {
        if *is_hidden_clone {
            is_hidden_clone.set(false);
        } else {
            is_hidden_clone.set(true);
        }
    });

    let reminders = props
        .reminder_list
        .iter()
        .map(|remind_props: &ReminderProps| {
            html! { <Reminder ..remind_props.clone() /> }
        });

    html! {
        <>
        <div onclick={on_click_unfold.clone()}>{props.month_name.clone()}</div>
        <div id={props.month_name.clone()} hidden={*is_hidden}>
        { for reminders }
        </div>
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct MonthProps {
    pub month_name: String,
    pub reminder_list: Vec<ReminderProps>,
}

#[function_component]
pub fn MonthList(props: &MonthListProps) -> Html {
    let months = props
        .months_names
        .iter()
        .zip(props.months_reminders.clone().into_iter())
        .map(|(name, list)| {
            html! {
                <Month month_name={name.clone()} reminder_list={list}/>
            }
        });

    html! {
        { for months }
    }
}

#[derive(Properties, PartialEq, Clone, Serialize, Deserialize)]
pub struct MonthListProps {
    pub months_names: [String; 12],
    pub months_reminders: [Vec<ReminderProps>; 12],
}
