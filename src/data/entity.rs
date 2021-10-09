use crate::data::item::Item;
use crate::data::part::Part;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum DataEntity {
    Part(Part),
    Item(Item),
}

#[cfg(test)]
mod tests {
    use crate::data::entity::DataEntity;
    use crate::data::item::ItemTag;

    #[test]
    fn test_deserialize() {
        let json = r#"
        [
          {
            "type": "part",
            "id": "frame",
            "name": "Main frame",
            "description": "Holds other parts on it."
          },
          {
            "type": "item",
            "id": "heart",
            "name": "Human heart",
            "tags": [ "BODY_PART" ]
          }
        ]
        "#;
        let data: Vec<DataEntity> = serde_json::from_str(json).unwrap();
        let slice = data.as_slice();
        assert!(matches!(slice[0], DataEntity::Part(..)));
        if let DataEntity::Part(part) = &slice[0] {
            assert_eq!(part.id, "frame");
            assert_eq!(part.name, "Main frame");
        } else {
            unreachable!("First DataEntity is not Part!");
        }
        assert!(matches!(slice[1], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[1] {
            assert!(item.id.eq("heart"));
            assert!(item.tags.contains(&ItemTag::BodyPart));
        } else {
            unreachable!("Second DataEntity is not Item!");
        }
    }
}
