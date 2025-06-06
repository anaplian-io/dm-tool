pub mod stat_roller;

use crate::dice::Roll;
use serde::Deserialize;

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

pub struct StatRoll {
    pub first_roll: Vec<Roll>,
    pub second_roll: Option<Vec<Roll>>,
    pub result: i32,
}

pub trait StatRoller {
    fn roll_stat(&self, modifier: i32, advantage_status: &Option<&AdvantageType>) -> StatRoll;
}
