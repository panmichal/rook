mod components;

use crate::components::add_form::{AddForm, State};
use gloo::console::log;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let on_form_submit = Callback::from(|data: State| {
        log!("name: ", data.name);
        log!("description: ", data.description);
    });

    html! {
        <div>
            <div class="main-content">
                <AddForm onsubmit={on_form_submit} />
            </div>
        </div>
    }
}
