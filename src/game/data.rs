use serde::Deserialize;

/// Entry representing the JSON data for Types.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct TypesEntry {
    pub english: String,
    pub chinese: Option<String>,
    pub japanese: Option<String>,
}

/// Entry representing the JSON data for Items.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ItemsEntry {
    pub id: u64,
    pub name: TypesEntry,
}

/// Entry representing the JSON data for Moves.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct MovesEntry {
    pub accuracy: Option<u8>,
    #[serde(alias = "type")]
    pub _type: String,
    pub cname: String,
    pub ename: String,
    pub jname: String,
    pub id: u64,
    pub power: Option<u8>,
    pub pp: Option<u8>,
}

/// Entry representing a Pokedex entry's name object.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct PokedexNameEntry {
    pub english: String,
    pub japanese: String,
    pub chinese: String,
    pub french: String,
}

/// Entry representing the JSON data for Pokemon's base stats.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct PokedexBaseEntry {
    #[serde(alias = "HP")]
    pub hp: u8,
    #[serde(alias = "Attack")]
    pub attack: u8,
    #[serde(alias = "Defense")]
    pub defense: u8,
    #[serde(alias = "Sp. Attack")]
    pub sp_attack: u8,
    #[serde(alias = "Sp. Defense")]
    pub sp_defense: u8,
    #[serde(alias = "Speed")]
    pub speed: u8,
}

/// Entry representing the JSON data for Pokemon.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct PokedexEntry {
    pub id: u64,
    pub name: PokedexNameEntry,
    #[serde(alias = "type")]
    pub _type: Vec<String>,
    pub base: PokedexBaseEntry,
}
