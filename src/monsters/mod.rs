pub mod monster;
pub(crate) mod search;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use strum_macros::Display;

pub trait Tokenize {
    fn tokenize(&self) -> HashSet<String>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Monster {
    pub name: String,
    pub ac: i32,
    pub size: Size,
    pub alignment: String,
    pub languages: Vec<String>,
    #[serde(rename = "creatureType")]
    pub creature_type: String,
    #[serde(rename = "maxHitPoints")]
    pub max_hit_points: i32,
    #[serde(rename = "hitDice")]
    pub hit_dice: String,
    pub speed: Speed,
    pub modifiers: Stats,
    pub stats: Stats,
    #[serde(rename = "savingThrows")]
    pub saving_throws: Stats,
    pub skills: Skills,
    pub traits: Vec<String>,
    pub actions: Option<Actions>,
    #[serde(rename = "legendaryActions")]
    pub legendary_actions: Vec<String>,
    pub reactions: Vec<String>,
    pub challenge: Challenge,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
pub enum Size {
    #[serde(rename = "gargantuan")]
    Gargantuan,
    #[serde(rename = "huge")]
    Huge,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "tiny")]
    Tiny,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Speed {
    pub walk: i32,
    pub fly: i32,
    pub swim: i32,
    pub burrow: i32,
    pub climb: i32,
    pub hover: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stats {
    #[serde(rename = "str")]
    pub strength: i32,
    #[serde(rename = "dex")]
    pub dexterity: i32,
    #[serde(rename = "con")]
    pub constitution: i32,
    #[serde(rename = "int")]
    pub intelligence: i32,
    #[serde(rename = "wis")]
    pub wisdom: i32,
    #[serde(rename = "cha")]
    pub charisma: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Skills {
    pub acrobatics: i32,
    pub arcana: i32,
    pub athletics: i32,
    pub deception: i32,
    pub history: i32,
    pub insight: i32,
    pub intimidation: i32,
    pub investigation: i32,
    pub medicine: i32,
    pub nature: i32,
    pub perception: i32,
    pub performance: i32,
    pub persuasion: i32,
    pub religion: i32,
    #[serde(rename = "sleightOfHand")]
    pub sleight_of_hand: i32,
    pub stealth: i32,
    pub survival: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Actions {
    pub list: Vec<String>,
    #[serde(rename = "attackRolls")]
    pub attack_rolls: Vec<AttackRoll>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttackRoll {
    pub name: String,
    #[serde(rename = "attackType")]
    pub attack_type: AttackType,
    pub reach: i32,
    pub hit: i32,
    pub damage: Vec<DamageRoll>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DamageRoll {
    #[serde(rename = "damageType")]
    pub damage_type: DamageType,
    pub roll: String,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
pub enum AttackType {
    #[serde(rename = "meleeWeapon")]
    MeleeWeapon,
    #[serde(rename = "rangedWeapon")]
    RangedWeapon,
    #[serde(rename = "meleeSpell")]
    MeleeSpell,
    #[serde(rename = "rangedSpell")]
    RangedSpell,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
pub enum DamageType {
    #[serde(rename = "bludgeoning")]
    Bludgeoning,
    #[serde(rename = "piercing")]
    Piercing,
    #[serde(rename = "slashing")]
    Slashing,
    #[serde(rename = "acid")]
    Acid,
    #[serde(rename = "lightning")]
    Lightning,
    #[serde(rename = "poison")]
    Poison,
    #[serde(rename = "fire")]
    Fire,
    #[serde(rename = "cold")]
    Cold,
    #[serde(rename = "radiant")]
    Radiant,
    #[serde(rename = "necrotic")]
    Necrotic,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Challenge {
    pub rating: String,
    pub xp: i32,
}
