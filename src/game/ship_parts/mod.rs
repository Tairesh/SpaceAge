pub mod door;
pub mod floor;
pub mod frame;
pub mod roof;
pub mod seat;
pub mod terminal;
pub mod wall;
pub mod wing;

use self::door::Door;
use self::floor::Floor;
use self::frame::Frame;
use self::roof::Roof;
use self::seat::Seat;
use self::terminal::Terminal;
use self::wall::Wall;
use self::wing::Wing;
use crate::ascii::tile::Tile;
use crate::game::passage::Passage;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[non_exhaustive]
pub enum ShipPartAction {
    Open,
    Close,
}

#[enum_dispatch(ShipPart)]
pub trait ShipPartView {
    /// only part with MAXIMUM z_index will be displayed
    fn z_index(&self) -> i8;
    /// false if it's a roof (invisible when inside)
    fn visible(&self) -> bool {
        true
    }
    /// tile representation
    fn tile(&self) -> Tile;
}

#[enum_dispatch(ShipPart)]
pub trait ShipPartInteract {
    fn passage(&self) -> Passage;
    fn supports_action(&self, action: ShipPartAction) -> bool {
        self.action_length(action).is_some()
    }
    fn action_length(&self, _action: ShipPartAction) -> Option<u32> {
        None
    }
    fn act(&mut self, _action: ShipPartAction) {}
}

#[enum_dispatch]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum ShipPart {
    Frame,
    Wing,
    Wall,
    Roof,
    Floor,
    Door,
    Seat,
    Terminal,
}

impl PartialOrd<Self> for ShipPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShipPart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z_index().cmp(&other.z_index())
    }
}
