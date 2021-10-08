use crate::enums;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use variant_count::VariantCount;

#[derive(
    Debug, Serialize, Deserialize, IntoPrimitive, TryFromPrimitive, VariantCount, Copy, Clone,
)]
#[repr(u8)]
pub enum GalaxySize {
    Tiny,
    Small,
    Normal,
    Big,
    Huge,
}

impl From<GalaxySize> for usize {
    fn from(s: GalaxySize) -> usize {
        match s {
            GalaxySize::Tiny => 64,
            GalaxySize::Small => 128,
            GalaxySize::Normal => 256,
            GalaxySize::Big => 512,
            GalaxySize::Huge => 1024,
        }
    }
}

impl From<GalaxySize> for &str {
    fn from(s: GalaxySize) -> Self {
        match s {
            GalaxySize::Tiny => "Tiny",
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

    pub fn next(&self) -> Self {
        enums::next(*self, Self::VARIANT_COUNT)
    }

    pub fn prev(&self) -> Self {
        enums::prev(*self, Self::VARIANT_COUNT)
    }
}
