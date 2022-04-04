use crate::game::ship::Ship;
use crate::game::ship_parts::door::Door;
use crate::game::ship_parts::floor::Floor;
use crate::game::ship_parts::frame::Frame;
use crate::game::ship_parts::roof::Roof;
use crate::game::ship_parts::seat::Seat;
use crate::game::ship_parts::terminal::Terminal;
use crate::game::ship_parts::wall::Wall;
use crate::game::ship_parts::wing::Wing;
use crate::game::ship_parts::ShipPart;
use crate::game::ship_tile::ShipTile;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ShipClass {
    pub id: String,
    pub name: String,
    pub tiles: Vec<String>,
    pub bounds: (i32, i32),
}

impl From<&str> for ShipTile {
    fn from(s: &str) -> Self {
        if s == " " {
            return ShipTile { parts: vec![] };
        }
        let mut parts: Vec<ShipPart> = vec![Frame::new().into()];
        match s {
            ch @ ("d" | "b" | "M" | "V" | "P") => {
                parts.push(Wing::new(ch).into());
            }
            ch @ ("╔" | "═" | "╗" | "║" | "╝" | "╚" | "╠" | "╦" | "╣" | "╩" | "╬") =>
            {
                parts.push(Wall::new(ch).into());
            }
            "." => {
                parts.push(Floor::new().into());
                parts.push(Roof::new().into());
            }
            "+" => {
                parts.push(Floor::new().into());
                parts.push(Door::new(false, false).into());
                parts.push(Roof::new().into());
            }
            "=" => {
                parts.push(Floor::new().into());
                parts.push(Door::new(false, true).into());
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
        ShipTile { parts }
    }
}

pub fn generate_ship<S: Into<String>>(name: S, scheme: &ShipClass) -> Ship {
    Ship {
        name: name.into(),
        class_name: scheme.name.clone(),
        tiles: scheme
            .tiles
            .iter()
            .map(|s| ShipTile::from(s.as_str()))
            .collect(),
        bounds: scheme.bounds,
    }
}

#[cfg(test)]
mod tests {
    use super::generate_ship;
    use crate::data::game_data::GameData;
    use crate::game::ship_parts::ShipPart;

    #[test]
    fn make_dugong() {
        let data = GameData::load();
        let ship = generate_ship("Dugong I", data.ships.get("dugong").unwrap());
        assert_eq!(ship.name, "Dugong I");
        assert_eq!(ship.class_name, "Dugong");
        let tiles = ship.tiles.as_slice();
        assert!(tiles[0].is_void());
        assert_eq!(tiles[30].parts.len(), 4);
        let term = tiles[30].parts.get(2).unwrap();
        assert!(matches!(term, ShipPart::Terminal(..)));
    }
}
