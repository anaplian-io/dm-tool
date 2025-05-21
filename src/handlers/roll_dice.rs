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
