use super::super::passage::Passage;
use super::{ShipPartInteract, ShipPartView};
use crate::ascii::tile::Tile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Frame {
    hp: u32,
}

impl Frame {
    pub fn new() -> Self {
        Frame { hp: 100 }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Frame {
    fn z_index(&self) -> i8 {
        0
    }

    fn tile(&self) -> Tile {
        Tile::default('┼')
    }
}

impl ShipPartInteract for Frame {
    fn passage(&self) -> Passage {
        Passage::Passable(100)
    }
}
