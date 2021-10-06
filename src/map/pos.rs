use crate::direction::Direction;
use std::ops::Add;

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl TilePos {
    pub fn new(x: i32, y: i32) -> Self {
        TilePos { x, y }
    }
}

impl Add<Direction> for TilePos {
    type Output = TilePos;

    fn add(self, dir: Direction) -> Self::Output {
        Self::new(self.x + dir.dx() as i32, self.y + dir.dy() as i32)
    }
}

impl Add<&Direction> for TilePos {
    type Output = TilePos;

    fn add(self, dir: &Direction) -> Self::Output {
        Self::new(self.x + dir.dx() as i32, self.y + dir.dy() as i32)
    }
}

impl Add<(i32, i32)> for TilePos {
    type Output = TilePos;

    fn add(self, (dx, dy): (i32, i32)) -> Self::Output {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl Add<&(i32, i32)> for TilePos {
    type Output = TilePos;

    fn add(self, (dx, dy): &(i32, i32)) -> Self::Output {
        Self::new(self.x + dx, self.y + dy)
    }
}
