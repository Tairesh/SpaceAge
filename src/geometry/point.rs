use crate::geometry::direction::Direction;
use crate::Vec2;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul, Range, Sub, SubAssign};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Create a zero point
    pub fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn random<R: Rng + ?Sized>(
        rng: &mut R,
        horizontal: Range<i32>,
        vertical: Range<i32>,
    ) -> Self {
        Self::new(rng.gen_range(horizontal), rng.gen_range(vertical))
    }

    /// Helper for map index conversion
    pub fn to_index(self, width: usize) -> usize {
        (self.y as usize * width) + self.x as usize
    }

    /// Helper for map index conversion
    pub fn from_index(index: usize, width: usize) -> Point {
        Point::new((index % width) as i32, (index / width) as i32)
    }

    /// Direction to other point
    pub fn dir_to(&self, other: &Point) -> Direction {
        Direction::from(*other - *self)
    }

    /// Square distance to other point
    pub fn square_distance(&self, other: &Self) -> i32 {
        let p = *self - *other;
        p.x * p.x + p.y * p.y
    }

    /// Distance (pythagorean) to other point
    pub fn distance(&self, other: &Self) -> f32 {
        f32::sqrt(self.square_distance(other) as f32)
    }
}

impl From<Point> for (i32, i32) {
    fn from(pos: Point) -> Self {
        (pos.x, pos.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl From<Point> for Vec2 {
    fn from(point: Point) -> Self {
        Self::new(point.x as f32, point.y as f32)
    }
}

impl From<Vec2> for Point {
    fn from(vec: Vec2) -> Self {
        Self::new(vec.x.round() as i32, vec.y.round() as i32)
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        Self::new(self.x + rhs.dx(), self.y + rhs.dy())
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, (dx, dy): (i32, i32)) -> Self::Output {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<Vec2> for Point {
    type Output = Point;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x + rhs.x.round() as i32, self.y + rhs.y.round() as i32)
    }
}

impl Sub<Direction> for Point {
    type Output = Point;

    fn sub(self, rhs: Direction) -> Self::Output {
        Self::new(self.x - rhs.dx(), self.y - rhs.dy())
    }
}

impl Sub<(i32, i32)> for Point {
    type Output = Point;

    fn sub(self, (dx, dy): (i32, i32)) -> Self::Output {
        Self::new(self.x - dx, self.y - dy)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Vec2> for Point {
    type Output = Point;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x - rhs.x.round() as i32, self.y - rhs.y.round() as i32)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<(i32, i32)> for Point {
    type Output = Point;

    fn mul(self, (mx, my): (i32, i32)) -> Self::Output {
        Self::new(self.x * mx, self.y * my)
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            (self.x as f32 * rhs).round() as i32,
            (self.y as f32 * rhs).round() as i32,
        )
    }
}

impl Mul<(f32, f32)> for Point {
    type Output = Point;

    fn mul(self, (mx, my): (f32, f32)) -> Self::Output {
        Self::new(
            (self.x as f32 * mx).round() as i32,
            (self.y as f32 * my).round() as i32,
        )
    }
}

impl Mul<Vec2> for Point {
    type Output = Point;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::new(
            (self.x as f32 * rhs.x).round() as i32,
            (self.y as f32 * rhs.y).round() as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::geometry::direction::Direction;
    use crate::Vec2;

    #[test]
    fn new_point() {
        let pt = Point::new(1, 2);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
    }

    #[test]
    fn index_converting() {
        let pt = Point::new(1, 2);
        assert_eq!(pt.to_index(10), 21);
        let pt2 = Point::from_index(21, 10);
        assert_eq!(pt2, pt);
    }

    #[test]
    fn point_from_vec2() {
        let pt = Point::from(Vec2::new(1.4, 1.5));
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
    }

    #[test]
    fn point_from_tuple() {
        let pt = Point::from((1, 2));
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
    }

    #[test]
    fn add_point_to_point() {
        let pt = Point::new(1, 2);
        let p2 = pt + Point::new(3, 4);
        assert_eq!(p2.x, 4);
        assert_eq!(p2.y, 6);
    }

    #[test]
    fn add_direction_to_point() {
        let pt = Point::new(1, 2);
        let pt2 = pt + Direction::NorthWest;
        assert_eq!(pt2.x, 0);
        assert_eq!(pt2.y, 1);
    }

    #[test]
    fn add_tuple_to_point() {
        let pt = Point::new(1, 2);
        let pt2 = pt + (3, 4);
        assert_eq!(pt2.x, 4);
        assert_eq!(pt2.y, 6);
    }

    #[test]
    fn add_vec2_to_point() {
        let pt = Point::new(1, 2);
        let pt2 = pt + Vec2::new(3.4, 3.5);
        assert_eq!(pt2.x, 4);
        assert_eq!(pt2.y, 6);
    }

    #[test]
    fn add_assign_point_to_point() {
        let mut pt = Point::new(1, 2);
        pt += Point::new(3, 4);
        assert_eq!(pt.x, 4);
        assert_eq!(pt.y, 6);
    }

    #[test]
    fn sub_point_to_point() {
        let pt = Point::new(1, 2);
        let p2 = pt - Point::new(3, 4);
        assert_eq!(p2.x, -2);
        assert_eq!(p2.y, -2);
    }

    #[test]
    fn sub_direction_to_point() {
        let pt = Point::new(1, 2);
        let pt2 = pt - Direction::NorthWest;
        assert_eq!(pt2.x, 2);
        assert_eq!(pt2.y, 3);
    }

    #[test]
    fn sub_tuple_to_point() {
        let pt = Point::new(1, 2);
        let pt2 = pt - (3, 4);
        assert_eq!(pt2.x, -2);
        assert_eq!(pt2.y, -2);
    }

    #[test]
    fn sub_vec2_to_point() {
        let pt = Point::new(1, 2);
        let pt2 = pt - Vec2::new(3.4, 3.5);
        assert_eq!(pt2.x, -2);
        assert_eq!(pt2.y, -2);
    }

    #[test]
    fn sub_assign_point_to_point() {
        let mut pt = Point::new(1, 2);
        pt -= Point::new(3, 4);
        assert_eq!(pt.x, -2);
        assert_eq!(pt.y, -2);
    }

    #[test]
    fn mul_point_to_int() {
        let pt = Point::new(1, 2);
        let pt2 = pt * 2;
        assert_eq!(pt2.x, 2);
        assert_eq!(pt2.y, 4);
    }

    #[test]
    fn mul_point_to_tuple() {
        let pt = Point::new(1, 2);
        let pt2 = pt * (3, 4);
        assert_eq!(pt2.x, 3);
        assert_eq!(pt2.y, 8);
    }

    #[test]
    fn mul_point_to_point() {
        let pt = Point::new(1, 2);
        let pt2 = pt * Point::new(3, 4);
        assert_eq!(pt2.x, 3);
        assert_eq!(pt2.y, 8);
    }

    #[test]
    fn mul_point_to_float() {
        let pt = Point::new(1, 2);
        let pt2 = pt * 2.0;
        assert_eq!(pt2.x, 2);
        assert_eq!(pt2.y, 4);
    }

    #[test]
    fn mul_point_to_float_tuple() {
        let pt = Point::new(1, 2);
        let pt2 = pt * (3.0, 4.0);
        assert_eq!(pt2.x, 3);
        assert_eq!(pt2.y, 8);
    }

    #[test]
    fn mul_point_to_vec() {
        let pt = Point::new(1, 2);
        let pt2 = pt * Vec2::new(3.0, 4.0);
        assert_eq!(pt2.x, 3);
        assert_eq!(pt2.y, 8);
    }

    #[test]
    fn test_dist() {
        let pt = Point::new(1, 2);
        let pt2 = Point::new(3, 4);
        assert_eq!(pt.square_distance(&pt2), 8);
        assert!(f32::abs(pt.distance(&pt2) - 2.828_427) < f32::EPSILON);
    }
}
