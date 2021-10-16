use crate::data::game_data::GameData;
use crate::human::gender::Gender;
use crate::human::main_hand::MainHand;
use crate::human::skin_tone::SkinTone;
use rand::distributions::Standard;
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

    pub fn random<R: Rng + ?Sized>(rng: &mut R, data: &GameData) -> Character {
        let gender = rng.sample(Standard);
        let packs = data.names.as_slice();
        let pack = &packs[rng.gen_range(0..packs.len())];
        let name = pack.random_name(rng, &gender);
        Character::new(
            name,
            gender,
            rng.gen_range(0..=199),
            rng.sample(Standard),
            rng.sample(Standard),
        )
    }
}
