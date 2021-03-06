use crate::geometry::point::Point;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, IntoPrimitive, TryFromPrimitive, Debug, Copy, Clone)]
#[repr(u8)]
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
    pub fn from_delta(dx: i32, dy: i32) -> Self {
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
        Self::from_delta(dx, dy)
    }
}

impl From<Point> for Direction {
    fn from(point: Point) -> Self {
        Self::from_delta(point.x, point.y)
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

pub const DIR8: [Direction; 8] = [
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
    Direction::North,
    Direction::NorthEast,
];

#[allow(dead_code)]
pub const DIR9: [Direction; 9] = [
    Direction::Here,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
    Direction::North,
    Direction::NorthEast,
];

#[cfg(test)]
mod tests {
    use super::super::point::Point;
    use super::{Direction, DirectionConvertError, TwoDimDirection};
    use std::convert::{TryFrom, TryInto};

    #[test]
    fn from_delta() {
        let dir = Direction::from_delta(10, 20);
        assert!(matches!(dir, Direction::SouthEast));
    }

    #[test]
    fn from_tuple() {
        let dir = Direction::from((10, 20));
        assert!(matches!(dir, Direction::SouthEast));
    }

    #[test]
    fn from_point() {
        let dir = Direction::from(Point::new(10, 20));
        assert!(matches!(dir, Direction::SouthEast));
    }

    #[test]
    fn from_point_diff() {
        let pt = Point::new(1, 2);
        let dir = pt.dir_to(&Point::new(3, 4));
        assert!(matches!(dir, Direction::SouthEast));
    }

    #[test]
    fn to_two_dim() {
        let dir: TwoDimDirection = Direction::SouthEast.try_into().unwrap();
        assert!(matches!(dir, TwoDimDirection::East));
        let dir: TwoDimDirection = Direction::West.try_into().unwrap();
        assert!(matches!(dir, TwoDimDirection::West));
        let dir = TwoDimDirection::try_from(Direction::North);
        assert!(matches!(dir, Err(DirectionConvertError::North)));
    }
}
