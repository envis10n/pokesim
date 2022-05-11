use super::ElementType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    pub static ref POKEMON_MOVES: HashMap<usize, PokemonMove> = {
        let mut map: HashMap<usize, PokemonMove> = HashMap::new();
        let pjson: Value =
            serde_json::from_slice(&fs::read("data/moves.json").unwrap()[..]).unwrap();
        let pjson = pjson.as_array().unwrap();
        for obj in pjson.iter() {
            let entry = PokemonMove::from_json(obj);
            map.insert(entry.id, entry);
        }
        map
    };
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}

impl MoveCategory {
    pub fn get_element_category(element: ElementType) -> Self {
        match element {
            ElementType::Normal
            | ElementType::Fighting
            | ElementType::Flying
            | ElementType::Poison
            | ElementType::Ground
            | ElementType::Rock
            | ElementType::Bug
            | ElementType::Ghost
            | ElementType::Steel => Self::Physical,

            ElementType::Fire
            | ElementType::Water
            | ElementType::Grass
            | ElementType::Electric
            | ElementType::Psychic
            | ElementType::Ice
            | ElementType::Dragon
            | ElementType::Dark => Self::Special,
            _ => Self::Special,
        }
    }
}

unsafe impl Send for MoveCategory {}
unsafe impl Sync for MoveCategory {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonMove {
    pub id: usize,
    pub name: String,
    pub element: ElementType,
    pub category: MoveCategory,
    pub pp: Option<u8>,
    pub accuracy: Option<u8>,
    pub power: Option<u8>,
}

impl PokemonMove {
    pub fn from_json(obj: &Value) -> PokemonMove {
        let obj = obj.as_object().unwrap();

        let accuracy = {
            if obj["accuracy"].is_null() {
                None
            } else {
                Some(obj["accuracy"].as_u64().unwrap() as u8)
            }
        };
        let id = obj["id"].as_u64().unwrap() as usize;
        let name = obj["ename"].as_str().unwrap().to_string();
        let power = {
            if obj["power"].is_null() {
                None
            } else {
                Some(obj["power"].as_u64().unwrap() as u8)
            }
        };
        let pp = {
            if obj["pp"].is_null() {
                None
            } else {
                Some(obj["pp"].as_u64().unwrap() as u8)
            }
        };
        let element_type = obj["type"].as_str().unwrap();
        let element = ElementType::from(element_type);
        let category = MoveCategory::get_element_category(element.clone());

        Self {
            id,
            name,
            element,
            category,
            pp,
            accuracy,
            power,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::monster::Monster;

    #[test]
    fn get_move() {
        let monster = Monster::from_dex(1);
        println!("{:?}\n\n{:?}", monster, monster.get_move(0).unwrap());
    }
}
