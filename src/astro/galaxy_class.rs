use crate::enums;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use variant_count::VariantCount;

#[derive(
    Debug, Serialize, Deserialize, IntoPrimitive, TryFromPrimitive, VariantCount, Copy, Clone,
)]
#[repr(u8)]
pub enum GalaxyClass {
    Spiral,
    BaredSpiral,
    Elliptical,
    Circular,
    Irregular,
}

impl From<GalaxyClass> for &str {
    fn from(s: GalaxyClass) -> Self {
        match s {
            GalaxyClass::Spiral => "Spiral",
            GalaxyClass::BaredSpiral => "Bared Spiral",
            GalaxyClass::Elliptical => "Elliptical",
            GalaxyClass::Circular => "Circular",
            GalaxyClass::Irregular => "Irregular",
        }
    }
}

impl GalaxyClass {
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

impl Distribution<GalaxyClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GalaxyClass {
        match rng.gen_range(0..GalaxyClass::VARIANT_COUNT) {
            0 => GalaxyClass::Spiral,
            1 => GalaxyClass::BaredSpiral,
            2 => GalaxyClass::Elliptical,
            3 => GalaxyClass::Circular,
            4 => GalaxyClass::Irregular,
            _ => unreachable!(),
        }
    }
}
