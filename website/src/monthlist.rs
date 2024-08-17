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
    month_name: AttrValue,
    reminder_list: Vec<ReminderProps>,
}
