use crate::dice::DiceExpressionParser;
use crate::dice::{DiceRoller, Roll};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct RollDiceHandlerDependencies {
    pub(crate) dice_expression_parser: Arc<dyn DiceExpressionParser + Send + Sync>,
    pub(crate) dice_roller: Arc<dyn DiceRoller + Send + Sync>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct RollDiceResponse {
    expression: String,
    total: i32,
    rolls: Vec<Roll>,
}

pub async fn roll_dice(
    Path(expression): Path<String>,
    State(dependencies): State<RollDiceHandlerDependencies>,
) -> Result<Json<RollDiceResponse>, (StatusCode, String)> {
    let parsed_dice_expression_result = match dependencies.dice_expression_parser.parse(&expression)
    {
        Ok(result) => result,
        Err(error) => {
            return Err((StatusCode::BAD_REQUEST, error.to_string()));
        }
    };
    let (rolls, total) = dependencies
        .dice_roller
        .roll(&parsed_dice_expression_result);
    Ok(Json(RollDiceResponse {
        expression,
        total,
        rolls,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice::DiceExpressionParser;
    use crate::dice::Die;
    use crate::dice::Die::{D6, Raw};
    use crate::dice::DieRoller;
    use crate::dice::dice_roller::DiceRollerImpl;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_roll_dice_error() {
        struct MockDiceExpressionParser {}
        impl DiceExpressionParser for MockDiceExpressionParser {
            fn parse(&self, _: &str) -> Result<Vec<(Die, i32)>, String> {
                Err(String::from("Invalid dice expression"))
            }
        }
        struct MockDieRoller {}
        impl DieRoller for MockDieRoller {
            fn roll(&self, _: &Die) -> i32 {
                panic!("Dice roller should not have been executed")
            }
        }

        let state = RollDiceHandlerDependencies {
            dice_roller: Arc::new(DiceRollerImpl::new(Arc::new(MockDieRoller {}))),
            dice_expression_parser: Arc::new(MockDiceExpressionParser {}),
        };

        let result = roll_dice(Path("fake-expression".to_string()), State(state)).await;

        assert_eq!(result.is_err(), true);
    }

    #[tokio::test]
    async fn test_roll_dice_success() {
        struct MockDiceExpressionParser {}
        impl DiceExpressionParser for MockDiceExpressionParser {
            fn parse(&self, _: &str) -> Result<Vec<(Die, i32)>, String> {
                Ok(vec![(D6, 2), (Raw, 5)])
            }
        }
        struct MockDieRoller {}
        impl DieRoller for MockDieRoller {
            fn roll(&self, _: &Die) -> i32 {
                3
            }
        }

        let state = RollDiceHandlerDependencies {
            dice_roller: Arc::new(DiceRollerImpl::new(Arc::new(MockDieRoller {}))),
            dice_expression_parser: Arc::new(MockDiceExpressionParser {}),
        };

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
                    Roll { die: Raw, value: 5 }
                ],
            }
        );
    }
}
