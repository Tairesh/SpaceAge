use super::super::passage::Passage;
use super::{ShipPartAction, ShipPartInteract, ShipPartView};
use crate::ascii::tile::Tile;
use crate::colors::Colors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Door {
    hp: u32,
    open: bool,
    locked: bool,
}

impl Door {
    pub fn new() -> Self {
        Self {
            hp: 42,
            open: false,
            locked: false,
        }
    }
}

impl Default for Door {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Door {
    fn z_index(&self) -> i8 {
        2
    }

    fn tile(&self) -> Tile {
        if self.open {
            Tile::with_floor('.', Colors::LIGHT_STEEL_BLUE)
        } else {
            Tile::with_floor('≡', Colors::LIGHT_STEEL_BLUE)
        }
    }

    fn is_transparent(&self) -> bool {
        self.open
    }
}

impl ShipPartInteract for Door {
    fn passage(&self) -> Passage {
        if self.open {
            Passage::Passable(10)
        } else {
            Passage::Unpassable
        }
    }

    fn supported_actions(&self) -> &[ShipPartAction] {
        if self.open {
            &[ShipPartAction::Close]
        } else {
            &[ShipPartAction::Open]
        }
    }

    fn action_length(&self, action: ShipPartAction) -> Option<u32> {
        match action {
            ShipPartAction::Open => {
                if !self.open {
                    Some(10)
                } else {
                    None
                }
            }
            ShipPartAction::Close => {
                if self.open {
                    Some(10)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn act(&mut self, action: ShipPartAction) {
        match action {
            ShipPartAction::Open => {
                self.open = true;
            }
            ShipPartAction::Close => {
                self.open = false;
            }
            _ => {}
        }
    }
}
