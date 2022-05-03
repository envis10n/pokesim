use uuid::Uuid;

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

pub enum MonsterAttribute {
    HP,
    ATT,
    DEF,
    SPD,
    SpATT,
    SpDEF,
}

pub struct MonsterStats {
    internal: [u8;6],
}

impl MonsterStats {
    pub fn new() -> Self {
        Self {internal: [0;6]}
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
}

pub struct Monster {
    nickname: String,
    id: Uuid,
    level: u8,
    xp: u16,
    next_xp: u16,
    iv: MonsterStats,
    iv_mod: MonsterStats,
}

impl Default for Monster {
    fn default() -> Self {
        Self {
            nickname: String::new(),
            id: Uuid::new_v4(),
            level: 1,
            xp: 0,
            next_xp: 50,
            iv: MonsterStats::new(),
            iv_mod: MonsterStats::new()
        }
    }
}

impl Monster {
    pub fn new(name: &str) -> Self {
        Self {
            nickname: String::from(name),
            id: Uuid::new_v4(),
            level: 1,
            xp: 0,
            next_xp: 50,
            iv: MonsterStats::new(),
            iv_mod: MonsterStats::new(),
        }
    }
}
