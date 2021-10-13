#![allow(dead_code)]
use crate::astro::galaxy::Galaxy;
use crate::avatar::Avatar;
use crate::geometry::direction::Direction;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    pub path: PathBuf,
    pub current_tick: u64,
    pub galaxy: Galaxy,
    pub avatar: Avatar,
}

impl World {
    pub fn new(path: PathBuf, galaxy: Galaxy, avatar: Avatar) -> Self {
        Self {
            path,
            current_tick: 0,
            galaxy,
            avatar,
        }
    }

    // TODO: save
    // pub fn save(&mut self) {
    //     let path = self.path.clone();
    //     save(&path, self)
    //         .map_err(|e| panic!("Error on saving world to {:?}: {}", self.path, e))
    //         .ok();
    // }

    // TODO: load tiles

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
