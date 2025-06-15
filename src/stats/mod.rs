pub mod modifier_extractor;
pub mod stat_roller;

use crate::dice::Roll;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
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

#[derive(Deserialize, Clone)]
pub enum SkillType {
    #[serde(rename = "acrobatics")]
    Acrobatics,
    #[serde(rename = "animal-handling")]
    AnimalHandling,
    #[serde(rename = "arcana")]
    Arcana,
    #[serde(rename = "athletics")]
    Athletics,
    #[serde(rename = "deception")]
    Deception,
    #[serde(rename = "history")]
    History,
    #[serde(rename = "insight")]
    Insight,
    #[serde(rename = "intimidation")]
    Intimidation,
    #[serde(rename = "investigation")]
    Investigation,
    #[serde(rename = "medicine")]
    Medicine,
    #[serde(rename = "nature")]
    Nature,
    #[serde(rename = "perception")]
    Perception,
    #[serde(rename = "performance")]
    Performance,
    #[serde(rename = "persuasion")]
    Persuasion,
    #[serde(rename = "religion")]
    Religion,
    #[serde(rename = "sleight-of-hand")]
    SleightOfHand,
    #[serde(rename = "stealth")]
    Stealth,
    #[serde(rename = "survival")]
    Survival,
}

#[derive(Deserialize, Hash, Eq, PartialEq)]
pub enum AdvantageType {
    #[serde(rename = "advantage")]
    Advantage,
    #[serde(rename = "disadvantage")]
    Disadvantage,
}

#[derive(Deserialize, Hash, Eq, PartialEq)]
pub enum Critical {
    #[serde(rename = "critical")]
    Critical,
}

pub struct StatRoll {
    pub first_roll: Vec<Roll>,
    pub second_roll: Option<Vec<Roll>>,
    pub result: i32,
}

pub trait StatRoller {
    fn roll_stat(&self, modifier: i32, advantage_status: &Option<&AdvantageType>) -> StatRoll;
}
