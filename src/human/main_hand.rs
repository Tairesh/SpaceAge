use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum MainHand {
    Left,
    Right,
    Ambidexter,
}

impl MainHand {
    pub fn name(&self) -> &str {
        match self {
            MainHand::Left => "Left",
            MainHand::Right => "Right",
            MainHand::Ambidexter => "Ambidexter",
        }
    }

    // TODO: use enums::next()
    pub fn next(&self) -> Self {
        match self {
            MainHand::Left => MainHand::Right,
            MainHand::Right => MainHand::Ambidexter,
            MainHand::Ambidexter => MainHand::Left,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            MainHand::Left => MainHand::Ambidexter,
            MainHand::Right => MainHand::Left,
            MainHand::Ambidexter => MainHand::Right,
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
