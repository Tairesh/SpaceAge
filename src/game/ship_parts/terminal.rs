use super::super::passage::Passage;
use super::{ShipPartInteract, ShipPartView};
use crate::ascii::tile::Tile;
use crate::colors::Colors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Terminal {
    hp: u32,
    // some other data
}

impl Terminal {
    pub fn new() -> Self {
        Self { hp: 10 }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Terminal {
    fn z_index(&self) -> i8 {
        10
    }

    fn tile(&self) -> Tile {
        Tile::new('◙', Colors::LIGHT_STEEL_BLUE, Some(Colors::LIGHT_GREEN))
    }
}

impl ShipPartInteract for Terminal {
    fn passage(&self) -> Passage {
        Passage::Unpassable
    }
}
