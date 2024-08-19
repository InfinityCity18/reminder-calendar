use crate::form::Form;
use crate::monthlist::MonthList;
use gloo_net::http::Request;
use log::{debug, info};
use monthlist::MonthListProps;
use yew::prelude::*;
use yew_router::prelude::*;

mod form;
mod monthlist;
mod reminder;

const SERVER_URL: &str = "http://10.21.37.100:2137"; // CHANGE TO TAKE ARGUMENTS OR ENV

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

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
    html! {
        <>
        <Form months={(*hook).months_names.clone()}/>
        <MonthList ..(*hook).clone()/>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <App/> },
    }
}

#[function_component]
fn Main() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
