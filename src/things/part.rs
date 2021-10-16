use crate::colors::Colors;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use tetra::graphics::Color;

#[enum_dispatch::enum_dispatch(Part)]
pub trait PartImpl {
    fn ch(&self) -> char;
    /// tile will display char of part with MAXIMUM z_index
    fn z_index(&self) -> i8;
    /// false if it's a roof for example
    fn visible(&self) -> bool {
        true
    }
    /// color of char
    fn color(&self) -> Color {
        Color::WHITE
    }
    /// bg color for doors
    fn bg_color(&self) -> Option<Color> {
        None
    }
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

impl From<&Part> for char {
    fn from(part: &Part) -> Self {
        part.ch()
    }
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

impl PartImpl for Frame {
    fn ch(&self) -> char {
        '┼'
    }

    fn z_index(&self) -> i8 {
        0
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum WingSegment {
    LeftFront,
    RightFront,
    Normal,
}

impl From<WingSegment> for char {
    fn from(var: WingSegment) -> Self {
        match var {
            WingSegment::LeftFront => 'd',
            WingSegment::RightFront => 'b',
            WingSegment::Normal => 'M',
        }
    }
}

impl From<&str> for WingSegment {
    fn from(ch: &str) -> Self {
        match ch {
            "d" => WingSegment::LeftFront,
            "b" => WingSegment::RightFront,
            "M" => WingSegment::Normal,
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

impl PartImpl for Wing {
    fn ch(&self) -> char {
        self.var.into()
    }

    fn z_index(&self) -> i8 {
        1
    }

    fn color(&self) -> Color {
        Colors::GRAY
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

impl PartImpl for Wall {
    fn ch(&self) -> char {
        self.var.into()
    }

    fn z_index(&self) -> i8 {
        2
    }

    fn color(&self) -> Color {
        Colors::LIGHT_STEEL_BLUE
    }

    fn bg_color(&self) -> Option<Color> {
        Some(Colors::SPACE_VIOLET)
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

impl PartImpl for Floor {
    fn ch(&self) -> char {
        '.'
    }

    fn z_index(&self) -> i8 {
        1
    }

    fn bg_color(&self) -> Option<Color> {
        Some(Colors::SPACE_VIOLET)
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

impl PartImpl for Roof {
    fn ch(&self) -> char {
        '.'
    }

    fn z_index(&self) -> i8 {
        100
    }

    fn visible(&self) -> bool {
        false
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

impl PartImpl for Door {
    fn ch(&self) -> char {
        if self.open {
            '.'
        } else {
            '+'
        }
    }

    fn z_index(&self) -> i8 {
        2
    }

    fn color(&self) -> Color {
        Colors::DARK_GRAY
    }

    fn bg_color(&self) -> Option<Color> {
        if self.open {
            None
        } else {
            Some(Colors::LIGHT_STEEL_BLUE)
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

impl PartImpl for Seat {
    fn ch(&self) -> char {
        'h'
    }

    fn z_index(&self) -> i8 {
        9
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

impl PartImpl for Terminal {
    fn ch(&self) -> char {
        '@'
    }

    fn z_index(&self) -> i8 {
        10
    }
}
