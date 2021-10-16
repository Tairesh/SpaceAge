use crate::ascii::tile::Tile;
use crate::colors::Colors;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// TODO: get rid of this mess
// Part logic (like hp) and Part view should be in different places
// also one file for one part

#[enum_dispatch::enum_dispatch(Part)]
pub trait PartView {
    /// only part with MAXIMUM z_index will be displayed
    fn z_index(&self) -> i8;
    /// false if it's a roof (invisible when inside)
    fn visible(&self) -> bool {
        true
    }
    /// tile representation
    fn tile(&self) -> Tile;
}

#[enum_dispatch::enum_dispatch]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Part {
    Frame,
    Wing,
    Wall,
    Roof,
    Floor,
    Door,
    Seat,
    Terminal,
}

impl Eq for Part {}

impl PartialEq<Self> for Part {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialOrd<Self> for Part {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Part {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z_index().cmp(&other.z_index())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl PartView for Frame {
    fn z_index(&self) -> i8 {
        0
    }

    fn tile(&self) -> Tile {
        Tile::default('┼')
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl PartView for Wing {
    fn z_index(&self) -> i8 {
        1
    }

    fn tile(&self) -> Tile {
        Tile::new(self.var.into(), Colors::LIGHT_GRAY, None)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl PartView for Wall {
    fn z_index(&self) -> i8 {
        2
    }

    fn tile(&self) -> Tile {
        Tile::with_floor(self.var.into(), Colors::LIGHT_GOLDEN_ROD_YELLOW)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl PartView for Floor {
    fn z_index(&self) -> i8 {
        1
    }

    fn tile(&self) -> Tile {
        Tile::with_floor('.', Colors::GRAY)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl PartView for Roof {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Door {
    hp: u32,
    open: bool,
    locked: bool,
}

impl Door {
    pub fn new() -> Self {
        Self {
            hp: 42,
            open: false,
            locked: false,
        }
    }
}

impl Default for Door {
    fn default() -> Self {
        Self::new()
    }
}

impl PartView for Door {
    fn z_index(&self) -> i8 {
        2
    }

    fn tile(&self) -> Tile {
        if self.open {
            Tile::with_floor('.', Colors::DARK_GRAY)
        } else {
            Tile::with_floor('=', Colors::LIGHT_GOLDEN_ROD_YELLOW)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl PartView for Seat {
    fn z_index(&self) -> i8 {
        9
    }

    fn tile(&self) -> Tile {
        Tile::new('▬', Colors::DARK_SLATE_GRAY, Some(Colors::LIGHT_GRAY))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl PartView for Terminal {
    fn z_index(&self) -> i8 {
        10
    }

    fn tile(&self) -> Tile {
        Tile::new('◙', Colors::LIGHT_STEEL_BLUE, Some(Colors::LIGHT_GREEN))
    }
}
