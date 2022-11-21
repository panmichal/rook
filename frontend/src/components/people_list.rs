use crate::models::person::Person;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub people: Vec<Person>,
}

#[function_component(PeopleList)]
pub fn people_list(props: &Props) -> Html {
    html! {
        <div>
            <h1>{"People List"}</h1>
            <ul>
                { for props.people.iter().map(|person| {
                    html! {
                        <li>
                            <p>{&*person.name}</p>
                            <p>{&person.description.clone().unwrap_or("No description".to_owned())}</p>
                        </li>
                    }
                })}
                </ul>
        </div>
    }
}
