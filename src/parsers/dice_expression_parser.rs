use crate::parsers::DiceExpressionParser;
use crate::state::Die;
use regex::Regex;

pub struct DiceExpressionParserImpl {
    eval_regex: Regex,
}

impl DiceExpressionParserImpl {
    fn new(eval_regex: Regex) -> Self {
        Self { eval_regex }
    }

    pub fn default() -> Self {
        Self::new(Regex::new(r"^(\d+)?(d4|d6|d8|d10|d12|d20)?$").unwrap())
    }
}

impl DiceExpressionParser for DiceExpressionParserImpl {
    fn parse(self: &Self, expression: &str) -> Result<Vec<(Die, u32)>, String> {
        let parsed_die_expression: Vec<(Die, u32)> = expression
            .split("+")
            .filter(|s| self.eval_regex.is_match(s))
            .map(|s| {
                if s.contains('d') {
                    let parts = s.split('d').collect::<Vec<&str>>();
                    return (
                        match parts.get(1).unwrap_or(&"") {
                            &"4" => Die::D4,
                            &"6" => Die::D6,
                            &"8" => Die::D8,
                            &"10" => Die::D10,
                            &"12" => Die::D12,
                            &"20" => Die::D20,
                            _ => Die::RAW,
                        },
                        u32::from_str_radix(parts.get(0).unwrap_or(&"0"), 10).unwrap_or(0),
                    );
                }
                (Die::RAW, u32::from_str_radix(s, 10).unwrap_or(0))
            })
            .collect();
        if parsed_die_expression.is_empty() {
            return Err(format!("\"{}\" is not a valid dice expression", expression));
        }
        Ok(parsed_die_expression)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Die;

    #[test]
    fn test_dice_expression_parser_ok() {
        let parser = DiceExpressionParserImpl::default();

        let result = parser.parse("1d20+2d12+3d10+4d8+15d6+200d4+1234");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                (Die::D20, 1),
                (Die::D12, 2),
                (Die::D10, 3),
                (Die::D8, 4),
                (Die::D6, 15),
                (Die::D4, 200),
                (Die::RAW, 1234)
            ]
        );
    }

    #[test]
    fn test_dice_expression_parser_err() {
        let parser = DiceExpressionParserImpl::default();

        let result = parser.parse("This is not a valid dice expression");
        assert!(result.is_err());
    }
}
