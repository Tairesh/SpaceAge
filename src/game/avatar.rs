use crate::game::action::Action;
use crate::human::character::Character;
use geometry::{Direction, Point};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Avatar {
    pub character: Character,
    pub pos: Point, // tile in ship
    pub vision: Direction,
    pub action: Option<Action>,
}

impl Avatar {
    pub fn new(character: Character, pos: Point) -> Self {
        Avatar {
            character,
            pos,
            vision: Direction::East,
            action: None,
        }
    }
}
