mod components;
mod models;

use std::ops::Deref;

use crate::components::add_form::{AddForm, State};
use crate::components::people_list::PeopleList;
use crate::models::person::Person;
use gloo::console::log;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokePeopleList, catch)]
    pub async fn get_people_list() -> Result<JsValue, JsValue>;
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

fn update_people_list(people_list: UseStateHandle<Vec<Person>>) {
    log!("Updating people list");
    spawn_local(async move {
        match get_people_list().await {
            Ok(people) => {
                log!("Got people list");
                log!("{:?}", people.clone());
                let people: Vec<Person> = serde_wasm_bindgen::from_value(people).unwrap();
                people_list.set(people);
            }
            Err(err) => {
                let window = window().unwrap();
                window
                    .alert_with_message(&format!("Error: {:?}", err))
                    .unwrap();
            }
        }
    })
}

#[function_component(App)]
pub fn app() -> Html {
    let people_list: UseStateHandle<Vec<Person>> = use_state_eq(|| vec![]);

    {
        let people_list = people_list.clone();
        use_effect(move || {
            update_people_list(people_list.clone());
            || ()
        });
    }

    let on_form_submit = Callback::from(|data: State| {
        log!("name: ", data.name);
        log!("description: ", data.description);
    });

    html! {
        <div>
            <div class="main-content">
                <AddForm onsubmit={on_form_submit} />
                <PeopleList people={people_list.deref().clone()} />
            </div>
        </div>
    }
}
