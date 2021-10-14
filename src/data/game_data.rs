use crate::data::entity::DataEntity;
use crate::data::item::Item;
use crate::data::part::Part;
use crate::data::ship::Ship;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn string_to_str(s: &str) -> &'static str {
    Box::leak(s.to_owned().into_boxed_str())
}

#[derive(Debug)]
pub struct GameData {
    items: HashMap<&'static str, Item>,
    parts: HashMap<&'static str, Part>,
    ships: HashMap<&'static str, Ship>,
}

impl GameData {
    pub fn load() -> GameData {
        let path: PathBuf = ["data", "core"].iter().collect();
        let mut data = GameData {
            // TODO: adjust amount of entities
            items: HashMap::with_capacity(10),
            parts: HashMap::with_capacity(10),
            ships: HashMap::with_capacity(1),
        };
        for entry in path.read_dir().unwrap() {
            if let Ok(file) = File::open(entry.unwrap().path()) {
                if let Ok(entities) =
                    serde_json::from_reader::<_, Vec<DataEntity>>(BufReader::new(file))
                {
                    for entity in entities {
                        data.add_entity(entity);
                    }
                }
            }
        }
        data
    }

    fn add_entity(&mut self, entity: DataEntity) {
        match entity {
            DataEntity::Part(part) => {
                self.parts.insert(string_to_str(part.id.as_str()), part);
            }
            DataEntity::Item(item) => {
                self.items.insert(string_to_str(item.id.as_str()), item);
            }
            DataEntity::Ship(ship) => {
                self.ships.insert(string_to_str(ship.id.as_str()), ship);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::game_data::GameData;
    use crate::data::item::ItemTag;

    #[test]
    fn test_load() {
        let data = GameData::load();
        assert!(data.items.contains_key("heart"));
        let heart = data.items.get("heart").unwrap();
        assert_eq!(heart.id, "heart");
        assert!(heart.tags.contains(&ItemTag::BodyPart));
        assert!(data.parts.contains_key("frame"));
        let frame = data.parts.get("frame").unwrap();
        assert_eq!(frame.id, "frame");
        assert_eq!(frame.name, "Main frame");
        let dugong = data.ships.get("dugong").unwrap();
        assert_eq!(dugong.name, "Dugong");
        assert_eq!(dugong.tiles.len(), dugong.bounds.0 * dugong.bounds.1);
        assert_eq!(dugong.tiles.as_slice()[30], "@");
    }
}
