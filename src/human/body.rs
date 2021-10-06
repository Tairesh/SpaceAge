use crate::human::character::Character;
use crate::human::gender::Gender;
use crate::human::skin_tone::SkinTone;
use crate::map::item::{Item, ItemType};
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BodyPart {
    pub age: u8,
    pub gender: Gender,
    pub skin_tone: SkinTone,
    pub outside: HashMap<String, Item>,
    pub inside: HashMap<String, Item>,
}

impl BodyPart {
    pub fn new(character: &Character) -> Self {
        Self {
            age: character.age,
            gender: character.gender.clone(),
            skin_tone: character.skin_tone,
            outside: HashMap::new(),
            inside: HashMap::new(),
        }
    }

    pub fn age_name(&self, with_gender: bool) -> &str {
        match self.age {
            0..=3 => "baby",
            4..=15 => {
                if with_gender {
                    match self.gender {
                        Gender::Male => "boy",
                        Gender::Female => "girl",
                        Gender::Custom(_) => "child",
                    }
                } else {
                    "child"
                }
            }
            16.. => {
                if with_gender {
                    match self.gender {
                        Gender::Male => "male",
                        Gender::Female => "female",
                        Gender::Custom(_) => "human",
                    }
                } else {
                    "human"
                }
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Body {
    pub parts: HashMap<String, Item>,
}

impl Body {
    pub fn human(character: &Character) -> Self {
        let body_part = BodyPart::new(character);
        let mut parts = HashMap::new();
        let mut head = body_part.clone();
        head.outside.insert(
            "left eye".to_string(),
            Item::new(ItemType::HumanEye(body_part.clone())),
        );
        head.outside.insert(
            "right eye".to_string(),
            Item::new(ItemType::HumanEye(body_part.clone())),
        );
        head.outside.insert(
            "nose".to_string(),
            Item::new(ItemType::HumanNose(body_part.clone())),
        );
        head.outside.insert(
            "mouth".to_string(),
            Item::new(ItemType::HumanMouth(body_part.clone())),
        );
        head.inside.insert(
            "brain".to_string(),
            Item::new(ItemType::HumanBrain(body_part.clone())),
        );
        parts.insert("head".to_string(), Item::new(ItemType::HumanHead(head)));
        let mut torso = body_part.clone();
        torso.inside.insert(
            "heart".to_string(),
            Item::new(ItemType::HumanHeart(body_part.clone())),
        );
        torso.inside.insert(
            "stomach".to_string(),
            Item::new(ItemType::HumanStomach(body_part.clone())),
        );
        torso.inside.insert(
            "left lung".to_string(),
            Item::new(ItemType::HumanLung(body_part.clone())),
        );
        torso.inside.insert(
            "right lung".to_string(),
            Item::new(ItemType::HumanLung(body_part.clone())),
        );
        torso.inside.insert(
            "left kidney".to_string(),
            Item::new(ItemType::HumanKidney(body_part.clone())),
        );
        torso.inside.insert(
            "right kidney".to_string(),
            Item::new(ItemType::HumanKidney(body_part.clone())),
        );
        parts.insert("torso".to_string(), Item::new(ItemType::HumanTorso(torso)));
        let mut left_arm = body_part.clone();
        left_arm.outside.insert(
            "hand".to_string(),
            Item::new(ItemType::HumanLeftHand(body_part.clone())),
        );
        parts.insert(
            "left arm".to_string(),
            Item::new(ItemType::HumanLeftArm(left_arm)),
        );
        let mut right_arm = body_part.clone();
        right_arm.outside.insert(
            "hand".to_string(),
            Item::new(ItemType::HumanRightHand(body_part.clone())),
        );
        parts.insert(
            "right arm".to_string(),
            Item::new(ItemType::HumanRightArm(right_arm)),
        );
        let mut left_foot = body_part.clone();
        left_foot.outside.insert(
            "foot".to_string(),
            Item::new(ItemType::HumanLeftFoot(body_part.clone())),
        );
        parts.insert(
            "left leg".to_string(),
            Item::new(ItemType::HumanLeftLeg(left_foot)),
        );
        let mut right_foot = body_part.clone();
        right_foot.outside.insert(
            "foot".to_string(),
            Item::new(ItemType::HumanRightFoot(body_part)),
        );
        parts.insert(
            "right leg".to_string(),
            Item::new(ItemType::HumanRightLeg(right_foot)),
        );

        Self { parts }
    }
}

#[cfg(test)]
mod tests {
    use crate::human::body::Body;
    use crate::human::character::Character;
    use crate::human::gender::Gender;
    use crate::human::main_hand::MainHand;
    use crate::human::skin_tone::SkinTone;
    use crate::map::item::ItemType;

    #[test]
    fn test_human_creating() {
        let character = Character::new(
            "Ashley",
            Gender::Female,
            16,
            MainHand::Right,
            SkinTone::Amber,
        );
        let body = Body::human(&character);
        let head = body.parts.get("head").unwrap();
        assert!(matches!(head.item_type, ItemType::HumanHead(..)));
        let head = head.item_type.body_part().unwrap();
        assert!(matches!(head.skin_tone, SkinTone::Amber));
        let brain = head.inside.get("brain").unwrap();
        assert!(matches!(brain.item_type, ItemType::HumanBrain(..)));
        let brain = brain.item_type.body_part().unwrap();
        assert!(matches!(brain.gender, Gender::Female));
        assert_eq!(
            head.outside
                .iter()
                .filter(|(_, item)| matches!(item.item_type, ItemType::HumanEye(..)))
                .count(),
            2
        );
    }
}
