use crate::data::entity::DataEntity;
use crate::data::item::Item;
use crate::data::part::Part;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn load_file(path: &Path, items: &mut HashMap<String, Item>, parts: &mut HashMap<String, Part>) {
    if let Ok(file) = File::open(path) {
        if let Ok(entities) = serde_json::from_reader::<_, Vec<DataEntity>>(BufReader::new(file)) {
            for entity in entities {
                match entity {
                    DataEntity::Part(part) => {
                        parts.insert(part.id.clone(), part);
                    }
                    DataEntity::Item(item) => {
                        items.insert(item.id.clone(), item);
                    }
                }
            }
        }
    }
}

fn load_folder(dir: &Path, items: &mut HashMap<String, Item>, parts: &mut HashMap<String, Part>) {
    for entry in dir.read_dir().unwrap() {
        load_file(&entry.unwrap().path(), items, parts);
    }
}

#[derive(Debug)]
pub struct GameData {
    items: HashMap<String, Item>,
    parts: HashMap<String, Part>,
}

impl GameData {
    pub fn load() -> GameData {
        let path: PathBuf = ["data", "core"].iter().collect();
        // TODO: adjust total amount of entities
        let mut items = HashMap::with_capacity(10);
        let mut parts = HashMap::with_capacity(10);
        load_folder(&path, &mut items, &mut parts);
        GameData { items, parts }
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
    }
}
