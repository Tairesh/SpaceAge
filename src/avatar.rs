use crate::human::character::Character;
use crate::ship::direction::Direction;
use crate::ship::pos::Pos;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Avatar {
    pub character: Character,
    pub pos: Pos,
    pub vision: Direction,
}

impl Avatar {
    pub fn new(character: Character, pos: Pos) -> Self {
        Avatar {
            character,
            pos,
            vision: Direction::East,
        }
    }
}
