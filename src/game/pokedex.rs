use super::{monster::*, ElementType};
use std::collections::HashMap;
use std::fs;
use super::data::PokedexEntry;

lazy_static! {
    pub static ref POKEDEX: HashMap<usize, PokedexData> = {
        let mut map: HashMap<usize, PokedexData> = HashMap::new();
        let pjson: Vec<PokedexEntry> =
            serde_json::from_slice(&fs::read("data/pokedex.json").unwrap()[..]).unwrap();
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
    pub fn from_json(obj: &PokedexEntry) -> Self {
        let id = obj.id as usize;
        let name = obj.name.english.clone();
        let elements = obj._type.clone();
        let elements: (&str, &str) = {
            if elements.len() == 2 {
                (elements[0].as_str(), elements[1].as_str())
            } else {
                (elements[0].as_str(), elements[0].as_str())
            }
        };
        let base_stats = [
            obj.base.hp,
            obj.base.attack,
            obj.base.defense,
            obj.base.speed,
            obj.base.sp_attack,
            obj.base.sp_defense,
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
