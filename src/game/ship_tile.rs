use crate::game::passage::Passage;
use crate::game::ship_parts::{ShipPart, ShipPartAction, ShipPartInteract, ShipPartView};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShipTile {
    pub parts: Vec<ShipPart>,
}

impl ShipTile {
    pub fn is_void(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn top_part(&self) -> Option<&ShipPart> {
        self.parts.iter().filter(|p| p.visible()).max()
    }

    pub fn passage(&self) -> Passage {
        if self.is_void() {
            return Passage::Unpassable; // TODO: EVA
        }
        self.top_part().unwrap().passage()
    }

    pub fn supports_action(&self, action: ShipPartAction) -> bool {
        self.parts.iter().any(|p| p.supports_action(action))
    }

    pub fn supports_any_action(&self) -> bool {
        self.parts.iter().any(|p| !p.supported_actions().is_empty())
    }

    pub fn action_length(&self, action: ShipPartAction) -> Option<u32> {
        self.parts
            .iter()
            .find(|p| p.supports_action(action))?
            .action_length(action)
    }

    pub fn act(&mut self, action: ShipPartAction) {
        self.parts
            .iter_mut()
            .filter(|p| p.supports_action(action))
            .for_each(|p| {
                p.act(action);
            });
    }

    pub fn is_transparent(&self) -> bool {
        self.parts.iter().all(|p| p.is_transparent())
    }
}
