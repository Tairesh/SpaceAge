#![allow(dead_code)]
use crate::astro::galaxy::Galaxy;
use crate::data::game_data::GameData;
use crate::game::avatar::Avatar;
use crate::game::ship::Ship;
use crate::geometry::direction::Direction;
use crate::savefile::{save, SaveFile};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    pub path: PathBuf,
    pub current_tick: u128, // TODO: create Clock structure
    pub galaxy: Galaxy,
    pub avatar: Avatar,
    pub ship: Ship,
}

impl World {
    pub fn new(
        path: PathBuf,
        galaxy: Galaxy,
        avatar: Avatar,
        ship: Ship,
        current_tick: u128,
    ) -> Self {
        Self {
            path,
            current_tick,
            galaxy,
            avatar,
            ship,
        }
    }

    pub fn create(savefile: &SaveFile, data: &GameData) -> Self {
        if let Ok(galaxy) = savefile.load_galaxy() {
            let ship = Ship::generate("Dugong", data.ships.get("dugong").unwrap());
            World::new(
                savefile.path.clone(),
                galaxy,
                Avatar::new(savefile.character.clone().unwrap(), ship.find_start_point()),
                ship,
                savefile.current_tick,
            )
        } else {
            panic!("Can't load galaxy: {:?}", savefile.path)
        }
    }

    pub fn save(&self) {
        save(self)
            .map_err(|e| panic!("Error on saving world to {:?}: {:?}", self.path, e))
            .ok();
    }

    pub fn time(&self) -> DateTime<Utc> {
        DateTime::from_utc(
            NaiveDateTime::from_timestamp(32_503_680_000 + self.current_tick as i64 / 60, 0),
            Utc,
        )
    }

    pub fn move_avatar(&mut self, dir: Direction) {
        self.avatar.pos = self.avatar.pos + dir;
        self.avatar.vision = dir;
    }

    /// Doing actions that should be done
    fn act(&mut self) {
        if let Some(action) = &self.avatar.action {
            if action.finish <= self.current_tick {
                action.clone().act(self);
                self.avatar.action = None;
            }
        }
    }

    pub fn tick(&mut self) {
        self.act();
        const SPEND_LIMIT: u8 = 100;
        let mut spend = 0;
        while self.avatar.action.is_some() && spend < SPEND_LIMIT {
            self.current_tick += 1;
            spend += 1;
            self.act();
        }
    }
}
