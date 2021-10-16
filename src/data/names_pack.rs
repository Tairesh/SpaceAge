use crate::human::gender::Gender;
use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NamesPack {
    pub id: String,
    pub first_names_male: Vec<String>,
    pub first_names_female: Vec<String>,
    pub last_names_male: Vec<String>,
    #[serde(default)]
    pub last_names_female: Vec<String>,
}

impl NamesPack {
    #[allow(dead_code)]
    pub fn random_name<R: Rng + ?Sized>(&self, rng: &mut R, gender: Gender) -> String {
        let first_names = match gender {
            Gender::Male => &self.first_names_male,
            Gender::Female => &self.first_names_female,
            Gender::Custom(_) => {
                if rng.gen_bool(0.5) {
                    &self.first_names_male
                } else {
                    &self.first_names_female
                }
            }
        }
        .as_slice();
        let first_name = first_names[rng.gen_range(0..first_names.len())].as_str();
        let last_names = if self.last_names_female.is_empty() {
            &self.last_names_male
        } else {
            match gender {
                Gender::Male => &self.last_names_male,
                Gender::Female => &self.last_names_female,
                Gender::Custom(_) => {
                    if rng.gen_bool(0.5) {
                        &self.first_names_male
                    } else {
                        &self.first_names_female
                    }
                }
            }
        }
        .as_slice();
        let last_name = last_names[rng.gen_range(0..last_names.len())].as_str();
        format!("{} {}", first_name, last_name)
    }
}

#[cfg(test)]
mod tests {
    use super::NamesPack;
    use crate::human::gender::Gender;
    use rand::thread_rng;

    #[test]
    fn random_name() {
        let pack = NamesPack {
            id: "test".to_string(),
            first_names_male: vec!["Ilya".to_string()],
            first_names_female: vec!["Ashley".to_string()],
            last_names_male: vec!["Afganov".to_string()],
            last_names_female: vec!["Afganova".to_string()],
        };
        let mut rng = thread_rng();
        let name = pack.random_name(&mut rng, Gender::Male);
        assert_eq!(name, "Ilya Afganov");

        let name = pack.random_name(&mut rng, Gender::Female);
        assert_eq!(name, "Ashley Afganova");

        let pack = NamesPack {
            id: "test".to_string(),
            first_names_male: vec!["Ilya".to_string()],
            first_names_female: vec!["Ashley".to_string()],
            last_names_male: vec!["Afganov".to_string()],
            last_names_female: vec![],
        };
        let name = pack.random_name(&mut rng, Gender::Female);
        assert_eq!(name, "Ashley Afganov");
    }
}
