use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Custom(String),
}

impl Gender {
    pub fn pronounce(&self) -> (&str, &str, &str) {
        match self {
            Gender::Male => ("He", "him", "his"),
            Gender::Female => ("She", "her", "her"),
            Gender::Custom(_) => ("They", "them", "their"),
        }
    }
}

impl From<String> for Gender {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            _ => Gender::Custom(value),
        }
    }
}

impl From<Gender> for String {
    fn from(gender: Gender) -> Self {
        match gender {
            Gender::Male => "Male".to_string(),
            Gender::Female => "Female".to_string(),
            Gender::Custom(s) => s,
        }
    }
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        if rng.gen_bool(0.51) {
            Gender::Female
        } else {
            Gender::Male
        }
    }
}
