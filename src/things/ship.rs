use crate::data::ship::Ship as ShipScheme;
use crate::geometry::point::Point;
use crate::things::part::{Door, Floor, Frame, Part, PartImpl, Roof, Seat, Terminal, Wall, Wing};
use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile {
    pub parts: Vec<Part>,
}

impl Tile {
    pub fn is_void(&self) -> bool {
        self.parts.is_empty()
    }

    fn top_part(&self) -> Option<&Part> {
        self.parts.iter().filter(|p| p.visible()).max()
    }

    pub fn ch(&self) -> char {
        self.top_part().map(char::from).unwrap_or(' ')
    }

    pub fn color(&self) -> Color {
        self.top_part().map(|p| p.color()).unwrap_or(Color::WHITE)
    }

    pub fn bg_color(&self) -> Option<Color> {
        self.top_part().map(|p| p.bg_color()).unwrap_or(None)
    }
}

impl From<&str> for Tile {
    fn from(s: &str) -> Self {
        if s == " " {
            return Tile { parts: vec![] };
        }
        let mut parts: Vec<Part> = vec![Frame::new().into()];
        match s {
            ch @ ("d" | "b" | "M") => {
                parts.push(Wing::new(ch).into());
            }
            ch
            @
            ("╔" | "═" | "╗" | "║" | "╝" | "╚" | "╠" | "╦" | "╣" | "╩" | "╬") =>
            {
                parts.push(Wall::new(ch).into());
            }
            "." => {
                parts.push(Floor::new().into());
                parts.push(Roof::new().into());
            }
            "+" => {
                parts.push(Floor::new().into());
                parts.push(Door::new().into());
                parts.push(Roof::new().into());
            }
            "@" => {
                parts.push(Floor::new().into());
                parts.push(Terminal::new().into());
                parts.push(Roof::new().into());
            }
            "h" => {
                parts.push(Floor::new().into());
                parts.push(Seat::new().into());
                parts.push(Roof::new().into());
            }
            _ => println!("'{}' is not a valid tile", s),
        }
        Tile { parts }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ship {
    pub name: String,
    pub class_name: String,
    pub tiles: Vec<Tile>,
    pub bounds: (usize, usize),
    // pub squawk: Squawk,  // TODO: implement squawk code (as Part)
}

impl Ship {
    pub fn generate<S: Into<String>>(name: S, scheme: &ShipScheme) -> Self {
        Self {
            name: name.into(),
            class_name: scheme.name.clone(),
            tiles: scheme
                .tiles
                .iter()
                .map(|s| Tile::from(s.as_str()))
                .collect(),
            bounds: scheme.bounds,
        }
    }

    pub fn find_start_point(&self) -> Point {
        Point::new(self.bounds.0 as i32 / 2, self.bounds.1 as i32 / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::super::part::{Part, PartImpl};
    use super::Ship;
    use crate::data::game_data::GameData;

    #[test]
    fn make_dugong() {
        let data = GameData::load();
        let ship = Ship::generate("Dugong I", data.ships.get("dugong").unwrap());
        assert_eq!(ship.name, "Dugong I");
        assert_eq!(ship.class_name, "Dugong");
        let tiles = ship.tiles.as_slice();
        assert!(tiles[0].is_void());
        assert_eq!(tiles[0].ch(), ' ');
        assert_eq!(tiles[30].ch(), '@');
        assert_eq!(tiles[30].parts.len(), 4);
        let term = tiles[30].parts.get(2).unwrap();
        assert_eq!(term.ch(), '@');
        assert!(matches!(term, Part::Terminal(..)));
    }
}
