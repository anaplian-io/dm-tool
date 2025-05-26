mod handlers;
mod parsers;
mod utilities;

use crate::parsers::DiceExpressionParser;
use crate::parsers::dice_expression_parser::DiceExpressionParserImpl;
use crate::utilities::dice_roller::DiceRollerImpl;
use crate::utilities::die_roller::DieRollerImpl;
use crate::utilities::{DiceRoller, DieRoller};
use axum::Router;
use axum::routing::get;
use handlers::roll_dice;
use handlers::roll_dice::RollDiceHandlerDependencies;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let (dice_expression_parser, dice_roller, _) = build_dependencies();
    let app = Router::new().route(
        "/v1/dice/roll/{roll_expression}",
        get(roll_dice::roll_dice).with_state(RollDiceHandlerDependencies {
            dice_expression_parser: dice_expression_parser.clone(),
            dice_roller: dice_roller.clone(),
        }),
    );
    let listener = TcpListener::bind(("0.0.0.0", 8080)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn build_dependencies() -> (
    Arc<dyn DiceExpressionParser + Send + Sync>,
    Arc<dyn DiceRoller + Send + Sync>,
    Arc<dyn DieRoller + Send + Sync>,
) {
    let dice_expression_parser = Arc::new(DiceExpressionParserImpl::default());
    let die_roller = Arc::new(DieRollerImpl::default());
    let dice_roller = Arc::new(DiceRollerImpl::new(die_roller.clone()));
    (dice_expression_parser, dice_roller, die_roller)
}
