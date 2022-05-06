use uuid::Uuid;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::{pokedex::PokedexData, ElementType, moves::{PokemonMove, POKEMON_MOVES}};

#[repr(u8)]
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum MonsterAttribute {
    HP,
    ATT,
    DEF,
    SPD,
    SpATT,
    SpDEF,
}

impl From<u8> for MonsterAttribute {
    fn from(v: u8) -> Self {
        match v {
            0 => MonsterAttribute::HP,
            1 => MonsterAttribute::ATT,
            2 => MonsterAttribute::DEF,
            3 => MonsterAttribute::SPD,
            4 => MonsterAttribute:: SpATT,
            5 => MonsterAttribute::SpDEF,
            _ => panic!("Value out of range."),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum MonsterNature {
    Adamant,
    Bashful,
    Bold,
    Brave,
    Calm,
    Careful,
    Docile,
    Gentle,
    Hardy,
    Hasty,
    Impish,
    Jolly,
    Lax,
    Lonely,
    Mild,
    Modest,
    Naive,
    Naughty,
    Quiet,
    Quirky,
    Rash,
    Relaxed,
    Sassy,
    Serious,
    Timid,
}

/*
    "Adamant", "Bashful", "Bold",
    "Brave",   "Calm",    "Careful",
    "Docile",  "Gentle",  "Hardy",
    "Hasty",   "Impish",  "Jolly",
    "Lax",     "Lonely",  "Mild",
    "Modest",  "Naive",   "Naughty",
    "Quiet",   "Quirky",  "Rash",
    "Relaxed", "Sassy",   "Serious",
    "Timid"
*/

impl From<&str> for MonsterNature {
    fn from(v: &str) -> Self {
        match v {
            "Adamant" => MonsterNature::Adamant,
            "Bashful" => MonsterNature::Bashful,
            "Bold" => MonsterNature::Bold,
            "Brave" => MonsterNature::Brave,
            "Calm" => MonsterNature::Calm,
            "Careful" => MonsterNature::Careful,
            "Docile" => MonsterNature::Docile,
            "Gentle" => MonsterNature::Gentle,
            "Hardy" => MonsterNature::Hardy,
            "Hasty" => MonsterNature::Hasty,
            "Impish" => MonsterNature::Impish,
            "Jolly" => MonsterNature::Jolly,
            "Lax" => MonsterNature::Lax,
            "Lonely" => MonsterNature::Lonely,
            "Mild" => MonsterNature::Mild,
            "Modest" => MonsterNature::Modest,
            "Naive" => MonsterNature::Naive,
            "Naughty" => MonsterNature::Naughty,
            "Quiet" => MonsterNature::Quiet,
            "Quirky" => MonsterNature::Quirky,
            "Rash" => MonsterNature::Rash,
            "Relaxed" => MonsterNature::Relaxed,
            "Sassy" => MonsterNature::Sassy,
            "Serious" => MonsterNature::Serious,
            "Timid" => MonsterNature::Timid,
            _ => panic!("Unexpected nature."),
        }
    }
}

const POKEMON_NATURES: [&'static str; 25] = [
    "Adamant",
    "Bashful",
    "Bold",
    "Brave",
    "Calm",
    "Careful",
    "Docile",
    "Gentle",
    "Hardy",
    "Hasty",
    "Impish",
    "Jolly",
    "Lax",
    "Lonely",
    "Mild",
    "Modest",
    "Naive",
    "Naughty",
    "Quiet",
    "Quirky",
    "Rash",
    "Relaxed",
    "Sassy",
    "Serious",
    "Timid"
];

impl MonsterNature {
    pub fn generate() -> Self {
        let mut rng = thread_rng();
        let i: usize = rng.gen_range(0..POKEMON_NATURES.len());
        MonsterNature::from(POKEMON_NATURES[i])
    }
}

lazy_static! {
    static ref NATURE_ARR: Vec<(&'static str, MonsterAttribute, MonsterAttribute)> = vec![
        ("Adamant", MonsterAttribute::ATT, MonsterAttribute::SpATT),
        ("Bashful", MonsterAttribute::SpATT, MonsterAttribute::SpATT),
        ("Bold", MonsterAttribute::DEF, MonsterAttribute::ATT),
        ("Brave", MonsterAttribute::ATT, MonsterAttribute::SPD),
        ("Calm", MonsterAttribute::SpDEF, MonsterAttribute::ATT),
        ("Careful", MonsterAttribute::SpDEF, MonsterAttribute::SpATT),
        ("Docile", MonsterAttribute::DEF, MonsterAttribute::DEF),
        ("Gentle", MonsterAttribute::SpDEF, MonsterAttribute::DEF),
        ("Hardy", MonsterAttribute::ATT, MonsterAttribute::ATT),
        ("Hasty", MonsterAttribute::SPD, MonsterAttribute::DEF),
        ("Impish", MonsterAttribute::DEF, MonsterAttribute::SpATT),
        ("Jolly", MonsterAttribute::SPD, MonsterAttribute::SpATT),
        ("Lax", MonsterAttribute::DEF, MonsterAttribute::SpDEF),
        ("Lonely", MonsterAttribute::ATT, MonsterAttribute::DEF),
        ("Mild", MonsterAttribute::SpATT, MonsterAttribute::DEF),
        ("Modest", MonsterAttribute::SpATT, MonsterAttribute::ATT),
        ("Naive", MonsterAttribute::SPD, MonsterAttribute::SpDEF),
        ("Naughty", MonsterAttribute::ATT, MonsterAttribute::SpDEF),
        ("Quiet", MonsterAttribute::SpATT, MonsterAttribute::SPD),
        ("Quirky", MonsterAttribute::SpDEF, MonsterAttribute::SpDEF),
        ("Rash", MonsterAttribute::SpATT, MonsterAttribute::SpDEF),
        ("Relaxed", MonsterAttribute::DEF, MonsterAttribute::SPD),
        ("Sassy", MonsterAttribute::SpDEF, MonsterAttribute::SPD),
        ("Serious", MonsterAttribute::SPD, MonsterAttribute::SPD),
        ("Timid", MonsterAttribute::SPD, MonsterAttribute::ATT),
    ];
    pub static ref NATURE_MODIFERS: HashMap<MonsterNature, (MonsterAttribute, MonsterAttribute)> = {
        let mut map: HashMap<MonsterNature, (MonsterAttribute, MonsterAttribute)> = HashMap::new();
        for entry in (*NATURE_ARR).iter() {
            map.insert(MonsterNature::from(entry.0), (entry.1.clone(), entry.2.clone()));
        }
        map
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonsterStats {
    pub internal: [u8; 6],
}

unsafe impl Send for MonsterAttribute {}
unsafe impl Send for MonsterStats {}
unsafe impl Send for MonsterNature {}

unsafe impl Sync for MonsterNature {}
unsafe impl Sync for MonsterAttribute {}
unsafe impl Sync for MonsterStats {}

pub mod error {
    #[derive(Debug)]
    pub enum MonsterError {
        StatParseError,
        MonsterParseError(String),
    }

    impl std::fmt::Display for MonsterError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let msg = match self {
                MonsterError::StatParseError => "Stat parse error.".to_string(),
                MonsterError::MonsterParseError(data) => data.clone(),
            };
            write!(f, "{}", msg)
        }
    }

    impl std::error::Error for MonsterError {}
}

pub mod result {
    pub type Result<T> = std::result::Result<T, super::error::MonsterError>;
}

impl MonsterStats {
    pub fn new() -> Self {
        Self { internal: [0; 6] }
    }
    /**
     * Generate a random set of values.
     * IVs 0-31
     * EVs 0-255 - Capped at 510 total
     */
    pub fn random(max: u8, min: u8) -> Self {
        let mut rng = thread_rng();
        let mut internal: [u8; 6] = [0u8; 6];
        for i in 0..6usize {
            internal[i] = rng.gen_range(min..=max);
        }
        internal.reverse();
        Self {
            internal
        }
    }
    pub fn random_iv() -> Self {
        Self::random(31, 0)
    }
    pub fn random_ev() -> Self {
        let mut temp = Self::random(252, 0);
        let mut total: u16 = 510;
        for i in 0..6u8 {
            let ix = MonsterAttribute::from(i);
            let val = temp.get(ix.clone());
            if total - val as u16 <= total {
                total -= val as u16;
            } else {
                if total > 0 {
                    temp.set(ix, val - total as u8);
                    total = 0;
                } else {
                    temp.set(ix, 0);
                }
            }
        }
        temp
    }
    pub fn get(&self, attribute: MonsterAttribute) -> u8 {
        self.internal[attribute as usize]
    }
    pub fn set(&mut self, attribute: MonsterAttribute, value: u8) {
        self.internal[attribute as usize] = value;
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let sli = &self.internal[..];
        Vec::from(sli)
    }
    pub fn from_bytes(bytes: &[u8]) -> result::Result<Self> {
        if bytes.len() != 6 {
            Err(error::MonsterError::StatParseError)
        } else {
            Ok(Self { internal: [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]] })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum MonsterGender {
    None,
    Male,
    Female,
}

impl MonsterGender {
    pub fn generate() -> Self {
        let mut rng = thread_rng();
        if rng.gen::<bool>() {
            MonsterGender::Male
        } else {
            MonsterGender::Female
        }
    }
}

/// The main Monster structure.
/// Each instance represents a generated Monster.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Monster {
    /// This monster's nickname.
    pub nickname: Option<String>,
    /// The base name of this monster.
    pub base_name: String,
    /// This monster's database ID.
    pub pid: usize,
    /// The generated UUID for this specific monster.
    pub uuid: String,
    /// This monster's level.
    pub level: u8,
    /// This monster's current HP.
    pub hp: u16,
    /// This monster's current XP.
    pub xp: u16,
    /// This monster's required XP for level up.
    pub next_xp: u16,
    /// This monster's type pairing.
    /// Duplicate elements are used for a single-type monster.
    pub elements: (ElementType, ElementType),
    /// This monster's gender.
    pub gender: MonsterGender,
    /// This monster's nature.
    pub nature: MonsterNature,
    /// This monster's moveset.
    /// Each value represents a move ID, with 0 being no move.
    pub moves: (u16, u16, u16, u16),
    /// This monster's currently held item.
    pub held_item: Option<u16>,
    /// This monster's base stats.
    pub base_stats: MonsterStats,
    /// This monster's generated IVs.
    pub iv: MonsterStats,
    /// This monster's EVs.
    pub ev: MonsterStats,
}

unsafe impl Send for Monster {}
unsafe impl Sync for Monster {}

impl Monster {
    pub fn from_dex(id: usize) -> Self {
        let data = PokedexData::get_pokemon(id).unwrap();
        let mut mon = Self::new();
        mon.pid = data.id;
        mon.base_name = data.name.clone();
        mon.base_stats = data.base.clone();
        mon.elements = data.element.clone();
        mon
    }
    pub fn max_hp(&self) -> u16 {
        if self.pid == 292 {
            1
        } else {
            self.get_stat(MonsterAttribute::HP)
        }
    }
    pub fn get_stat(&self, attr: MonsterAttribute) -> u16 {
        let base = self.base_stats.get(attr.clone()) as f32;
        let iv = self.iv.get(attr.clone()) as f32;
        let ev = (self.ev.get(attr.clone()) as f32 * 0.25f32).floor();
        let level = self.level as f32;
        match attr {
            MonsterAttribute::HP => {
                let res = (0.01f32 * (2f32 * base + iv + ev) * level).floor() + level + 10f32;
                res.round() as u16
            }
            _ => {
                let nature_bonus: f32 = {
                    let bn = (*NATURE_MODIFERS).get(&self.nature).unwrap().clone();
                    if bn.0 == attr {
                        1.1f32
                    } else if bn.1 == attr {
                        0.9f32
                    } else {
                        1f32
                    }
                };
                let res = (((0.01f32 * (2f32 * base + iv + ev) * level)).floor() + 5f32) * nature_bonus;
                res.round() as u16
            }
        }
    }
    pub fn get_move(&self, index: usize) -> Option<PokemonMove> {
        let move_id: usize = match index {
            0 => self.moves.0,
            1 => self.moves.1,
            2 => self.moves.2,
            3 => self.moves.3,
            _ => panic!("Index out of range."),
        } as usize;
        if move_id == 0 {
            None
        } else {
            if let Some(entry) = (*POKEMON_MOVES).get(&move_id) {
                Some(entry.clone())
            } else {
                None
            }
        }
    }
    pub fn new() -> Self {
        let iv = MonsterStats::random_iv();
        let ev = MonsterStats::new();
        let stats = MonsterStats::new();
        let mut mon = Self {
            nickname: None,
            base_name: String::new(),
            pid: 0,
            uuid: Uuid::new_v4().to_string(),
            level: 1,
            hp: 1,
            xp: 0,
            next_xp: 50,
            nature: MonsterNature::generate(),
            gender: MonsterGender::generate(),
            moves: (33, 0, 0, 0),
            held_item: None,
            elements: ElementType::generate(),
            base_stats: stats,
            iv,
            ev,
        };
        mon.hp = mon.get_stat(MonsterAttribute::HP);
        mon
    }
}

#[cfg(test)]
mod tests {
    use super::Monster;
    use serde_json;

    #[test]
    fn test_serialize() {
        let t = Monster::from_dex(1);
        let json = serde_json::to_string(&t).unwrap();
        let t2: Monster = serde_json::from_str(&json).unwrap();
        assert_eq!(t, t2);
    }
}