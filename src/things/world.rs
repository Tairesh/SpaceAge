#![allow(dead_code)]
use crate::astro::galaxy::Galaxy;
use crate::avatar::Avatar;
use crate::data::game_data::GameData;
use crate::geometry::direction::Direction;
use crate::savefile::{save, SaveFile};
use crate::things::ship::Ship;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    pub path: PathBuf,
    pub current_tick: u128,
    pub galaxy: Galaxy,
    pub avatar: Avatar,
    pub ship: Ship,
}

impl World {
    pub fn new(path: PathBuf, galaxy: Galaxy, avatar: Avatar, ship: Ship) -> Self {
        Self {
            path,
            current_tick: 0,
            galaxy,
            avatar,
            ship,
        }
    }

    pub fn create(savefile: &SaveFile, data: &GameData) -> Self {
        let galaxy = savefile.load_galaxy();
        let ship = Ship::generate("Dugong", data.ships.get("dugong").unwrap());
        World::new(
            savefile.path.clone(),
            galaxy,
            Avatar::new(savefile.character.clone().unwrap(), ship.find_start_point()),
            ship,
        )
    }

    pub fn save(&self) {
        save(self)
            .map_err(|e| panic!("Error on saving world to {:?}: {:?}", self.path, e))
            .ok();
    }

    // TODO: probably this should be moved to Ship
    pub fn move_avatar(&mut self, dir: Direction) {
        let pos = self.avatar.pos;
        // self.load_tile_mut(pos).off_step();
        self.avatar.pos = pos + dir;
        self.avatar.vision = dir;
        // self.load_tile_mut(self.avatar.pos).on_step();
    }

    /// Doing actions that should be done
    fn act(&mut self) {
        // if let Some(action) = self.avatar.action {
        //     if action.finish <= self.meta.current_tick {
        //         action.act(self);
        //     }
        // }
    }

    pub fn tick(&mut self) {
        // self.act();
        // const SPEND_LIMIT: u32 = 100;
        // let mut spend = 0;
        // while self.avatar.action.is_some() && spend < SPEND_LIMIT {
        //     self.meta.current_tick += 1;
        //     spend += 1;
        //     self.act();
        // }
    }
}
