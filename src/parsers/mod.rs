pub mod dice_expression_parser;

use crate::state::Die;

pub trait DiceExpressionParser {
    fn parse(self: &Self, expression: &str) -> Result<Vec<(Die, u32)>, String>;
}
