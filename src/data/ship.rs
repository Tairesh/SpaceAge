use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ship {
    pub id: String,
    pub name: String,
    pub tiles: Vec<String>,
    pub bounds: (usize, usize),
}

// TODO: create actual ship by this scheme
