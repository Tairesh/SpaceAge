use crate::data::ship_class::{generate_ship, ShipClass};
use crate::game::ship_tile::ShipTile;
use crate::geometry::point::Point;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ship {
    pub name: String,
    pub class_name: String,
    pub tiles: Vec<ShipTile>,
    pub bounds: (usize, usize),
    // pub squawk: Squawk,  // TODO: implement squawk code (as Part)
}

impl Ship {
    pub fn generate<S: Into<String>>(name: S, scheme: &ShipClass) -> Self {
        generate_ship(name, scheme)
    }

    pub fn find_start_point(&self) -> Point {
        Point::new(self.bounds.0 as i32 / 2, self.bounds.1 as i32 / 2)
    }
}
