use crate::game::part::{Part, PartInteract, PartView};
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
}
