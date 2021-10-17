use super::super::passage::Passage;
use super::{ShipPartInteract, ShipPartView};
use crate::ascii::tile::Tile;
use crate::colors::Colors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum WingSegment {
    Normal,
    LeftFront,
    RightFront,
    LeftBack,
    RightBack,
}

impl From<WingSegment> for char {
    fn from(var: WingSegment) -> Self {
        match var {
            WingSegment::Normal => 'M',
            WingSegment::LeftFront => 'd',
            WingSegment::RightFront => 'b',
            WingSegment::LeftBack => 'V',
            WingSegment::RightBack => 'P',
        }
    }
}

impl From<&str> for WingSegment {
    fn from(ch: &str) -> Self {
        match ch {
            "d" => WingSegment::LeftFront,
            "b" => WingSegment::RightFront,
            "M" => WingSegment::Normal,
            "V" => WingSegment::LeftBack,
            "P" => WingSegment::RightBack,
            _ => unimplemented!("Invalid wing segment: '{}'", ch),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Wing {
    hp: u32,
    var: WingSegment,
}

impl Wing {
    pub fn new(ch: &str) -> Self {
        Self {
            hp: 30,
            var: WingSegment::from(ch),
        }
    }
}

impl Default for Wing {
    fn default() -> Self {
        Self::new("M")
    }
}

impl ShipPartView for Wing {
    fn z_index(&self) -> i8 {
        1
    }

    fn tile(&self) -> Tile {
        Tile::new(self.var.into(), Colors::LIGHT_GRAY, None)
    }
}

impl ShipPartInteract for Wing {
    fn passage(&self) -> Passage {
        Passage::Passable(50)
    }
}
