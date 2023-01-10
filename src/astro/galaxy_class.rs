use enum_iterator::{next_cycle, previous_cycle, Sequence};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Sequence, Copy, Clone, Eq, PartialEq)]
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

    pub fn next(self) -> Self {
        next_cycle(&self).unwrap()
    }

    pub fn prev(self) -> Self {
        previous_cycle(&self).unwrap()
    }
}

impl Distribution<GalaxyClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GalaxyClass {
        match rng.gen_range(0..5) {
            0 => GalaxyClass::Spiral,
            1 => GalaxyClass::BaredSpiral,
            2 => GalaxyClass::Elliptical,
            3 => GalaxyClass::Circular,
            4 => GalaxyClass::Irregular,
            _ => unreachable!(),
        }
    }
}
