use uuid::Uuid;
use rand::prelude::*;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum MonsterElement {
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

impl From<&str> for MonsterElement {
    fn from(v: &str) -> Self {
        match v {
            "Normal" => MonsterElement::Normal,
            "Fire" => MonsterElement::Fire,
            "Water" => MonsterElement::Water,
            "Electric" => MonsterElement::Electric,
            "Grass" => MonsterElement::Grass,
            "Ice" => MonsterElement::Ice,
            "Fighting" => MonsterElement::Fighting,
            "Poison" => MonsterElement::Poison,
            "Ground" => MonsterElement::Ground,
            "Flying" => MonsterElement::Flying,
            "Psychic" => MonsterElement::Psychic,
            "Bug" => MonsterElement::Bug,
            "Rock" => MonsterElement::Rock,
            "Ghost" => MonsterElement::Ghost,
            "Dragon" => MonsterElement::Dragon,
            "Dark" => MonsterElement::Dark,
            "Steel" => MonsterElement::Steel,
            "Fairy" => MonsterElement::Fairy,
            _ => panic!("Unsupported element type."),
        }
    }
}

#[repr(u8)]
#[derive(Clone)]
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
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct MonsterStats {
    internal: [u8; 6],
}

unsafe impl Send for MonsterElement {}
unsafe impl Send for MonsterAttribute {}
unsafe impl Send for MonsterStats {}
unsafe impl Send for MonsterNature {}

unsafe impl Sync for MonsterElement {}
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

impl Default for MonsterStats {
    fn default() -> Self {
        let mut temp = [0u8; 6];
        let mut rng = thread_rng();
        for i in 0..5 as usize {
            temp[i] = rng.gen_range(0..=31);
        }
        Self {
            internal: temp,
        }
    }
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
        Self {
            internal: [
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
            ]
        }
    }
    pub fn random_iv() -> Self {
        Self::random(31, 0)
    }
    pub fn random_ev() -> Self {
        let mut temp = Self::random(252, 0);
        let mut total: u16 = 510;
        for i in 0..5 {
            let ix = MonsterAttribute::from(i as u8);
            let val = temp.get(ix.clone());
            if total - val as u16 <= total {
                total -= val as u16;
            } else {
                if total > 0 {
                    temp.set(ix, (val - total as u8));
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

#[derive(Debug, Clone)]
pub struct Monster {
    pub nickname: String,
    pub id: Uuid,
    pub level: u8,
    pub hp: u16,
    pub xp: u16,
    pub next_xp: u16,
    pub nature: MonsterNature,
    pub stats: MonsterStats,
    pub iv: MonsterStats,
    pub ev: MonsterStats,
}

unsafe impl Send for Monster {}
unsafe impl Sync for Monster {}

/*
    HP = ((2*Base + IV + EV/4 + 100) * Level) / 100 + 10

    Stat = (((2*Base + IV + EV/4) * Level) / 100 + 5) * Nature
    The Nature modifier is 1.1 positive 1 for neutral 0.9 for negative.
*/

impl Monster {
    pub fn get_stat(&self, attr: MonsterAttribute) -> u16 {
        unimplemented!("")
    }
    pub fn new() -> Self {
        let iv = MonsterStats::default();
        let ev = MonsterStats::new();
        let stats = MonsterStats::new();
        Self {
            nickname: Default::default(),
            id: Uuid::new_v4(),
            level: 1,
            hp: 1,
            xp: 1,
            next_xp: 50,
            nature: MonsterNature::generate(),
            stats,
            iv,
            ev,
        }
    }
}