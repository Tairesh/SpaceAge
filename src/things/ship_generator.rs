use super::part::*;
use super::ship::{Ship, ShipTile};
use crate::data::ship::Ship as ShipScheme;

impl From<&str> for ShipTile {
    fn from(s: &str) -> Self {
        if s == " " {
            return ShipTile { parts: vec![] };
        }
        let mut parts: Vec<Part> = vec![Frame::new().into()];
        match s {
            ch @ ("d" | "b" | "M" | "V" | "P") => {
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
        ShipTile { parts }
    }
}

pub fn generate_ship<S: Into<String>>(name: S, scheme: &ShipScheme) -> Ship {
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
    use super::super::part::{Part, PartView};
    use super::generate_ship;
    use crate::data::game_data::GameData;

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
        assert!(matches!(term, Part::Terminal(..)));
    }
}
