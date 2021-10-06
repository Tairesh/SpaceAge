use std::cmp::Ordering;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum TwoDimDirection {
    East,
    West,
}

impl Direction {
    pub fn dx(&self) -> i8 {
        match self {
            Direction::NorthWest | Direction::West | Direction::SouthWest => -1,
            Direction::NorthEast | Direction::East | Direction::SouthEast => 1,
            Direction::North | Direction::South | Direction::Here => 0,
        }
    }

    pub fn dy(&self) -> i8 {
        match self {
            Direction::NorthEast | Direction::North | Direction::NorthWest => -1,
            Direction::SouthEast | Direction::South | Direction::SouthWest => 1,
            Direction::East | Direction::West | Direction::Here => 0,
        }
    }

    pub fn as_two_dimensional(&self) -> Option<TwoDimDirection> {
        match self {
            Direction::NorthEast | Direction::East | Direction::SouthEast => {
                Some(TwoDimDirection::East)
            }
            Direction::SouthWest | Direction::West | Direction::NorthWest => {
                Some(TwoDimDirection::West)
            }
            Direction::North | Direction::South | Direction::Here => None,
        }
    }

    pub fn is_here(&self) -> bool {
        matches!(self, Direction::Here)
    }
}

impl From<Direction> for &str {
    fn from(d: Direction) -> &'static str {
        match d {
            Direction::Here => "here",
            Direction::North => "N",
            Direction::NorthEast => "NE",
            Direction::East => "E",
            Direction::SouthEast => "SE",
            Direction::South => "S",
            Direction::SouthWest => "SW",
            Direction::West => "W",
            Direction::NorthWest => "NW",
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "N" => Direction::North,
            "NE" => Direction::NorthEast,
            "NW" => Direction::NorthWest,
            "S" => Direction::South,
            "SE" => Direction::SouthEast,
            "SW" => Direction::SouthWest,
            "E" => Direction::East,
            "W" => Direction::West,
            _ => Direction::Here,
        }
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
