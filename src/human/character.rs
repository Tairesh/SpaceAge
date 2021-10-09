use crate::human::gender::Gender;
use crate::human::main_hand::MainHand;
use crate::human::skin_tone::SkinTone;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub gender: Gender,
    pub age: u8,
    pub main_hand: MainHand,
    pub skin_tone: SkinTone,
}

impl Character {
    pub fn new<S: Into<String>>(
        name: S,
        gender: Gender,
        age: u8,
        main_hand: MainHand,
        skin_tone: SkinTone,
    ) -> Self {
        Self {
            name: name.into(),
            gender,
            age,
            main_hand,
            skin_tone,
        }
    }
}

impl Distribution<Character> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Character {
        let gender = rng.sample(Standard);
        Character::new(
            "Ashley",
            gender,
            rng.gen_range(0..=199),
            rng.sample(Standard),
            rng.sample(Standard),
        )
    }
}
