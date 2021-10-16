use crate::data::item::Item;
use crate::data::names_pack::NamesPack;
use crate::data::ship_class::ShipClass;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum DataEntity {
    Item(Item),
    ShipClass(ShipClass),
    NamesPack(NamesPack),
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
            "type": "item",
            "id": "heart",
            "name": "Human heart",
            "tags": [ "BODY_PART" ]
          },
          {
            "type": "ship_class",
            "id": "dugong",
            "name": "Dugong",
            "tiles": [
                " ", "^", " ",
                "|", "h", "|",
                "|", ".", "|",
                "E", "+", "E"
            ],
            "bounds": [3, 4]
          },
          {
            "type": "names_pack",
            "id": "test",
            "first_names_male": [ "Ilya", "Victor" ],
            "first_names_female": [ "Ashley" ],
            "last_names_male": [ "Agafonov", "Nikolayev" ]
          }
        ]
        "#;
        let data: Vec<DataEntity> = serde_json::from_str(json).unwrap();
        let slice = data.as_slice();
        assert!(matches!(slice[0], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[0] {
            assert_eq!(item.id, "heart");
            assert!(item.tags.contains(&ItemTag::BodyPart));
        } else {
            unreachable!("First DataEntity is not Item!");
        }
        assert!(matches!(slice[1], DataEntity::ShipClass(..)));
        if let DataEntity::ShipClass(ship) = &slice[1] {
            assert_eq!(ship.id, "dugong");
            assert_eq!(ship.bounds, (3, 4));
            assert_eq!(ship.tiles.len(), 3 * 4);
        } else {
            unreachable!("Second DataEntity is not Ship!");
        }
        assert!(matches!(slice[2], DataEntity::NamesPack(..)));
        if let DataEntity::NamesPack(name_pack) = &slice[2] {
            assert_eq!(name_pack.id, "test");
            assert!(name_pack.first_names_male.contains(&"Ilya".to_string()));
            assert!(name_pack.last_names_male.contains(&"Agafonov".to_string()));
            assert!(name_pack.last_names_female.is_empty());
        } else {
            unreachable!("Third DataEntity is not NamePack!");
        }
    }
}
