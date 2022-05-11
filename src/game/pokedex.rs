use super::{monster::*, ElementType};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

const POKEDEX_LANG: &'static str = "english";

lazy_static! {
    pub static ref POKEDEX: HashMap<usize, PokedexData> = {
        let mut map: HashMap<usize, PokedexData> = HashMap::new();
        let pjson: Value =
            serde_json::from_slice(&fs::read("data/pokedex.json").unwrap()[..]).unwrap();
        let pjson = pjson.as_array().unwrap();
        for obj in pjson.iter() {
            let entry = PokedexData::from_json(obj);
            map.insert(entry.id, entry);
        }
        map
    };
}

#[derive(Debug, Clone)]
pub struct PokedexData {
    pub id: usize,
    pub name: String,
    pub element: (ElementType, ElementType),
    pub base: MonsterStats,
}

impl PokedexData {
    pub fn get_pokemon(id: usize) -> Option<Self> {
        if let Some(val) = (*POKEDEX).get(&id) {
            Some(val.clone())
        } else {
            None
        }
    }
    pub fn from_json(obj: &Value) -> Self {
        let obj = obj.as_object().unwrap();
        let id = obj["id"].as_u64().unwrap() as usize;
        let name = obj["name"].as_object().unwrap()[POKEDEX_LANG]
            .as_str()
            .unwrap()
            .to_string();
        let elements = obj["type"].as_array().unwrap();
        let elements: (&str, &str) = {
            if elements.len() == 2 {
                (elements[0].as_str().unwrap(), elements[1].as_str().unwrap())
            } else {
                (elements[0].as_str().unwrap(), elements[0].as_str().unwrap())
            }
        };
        let base_stats = obj["base"].as_object().unwrap();
        let base_stats = [
            base_stats["HP"].as_u64().unwrap() as u8,
            base_stats["Attack"].as_u64().unwrap() as u8,
            base_stats["Defense"].as_u64().unwrap() as u8,
            base_stats["Speed"].as_u64().unwrap() as u8,
            base_stats["Sp. Attack"].as_u64().unwrap() as u8,
            base_stats["Sp. Defense"].as_u64().unwrap() as u8,
        ];
        Self {
            id,
            name,
            element: (ElementType::from(elements.0), ElementType::from(elements.1)),
            base: MonsterStats {
                internal: base_stats,
            },
        }
    }
}
