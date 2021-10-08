use crate::enums;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use variant_count::VariantCount;

#[derive(
    Serialize, Deserialize, IntoPrimitive, TryFromPrimitive, VariantCount, Debug, Copy, Clone,
)]
#[repr(u8)]
pub enum MainHand {
    Left,
    Right,
    Ambidexter,
}

impl MainHand {
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

impl From<MainHand> for &str {
    fn from(s: MainHand) -> Self {
        match s {
            MainHand::Left => "Left",
            MainHand::Right => "Right",
            MainHand::Ambidexter => "Ambidexter",
        }
    }
}

impl Distribution<MainHand> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MainHand {
        if rng.gen_bool(0.01) {
            MainHand::Ambidexter
        } else if rng.gen_bool(0.16) {
            MainHand::Left
        } else {
            MainHand::Right
        }
    }
}
