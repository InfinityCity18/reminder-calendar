use crate::consts::SERVER_URL;
use crate::form::Form;
use crate::monthlist::MonthList;
use gloo_net::http::Request;
use monthlist::MonthListProps;
use yew::prelude::*;
use yew_router::prelude::*;

mod consts;
mod form;
mod monthlist;
mod reminder;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

#[function_component]
fn App() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
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
                let data: MonthListProps = response
                    .json()
                    .await
                    .expect("deserialization of months failed");
                hook.set(data);
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
