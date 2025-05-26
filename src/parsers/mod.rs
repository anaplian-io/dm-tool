pub mod dice_expression_parser;

use crate::state::Die;

pub trait DiceExpressionParser {
    fn parse(&self, expression: &str) -> Result<Vec<(Die, i32)>, String>;
}
