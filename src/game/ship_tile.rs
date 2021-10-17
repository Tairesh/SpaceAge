use crate::game::part::{Part, PartInteract, PartView};
use crate::game::part_action::PartAction;
use crate::game::passage::Passage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShipTile {
    pub parts: Vec<Part>,
}

impl ShipTile {
    pub fn is_void(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn top_part(&self) -> Option<&Part> {
        self.parts.iter().filter(|p| p.visible()).max()
    }

    pub fn passage(&self) -> Passage {
        if self.is_void() {
            return Passage::Unpassable; // TODO: EVA
        }
        self.top_part().unwrap().passage()
    }

    pub fn support_action(&self, action: PartAction) -> bool {
        self.parts.iter().any(|p| p.support_action(action))
    }

    pub fn action_length(&self, action: PartAction) -> Option<u32> {
        self.parts
            .iter()
            .find(|p| p.support_action(action))?
            .action_length(action)
    }

    pub fn act(&mut self, action: PartAction) {
        self.parts
            .iter_mut()
            .filter(|p| p.support_action(action))
            .for_each(|p| {
                p.act(action);
            });
    }
}
