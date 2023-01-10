use enum_iterator::{next_cycle, previous_cycle, Sequence};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Sequence, Copy, Clone)]
pub enum GalaxySize {
    Small,
    Normal,
    Big,
    Huge,
}

impl From<GalaxySize> for usize {
    fn from(s: GalaxySize) -> usize {
        match s {
            GalaxySize::Small => 64,
            GalaxySize::Normal => 128,
            GalaxySize::Big => 256,
            GalaxySize::Huge => 512,
        }
    }
}

impl From<GalaxySize> for &str {
    fn from(s: GalaxySize) -> Self {
        match s {
            GalaxySize::Small => "Small",
            GalaxySize::Normal => "Normal",
            GalaxySize::Big => "Big",
            GalaxySize::Huge => "Huge",
        }
    }
}

impl GalaxySize {
    pub fn name(&self) -> &str {
        (*self).into()
    }

    pub fn next(self) -> Self {
        next_cycle(&self).unwrap()
    }

    pub fn prev(self) -> Self {
        previous_cycle(&self).unwrap()
    }
}
