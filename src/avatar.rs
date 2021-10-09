use crate::direction::Direction;
use crate::human::character::Character;
use crate::map::pos::TilePos;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Avatar {
    pub character: Character,
    pub pos: TilePos,
    pub vision: Direction,
}

impl Avatar {
    pub fn new(character: Character, pos: TilePos) -> Self {
        Avatar {
            character,
            pos,
            vision: Direction::East,
        }
    }
}
