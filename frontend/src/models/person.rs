use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone)]
pub struct Person {
    pub name: String,
    pub description: String,
}
