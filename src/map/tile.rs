use crate::map::item::Item;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Tile {
    pub items: Vec<Item>,
}

#[allow(dead_code)]
impl Tile {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Calls when avatar leaves tile
    pub fn off_step(&mut self) {}

    /// Calls when avatar walks on tile
    pub fn on_step(&mut self) {}

    pub fn top_item(&self) -> Option<&Item> {
        self.items.first()
    }
}
