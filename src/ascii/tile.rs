use crate::colors::Colors;
use crate::game::ship_parts::ShipPartView;
use crate::game::ship_tile::ShipTile;
use tetra::graphics::Color;

#[derive(Debug, Clone)]
pub struct Tile {
    pub ch: char,
    pub fg: Color,
    pub bg: Option<Color>,
}

impl Tile {
    pub fn new(ch: char, fg: Color, bg: Option<Color>) -> Self {
        Self { ch, fg, bg }
    }

    pub fn default(ch: char) -> Self {
        Self {
            ch,
            fg: Color::WHITE,
            bg: None,
        }
    }

    pub fn with_floor(ch: char, fg: Color) -> Self {
        Self {
            ch,
            fg,
            bg: Some(Colors::DARKEST_GRAY),
        }
    }

    pub fn empty() -> Self {
        Self::default(' ')
    }
}

impl From<&ShipTile> for Tile {
    fn from(tile: &ShipTile) -> Self {
        if let Some(top_part) = tile.top_part() {
            top_part.tile()
        } else {
            Tile::empty()
        }
    }
}
