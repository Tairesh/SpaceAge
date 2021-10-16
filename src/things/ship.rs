use crate::data::ship::Ship as ShipScheme;
use crate::geometry::point::Point;
use crate::things::part::Part;
use crate::things::ship_generator::generate_ship;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShipTile {
    pub parts: Vec<Part>,
}

impl ShipTile {
    pub fn is_void(&self) -> bool {
        self.parts.is_empty()
    }
}

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
    pub fn generate<S: Into<String>>(name: S, scheme: &ShipScheme) -> Self {
        generate_ship(name, scheme)
    }

    pub fn find_start_point(&self) -> Point {
        Point::new(self.bounds.0 as i32 / 2, self.bounds.1 as i32 / 2)
    }
}
