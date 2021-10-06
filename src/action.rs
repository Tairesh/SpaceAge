use crate::direction::Direction;
use crate::world::World;
use std::cell::RefMut;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
}

impl ActionType {
    pub fn name(&self, _world: &mut RefMut<World>) -> String {
        match self {
            ActionType::SkippingTime => "skip time".to_string(),
            ActionType::Walking(_dir) => {
                // let pos = world.avatar.pos + dir;
                // format!("walk through {}", world.load_tile(pos).terrain.name())
                "walk".to_string()
            }
        }
    }

    pub fn verb(&self) -> &str {
        match self {
            ActionType::SkippingTime => "waiting",
            ActionType::Walking(_) => "walking",
        }
    }

    pub fn length(&self, _world: &mut RefMut<World>) -> u64 {
        match self {
            ActionType::SkippingTime => 1,
            ActionType::Walking(_dir) => {
                // TODO: check avatar perks for calculating speed
                // let pos = world.avatar.pos + dir;
                // match world.load_tile(pos).terrain.pass() {
                //     Passage::Passable(length) => length.round() as u64,
                //     Passage::Unpassable => 0,
                // }
                10
            }
        }
    }

    pub fn is_possible(&self, _world: &mut RefMut<World>) -> bool {
        match self {
            ActionType::SkippingTime => true,
            ActionType::Walking(_dir) => {
                // let pos = world.avatar.pos + dir;
                // world.load_tile(pos).terrain.is_walkable()
                true
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Action {
    pub action: ActionType,
    pub finish: u64,
}

impl Action {
    pub fn new(finish: u64, action: ActionType) -> Self {
        Self { action, finish }
    }

    pub fn act(&self, world: &mut World) {
        // TODO: add log messages
        match self.action {
            ActionType::SkippingTime => {}
            ActionType::Walking(dir) => {
                world.move_avatar(dir);
            }
        }
        world.avatar.action = None;
    }
}
