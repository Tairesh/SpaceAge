use crate::geometry::direction::Direction;
use crate::geometry::point::Point;
use crate::human::character::Character;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Avatar {
    pub character: Character,
    pub pos: Point, // tile in ship
    pub vision: Direction,
}

impl Avatar {
    pub fn new(character: Character, pos: Point) -> Self {
        Avatar {
            character,
            pos,
            vision: Direction::East,
        }
    }
}
