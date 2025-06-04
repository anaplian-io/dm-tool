use serde::Deserialize;

pub mod get_monster;
pub mod list_dice;
pub mod list_monsters;
pub mod roll_dice;
pub mod roll_saving_throw;

#[derive(Deserialize)]
pub enum StatType {
    #[serde(rename = "str")]
    Strength,
    #[serde(rename = "dex")]
    Dexterity,
    #[serde(rename = "con")]
    Constitution,
    #[serde(rename = "int")]
    Intelligence,
    #[serde(rename = "wis")]
    Wisdom,
    #[serde(rename = "cha")]
    Charisma,
}

#[derive(Deserialize, Hash, Eq, PartialEq)]
pub enum AdvantageType {
    #[serde(rename = "advantage")]
    Advantage,
    #[serde(rename = "disadvantage")]
    Disadvantage,
}
