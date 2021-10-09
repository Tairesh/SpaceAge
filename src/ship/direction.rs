use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Direction {
    Here,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn dx(&self) -> i32 {
        match self {
            Direction::NorthWest | Direction::West | Direction::SouthWest => -1,
            Direction::NorthEast | Direction::East | Direction::SouthEast => 1,
            Direction::North | Direction::South | Direction::Here => 0,
        }
    }

    pub fn dy(&self) -> i32 {
        match self {
            Direction::NorthEast | Direction::North | Direction::NorthWest => -1,
            Direction::SouthEast | Direction::South | Direction::SouthWest => 1,
            Direction::East | Direction::West | Direction::Here => 0,
        }
    }

    pub fn is_here(&self) -> bool {
        matches!(self, Direction::Here)
    }
}

impl From<(i32, i32)> for Direction {
    fn from((dx, dy): (i32, i32)) -> Self {
        let zero = 0;
        match dx.cmp(&zero) {
            Ordering::Less => match dy.cmp(&zero) {
                Ordering::Less => Direction::NorthWest,
                Ordering::Equal => Direction::West,
                Ordering::Greater => Direction::SouthWest,
            },
            Ordering::Equal => match dy.cmp(&zero) {
                Ordering::Less => Direction::North,
                Ordering::Equal => Direction::Here,
                Ordering::Greater => Direction::South,
            },
            Ordering::Greater => match dy.cmp(&zero) {
                Ordering::Less => Direction::NorthEast,
                Ordering::Equal => Direction::East,
                Ordering::Greater => Direction::SouthEast,
            },
        }
    }
}

#[derive(Debug)]
pub enum TwoDimDirection {
    East,
    West,
}

#[derive(Debug)]
pub enum DirectionConvertError {
    North,
    South,
    Here,
}

impl TryFrom<Direction> for TwoDimDirection {
    type Error = DirectionConvertError;

    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        match value {
            Direction::NorthEast | Direction::East | Direction::SouthEast => {
                Ok(TwoDimDirection::East)
            }
            Direction::SouthWest | Direction::West | Direction::NorthWest => {
                Ok(TwoDimDirection::West)
            }
            Direction::North => Err(DirectionConvertError::North),
            Direction::South => Err(DirectionConvertError::South),
            Direction::Here => Err(DirectionConvertError::Here),
        }
    }
}
