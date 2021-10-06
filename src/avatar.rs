use crate::action::Action;
use crate::direction::Direction;
use crate::human::body::Body;
use crate::human::character::Character;
use crate::map::pos::TilePos;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub character: Character,
    pub body: Body,
    pub pos: TilePos,
    pub action: Option<Action>,
    pub vision: Direction,
}

impl Avatar {
    pub fn new(character: Character, pos: TilePos) -> Self {
        Avatar {
            body: Body::human(&character),
            character,
            pos,
            action: None,
            vision: Direction::East,
        }
    }
}
