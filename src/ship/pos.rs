use crate::ship::direction::Direction;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

impl From<Pos> for (i32, i32) {
    fn from(pos: Pos) -> Self {
        (pos.x, pos.y)
    }
}

impl From<(i32, i32)> for Pos {
    fn from((x, y): (i32, i32)) -> Self {
        Pos::new(x, y)
    }
}

impl Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, dir: Direction) -> Self::Output {
        self.add(&dir)
    }
}

impl Add<&Direction> for Pos {
    type Output = Pos;

    fn add(self, dir: &Direction) -> Self::Output {
        Self::new(self.x + dir.dx(), self.y + dir.dy())
    }
}

impl Add<(i32, i32)> for Pos {
    type Output = Pos;

    fn add(self, (dx, dy): (i32, i32)) -> Self::Output {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl Add<&(i32, i32)> for Pos {
    type Output = Pos;

    fn add(self, (dx, dy): &(i32, i32)) -> Self::Output {
        self.add((*dx, *dy))
    }
}
