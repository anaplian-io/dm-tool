use crate::state::{Die, HandlerDependencies};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize, PartialEq)]
struct Roll {
    die: Die,
    value: i32,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct RollDiceResponse {
    expression: String,
    total: i32,
    rolls: Vec<Roll>,
}

pub async fn roll_dice(
    Path(expression): Path<String>,
    State(dependencies): State<Arc<HandlerDependencies>>,
) -> Result<Json<RollDiceResponse>, (StatusCode, String)> {
    let parsed_dice_expression_result = match dependencies.dice_expression_parser.parse(&expression)
    {
        Ok(result) => result,
        Err(error) => {
            return Err((StatusCode::BAD_REQUEST, error.to_string()));
        }
    };
    let rolls = parsed_dice_expression_result
        .iter()
        .flat_map(|(die, n)| match die {
            Die::RAW => vec![Roll {
                die: Die::RAW,
                value: *n as i32,
            }],
            die => (0..*n)
                .into_iter()
                .map(|_| Roll {
                    die: die.clone(),
                    value: dependencies.die_roller.roll(die) as i32,
                })
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<Roll>>();
    let total = rolls
        .iter()
        .map(|roll| roll.value)
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    Ok(Json(RollDiceResponse {
        expression,
        total,
        rolls,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::DiceExpressionParser;
    use crate::state::Die;
    use crate::state::Die::{D6, RAW};
    use crate::utilities::DieRoller;

    #[tokio::test]
    async fn test_roll_dice_error() {
        struct MockDiceExpressionParser {}
        impl DiceExpressionParser for MockDiceExpressionParser {
            fn parse(self: &Self, _: &str) -> Result<Vec<(Die, u32)>, String> {
                Err(String::from("Invalid dice expression"))
            }
        }
        struct MockDieRoller {}
        impl DieRoller for MockDieRoller {
            fn roll(self: &Self, _: &Die) -> u32 {
                panic!("Dice roller should not have been executed")
            }
        }

        let state: Arc<HandlerDependencies> = Arc::new(HandlerDependencies {
            die_roller: Box::new(MockDieRoller {}),
            dice_expression_parser: Box::new(MockDiceExpressionParser {}),
        });

        let result = roll_dice(Path("fake-expression".to_string()), State(state)).await;

        assert_eq!(result.is_err(), true);
    }

    #[tokio::test]
    async fn test_roll_dice_success() {
        struct MockDiceExpressionParser {}
        impl DiceExpressionParser for MockDiceExpressionParser {
            fn parse(self: &Self, _: &str) -> Result<Vec<(Die, u32)>, String> {
                Ok(vec![(D6, 2), (RAW, 5)])
            }
        }
        struct MockDieRoller {}
        impl DieRoller for MockDieRoller {
            fn roll(self: &Self, _: &Die) -> u32 {
                3
            }
        }

        let state: Arc<HandlerDependencies> = Arc::new(HandlerDependencies {
            die_roller: Box::new(MockDieRoller {}),
            dice_expression_parser: Box::new(MockDiceExpressionParser {}),
        });

        let result = roll_dice(Path("fake-expression".to_string()), State(state))
            .await
            .unwrap();

        assert_eq!(
            result.0,
            RollDiceResponse {
                expression: "fake-expression".to_string(),
                total: 11,
                rolls: vec![
                    Roll { die: D6, value: 3 },
                    Roll { die: D6, value: 3 },
                    Roll { die: RAW, value: 5 }
                ],
            }
        );
    }
}
