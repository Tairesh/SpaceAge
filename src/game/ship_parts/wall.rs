use super::super::passage::Passage;
use super::{ShipPartInteract, ShipPartView};
use crate::ascii::tile::Tile;
use crate::colors::Colors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum WallSegment {
    Vertical,
    Horizontal,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
    VerticalLeft,
    VerticalRight,
    HorizontalTop,
    HorizontalBottom,
    Cross,
}

impl From<WallSegment> for char {
    fn from(var: WallSegment) -> Self {
        match var {
            WallSegment::Vertical => '║',
            WallSegment::Horizontal => '═',
            WallSegment::LeftTop => '╔',
            WallSegment::LeftBottom => '╚',
            WallSegment::RightTop => '╗',
            WallSegment::RightBottom => '╝',
            WallSegment::VerticalLeft => '╠',
            WallSegment::VerticalRight => '╣',
            WallSegment::HorizontalTop => '╦',
            WallSegment::HorizontalBottom => '╩',
            WallSegment::Cross => '╬',
        }
    }
}

impl From<&str> for WallSegment {
    fn from(ch: &str) -> Self {
        match ch {
            "═" => WallSegment::Horizontal,
            "║" => WallSegment::Vertical,
            "╔" => WallSegment::LeftTop,
            "╚" => WallSegment::LeftBottom,
            "╗" => WallSegment::RightTop,
            "╝" => WallSegment::RightBottom,
            "╠" => WallSegment::VerticalLeft,
            "╣" => WallSegment::VerticalRight,
            "╦" => WallSegment::HorizontalTop,
            "╩" => WallSegment::HorizontalBottom,
            "╬" => WallSegment::Cross,
            _ => unimplemented!("Invalid wall segment: '{}'", ch),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Wall {
    hp: u32,
    var: WallSegment,
}

impl Wall {
    pub fn new(ch: &str) -> Self {
        Self {
            hp: 100,
            var: WallSegment::from(ch),
        }
    }
}

impl Default for Wall {
    fn default() -> Self {
        Self::new("╬")
    }
}

impl ShipPartView for Wall {
    fn z_index(&self) -> i8 {
        2
    }

    fn tile(&self) -> Tile {
        Tile::with_floor(self.var.into(), Colors::LIGHT_GOLDEN_ROD_YELLOW)
    }
}

impl ShipPartInteract for Wall {
    fn passage(&self) -> Passage {
        Passage::Unpassable
    }
}
