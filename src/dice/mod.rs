pub mod dice_expression_parser;
pub mod dice_roller;
pub mod die_roller;

use enum_iterator::Sequence;
use serde::Serialize;
use strum_macros::Display;

pub trait DieRoller {
    fn roll(&self, die: &Die) -> i32;
}

pub trait DiceRoller {
    fn roll(&self, dice: &[(Die, i32)]) -> (Vec<Roll>, i32);
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct Roll {
    pub die: Die,
    pub value: i32,
}

#[derive(Debug, Serialize, PartialEq, Clone, Sequence, Display)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    Raw,
}

pub trait DiceExpressionParser {
    fn parse(&self, expression: &str) -> Result<Vec<(Die, i32)>, String>;
}
