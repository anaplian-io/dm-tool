use crate::dice::Roll;
use crate::monsters::Monster;
use crate::stats::StatRoller;
use crate::stats::modifier_extractor::ModifierExtractor;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

pub mod get_monster;
pub mod list_dice;
pub mod list_monsters;
pub mod roll_attack;
pub mod roll_dice;
pub mod roll_stat;

#[derive(Serialize, Debug)]
pub struct StatRollResponse {
    #[serde(rename = "firstRoll")]
    first_roll: Vec<Roll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "secondRoll")]
    second_roll: Option<Vec<Roll>>,
    result: i32,
}

#[derive(Clone)]
pub struct MonsterRollerDependencies<T> {
    pub(crate) monster_map: Arc<HashMap<String, Monster>>,
    pub(crate) stats_roller: Arc<dyn StatRoller + Sync + Send>,
    pub(crate) modifier_extractor: Arc<ModifierExtractor<T>>,
}
