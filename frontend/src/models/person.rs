use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}
