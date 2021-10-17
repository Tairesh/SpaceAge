use super::super::passage::Passage;
use super::{ShipPartInteract, ShipPartView};
use crate::ascii::tile::Tile;
use crate::colors::Colors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Floor {
    hp: u32,
}

impl Floor {
    pub fn new() -> Self {
        Self { hp: 100 }
    }
}

impl Default for Floor {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Floor {
    fn z_index(&self) -> i8 {
        1
    }

    fn tile(&self) -> Tile {
        Tile::with_floor('.', Colors::GRAY)
    }
}

impl ShipPartInteract for Floor {
    fn passage(&self) -> Passage {
        Passage::Passable(10)
    }
}
