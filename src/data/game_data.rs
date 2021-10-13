use crate::data::entity::DataEntity;
use crate::data::item::Item;
use crate::data::part::Part;
use crate::data::ship::Ship;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn load_file(path: &Path, data: &mut GameData) {
    if let Ok(file) = File::open(path) {
        if let Ok(entities) = serde_json::from_reader::<_, Vec<DataEntity>>(BufReader::new(file)) {
            for entity in entities {
                match entity {
                    DataEntity::Part(part) => {
                        data.parts.insert(part.id.clone(), part);
                    }
                    DataEntity::Item(item) => {
                        data.items.insert(item.id.clone(), item);
                    }
                    DataEntity::Ship(ship) => {
                        data.ships.insert(ship.id.clone(), ship);
                    }
                }
            }
        }
    }
}

fn load_folder(dir: &Path, data: &mut GameData) {
    for entry in dir.read_dir().unwrap() {
        load_file(&entry.unwrap().path(), data);
    }
}

#[derive(Debug)]
pub struct GameData {
    items: HashMap<String, Item>,
    parts: HashMap<String, Part>,
    ships: HashMap<String, Ship>,
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
        load_folder(&path, &mut data);
        data
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
