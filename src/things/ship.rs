use crate::data::ship::Ship as ShipScheme;

// TODO: use enum for doors/walls/engines etc
pub struct Part {
    pub proto: String,
    pub char: char,
}

impl Part {
    pub fn new(proto: &str, char: char) -> Self {
        Self {
            proto: proto.to_string(),
            char,
        }
    }

    pub fn frame() -> Self {
        Self::new("frame", '┼')
    }

    pub fn wing_segment(char: char) -> Self {
        Self::new("wing_segment", char)
    }

    pub fn wall(char: char) -> Self {
        Self::new("wall", char)
    }

    pub fn door(open: bool) -> Self {
        Self::new("door", if open { '.' } else { '+' })
    }

    pub fn floor() -> Self {
        Self::new("floor", '.')
    }

    pub fn roof() -> Self {
        Self::new("roof", '┼')
    }

    pub fn terminal() -> Self {
        Self::new("terminal", '@')
    }

    pub fn seat() -> Self {
        Self::new("seat", 'h')
    }
}

pub struct Tile {
    pub parts: Vec<Part>,
}

impl Tile {
    #[allow(dead_code)]
    pub fn is_void(&self) -> bool {
        self.parts.is_empty()
    }
}

impl From<&str> for Tile {
    fn from(s: &str) -> Self {
        if s == " " {
            return Tile { parts: vec![] };
        }
        let mut parts = vec![Part::frame()];
        match s {
            ch @ ("d" | "b" | "M") => {
                parts.push(Part::wing_segment(ch.as_bytes()[0] as char));
            }
            ch
            @
            ("╔" | "═" | "╗" | "║" | "╝" | "╚" | "╠" | "╦" | "╣" | "╩" | "╬") =>
            {
                parts.push(Part::wall(ch.as_bytes()[0] as char));
            }
            "." => {
                parts.push(Part::floor());
                parts.push(Part::roof());
            }
            "+" => {
                parts.push(Part::floor());
                parts.push(Part::door(false));
                parts.push(Part::roof());
            }
            "@" => {
                parts.push(Part::floor());
                parts.push(Part::terminal());
                parts.push(Part::roof());
            }
            "h" => {
                parts.push(Part::floor());
                parts.push(Part::seat());
                parts.push(Part::roof());
            }
            _ => unimplemented!("'{}' is not valid tile", s),
        }
        Tile { parts }
    }
}

#[allow(dead_code)]
pub struct Ship {
    pub name: String,
    pub class_name: String,
    pub tiles: Vec<Tile>,
    pub bounds: (usize, usize),
    // pub squawk: Squawk,  // TODO: implement squawk code
}

impl Ship {
    #[allow(dead_code)]
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
}

#[cfg(test)]
mod tests {
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
        assert_eq!(tiles[30].parts.len(), 4);
        assert_eq!(tiles[30].parts.get(2).unwrap().proto, "terminal");
    }
}
