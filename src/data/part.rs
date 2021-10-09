use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Part {
    pub id: String,
    pub name: String,
    pub description: String,
}
