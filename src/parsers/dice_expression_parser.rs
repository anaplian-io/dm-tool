use crate::parsers::DiceExpressionParser;
use crate::utilities::Die;
use regex::Regex;

pub struct DiceExpressionParserImpl {
    eval_regex: Regex,
}

impl DiceExpressionParserImpl {
    fn new(eval_regex: Regex) -> Self {
        Self { eval_regex }
    }

    pub fn default() -> Self {
        Self::new(Regex::new(r"^-?(\d+)?(d4|d6|d8|d10|d12|d20)?$").unwrap())
    }
}

impl DiceExpressionParser for DiceExpressionParserImpl {
    fn parse(&self, expression: &str) -> Result<Vec<(Die, i32)>, String> {
        let parsed_die_expression: Vec<(Die, i32)> = expression
            .to_lowercase()
            .replace('-', "+-")
            .split("+")
            .filter(|s| self.eval_regex.is_match(s))
            .map(|s| {
                if s.contains('d') {
                    let parts = s.split('d').collect::<Vec<&str>>();
                    return (
                        match *parts.get(1).unwrap_or(&"") {
                            "4" => Die::D4,
                            "6" => Die::D6,
                            "8" => Die::D8,
                            "10" => Die::D10,
                            "12" => Die::D12,
                            _ => Die::D20,
                        },
                        parts.first().unwrap_or(&"0").parse::<i32>().unwrap_or(0),
                    );
                }
                (Die::Raw, s.parse::<i32>().unwrap_or(0))
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
    use crate::utilities::Die;

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
                (Die::Raw, 1234),
            ]
        );
    }

    #[test]
    fn test_dice_expression_parser_err() {
        let parser = DiceExpressionParserImpl::default();

        let result = parser.parse("This is not a valid dice expression");
        assert!(result.is_err());
    }

    #[test]
    fn test_dice_expression_with_uppercase_ok() {
        let parser = DiceExpressionParserImpl::default();

        let result = parser.parse("1D20+2D12");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![(Die::D20, 1), (Die::D12, 2),]);
    }

    #[test]
    fn test_dice_expression_with_negatives_ok() {
        let parser = DiceExpressionParserImpl::default();

        let result = parser.parse("4d10-2d4-3");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![(Die::D10, 4), (Die::D4, -2), (Die::Raw, -3),]
        );
    }
}
