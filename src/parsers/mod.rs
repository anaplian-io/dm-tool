pub mod dice_expression_parser;

use crate::utilities::Die;

pub trait DiceExpressionParser {
    fn parse(&self, expression: &str) -> Result<Vec<(Die, i32)>, String>;
}
