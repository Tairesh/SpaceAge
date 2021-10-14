use serde::Deserialize;

// TODO: use enum for doors/walls/engines etc
#[derive(Deserialize, Debug)]
pub struct Part {
    pub id: String,
    pub name: String,
    pub description: String,
}
