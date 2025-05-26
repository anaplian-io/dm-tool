use crate::parsers::DiceExpressionParser;
use crate::utilities::DieRoller;
use serde::Serialize;

pub struct HandlerDependencies {
    pub(crate) dice_expression_parser: Box<dyn DiceExpressionParser + Send + Sync>,
    pub(crate) die_roller: Box<dyn DieRoller + Send + Sync>,
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
