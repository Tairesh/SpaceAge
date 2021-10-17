use super::super::passage::Passage;
use super::{ShipPartInteract, ShipPartView};
use crate::ascii::tile::Tile;
use crate::colors::Colors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Seat {
    hp: u32,
    // some other data
}

impl Seat {
    pub fn new() -> Self {
        Self { hp: 10 }
    }
}

impl Default for Seat {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Seat {
    fn z_index(&self) -> i8 {
        9
    }

    fn tile(&self) -> Tile {
        Tile::new('▬', Colors::GRAY, Some(Colors::DARK_GRAY))
    }
}

impl ShipPartInteract for Seat {
    fn passage(&self) -> Passage {
        Passage::Passable(100)
    }
}
