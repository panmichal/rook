use super::text_input::TextInput;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct State {
    pub name: String,
    pub description: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<State>,
}

#[function_component(AddForm)]
pub fn add_form(props: &Props) -> Html {
    let state = use_state(|| State::default());

    let cloned_state = state.clone();
    let on_name_change = Callback::from(move |value| {
        cloned_state.set(State {
            name: value,
            ..cloned_state.deref().clone()
        });
    });
    let cloned_state = state.clone();
    let on_desc_change = Callback::from(move |value| {
        cloned_state.set(State {
            description: value,
            ..cloned_state.deref().clone()
        });
    });

    let form_submit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_submit.emit(data);
    });
    html! {
        <>
        <form onsubmit={onsubmit}>
                <label for="name">{"Name"}</label><br/>
                <TextInput name={"name"} handle_onchange={on_name_change}/><br/>
                <label for="desc">{"Description"}</label><br/>
                <TextInput name={"desc"} handle_onchange={on_desc_change}/><br/>
                <input type="submit" value="Add" class="btn-submit"/>
                </form>
        <div>
        <p>{&*state.name}</p>
        <p>{&*state.description}</p>
        </div>
        </>
    }
}
