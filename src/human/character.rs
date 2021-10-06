use crate::human::gender::Gender;
use crate::human::main_hand::MainHand;
use crate::human::skin_tone::SkinTone;
use rand::distributions::Standard;
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let gender = rng.sample(Standard);
        Self::new(
            "Ashley",
            gender,
            rng.gen_range(0..=199),
            rng.sample(Standard),
            rng.sample(Standard),
        )
    }
}
