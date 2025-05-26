pub mod dice_roller;
pub mod die_roller;

use serde::Serialize;

pub trait DieRoller {
    fn roll(&self, die: &Die) -> i32;
}

pub trait DiceRoller {
    fn roll(&self, dice: &[(Die, i32)]) -> (Vec<Roll>, i32);
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Roll {
    pub die: Die,
    pub value: i32,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    Raw,
}
