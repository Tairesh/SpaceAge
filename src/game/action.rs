use crate::game::passage::Passage;
use crate::game::ship_parts::ShipPartAction;
use crate::game::world::World;
use crate::geometry::direction::Direction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
    ActivatingPart(Direction, ShipPartAction),
}

impl ActionType {
    pub fn length(&self, world: &World) -> Option<u32> {
        match self {
            ActionType::SkippingTime => Some(1),
            ActionType::Walking(dir) => {
                let tile = world.ship.get_tile(world.avatar.pos + *dir)?;
                if let Passage::Passable(length) = tile.passage() {
                    Some(length as u32)
                } else {
                    None
                }
            }
            ActionType::ActivatingPart(dir, action) => {
                if matches!(dir, Direction::Here)
                    && matches!(action, ShipPartAction::Open | ShipPartAction::Close)
                {
                    None // TODO: hint explaining why it isn't working
                } else {
                    let tile = world.ship.get_tile(world.avatar.pos + *dir)?;
                    tile.action_length(*action)
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
            ActionType::ActivatingPart(dir, action) => {
                let pos = world.avatar.pos + dir;
                if let Some(tile) = world.ship.get_tile_mut(pos) {
                    tile.act(action);
                }
            }
        }
    }
}
