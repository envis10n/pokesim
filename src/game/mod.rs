use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub mod monster;
pub mod moves;
pub mod pokedex;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum ElementType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

impl From<&str> for ElementType {
    fn from(v: &str) -> Self {
        match v {
            "Normal" => ElementType::Normal,
            "Fire" => ElementType::Fire,
            "Water" => ElementType::Water,
            "Electric" => ElementType::Electric,
            "Grass" => ElementType::Grass,
            "Ice" => ElementType::Ice,
            "Fighting" => ElementType::Fighting,
            "Poison" => ElementType::Poison,
            "Ground" => ElementType::Ground,
            "Flying" => ElementType::Flying,
            "Psychic" => ElementType::Psychic,
            "Bug" => ElementType::Bug,
            "Rock" => ElementType::Rock,
            "Ghost" => ElementType::Ghost,
            "Dragon" => ElementType::Dragon,
            "Dark" => ElementType::Dark,
            "Steel" => ElementType::Steel,
            "Fairy" => ElementType::Fairy,
            _ => panic!("Unsupported element type."),
        }
    }
}

pub const POKEMON_ELEMENTS: [&'static str; 18] = [
    "Normal", "Fire", "Water", "Electric", "Grass", "Ice", "Fighting", "Poison", "Ground",
    "Flying", "Psychic", "Bug", "Rock", "Ghost", "Dragon", "Dark", "Steel", "Fairy",
];

impl ElementType {
    pub fn generate() -> (ElementType, ElementType) {
        let mut rng = thread_rng();
        let i1: usize = rng.gen_range(0..POKEMON_ELEMENTS.len());
        let i2: usize = rng.gen_range(0..POKEMON_ELEMENTS.len());
        (
            ElementType::from(POKEMON_ELEMENTS[i1]),
            ElementType::from(POKEMON_ELEMENTS[i2]),
        )
    }
}

unsafe impl Send for ElementType {}
unsafe impl Sync for ElementType {}
