use super::super::passage::Passage;
use super::{ShipPartInteract, ShipPartView};
use crate::ascii::tile::Tile;
use crate::colors::Colors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Roof {
    hp: u32,
}

impl Roof {
    pub fn new() -> Self {
        Self { hp: 100 }
    }
}

impl Default for Roof {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Roof {
    fn z_index(&self) -> i8 {
        100
    }

    fn visible(&self) -> bool {
        false
    }

    fn tile(&self) -> Tile {
        Tile::with_floor('+', Colors::LIGHT_GOLDEN_ROD_YELLOW)
    }
}

impl ShipPartInteract for Roof {
    fn passage(&self) -> Passage {
        Passage::Passable(50)
    }
}
