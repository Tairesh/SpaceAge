use crate::game::passage::Passage;
use crate::game::world::World;
use crate::geometry::direction::Direction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
}

impl ActionType {
    pub fn length(&self, world: &World) -> Option<u64> {
        match self {
            ActionType::SkippingTime => Some(1),
            ActionType::Walking(dir) => {
                let point = world.avatar.pos + *dir;
                if let Some(tile) = world.ship.tiles.get(point.to_index(world.ship.bounds.0)) {
                    match tile.passage() {
                        Passage::Passable(length) => Some(length as u64),
                        Passage::Unpassable => None,
                    }
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub typ: ActionType,
    pub finish: u128,
}

impl Action {
    pub fn new(typ: ActionType, world: &World) -> Option<Action> {
        let finish = world.current_tick + typ.length(world)? as u128;
        Some(Self { typ, finish })
    }

    pub fn act(&self, world: &mut World) {
        match self.typ {
            ActionType::SkippingTime => {}
            ActionType::Walking(dir) => {
                world.move_avatar(dir);
            }
        }
    }
}
