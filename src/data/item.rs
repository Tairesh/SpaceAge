use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub tags: Vec<ItemTag>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemTag {
    BodyPart,
}
