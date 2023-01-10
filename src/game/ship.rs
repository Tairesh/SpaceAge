use crate::data::ship_class::{generate_ship, ShipClass};
use crate::fov::FovMap;
use crate::game::ship_tile::ShipTile;
use geometry::Point;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ship {
    pub name: String,
    pub class_name: String,
    pub tiles: Vec<ShipTile>,
    pub bounds: (i32, i32),
    // pub squawk: Squawk,  // TODO: implement squawk code (as Part)
}

impl Ship {
    pub fn generate<S: Into<String>>(name: S, scheme: &ShipClass) -> Self {
        generate_ship(name, scheme)
    }

    pub fn find_start_point(&self) -> Point {
        Point::new(self.bounds.0 / 2, self.bounds.1 / 2)
    }

    pub fn get_tile(&self, point: Point) -> Option<&ShipTile> {
        self.tiles.get(point.to_index(self.bounds.0)?)
    }

    pub fn get_tile_mut(&mut self, point: Point) -> Option<&mut ShipTile> {
        self.tiles.get_mut(point.to_index(self.bounds.0)?)
    }
}

impl FovMap for Ship {
    fn dimensions(&self) -> Point {
        self.bounds.into()
    }

    fn is_opaque(&self, idx: usize) -> bool {
        !self.tiles[idx].is_transparent()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::game_data::GameData;
    use crate::data::ship_class::generate_ship;
    use crate::fov::field_of_view_set;
    use geometry::Point;

    #[test]
    fn check_fov() {
        let data = GameData::load();
        let ship = generate_ship("Dugong I", data.ships.get("dugong").unwrap());

        let fov = field_of_view_set(
            Point::new(6, 14),
            ship.bounds.0.max(ship.bounds.1) as u32,
            &ship,
        );
        assert!(fov.contains(&Point::new(5, 15)));
        assert!(fov.contains(&Point::new(5, 14)));
        assert!(fov.contains(&Point::new(4, 15)));
    }
}
