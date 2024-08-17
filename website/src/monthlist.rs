use gloo_net::http::Request;
use yew::prelude::*;

use crate::{remainder::Remainder, SERVER_URL};

#[function_component]
pub fn MonthList(props: &MonthListProps) -> Html {
    let reminders: Vec<Remainder> = Request::get(SERVER_URL)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    html! {}
}

#[derive(Clone, PartialEq, Properties)]
pub struct MonthListProps {
    month: AttrValue,
    reminder_list: Vec<Remainder>,
}
