mod handlers;
mod parsers;
mod state;
mod utilities;

use crate::parsers::dice_expression_parser::DiceExpressionParserImpl;
use crate::state::HandlerDependencies;
use crate::utilities::die_roller::DieRollerImpl;
use axum::routing::get;
use axum::Router;
use handlers::roll_dice;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let dependencies = Arc::new(HandlerDependencies {
        dice_expression_parser: Box::new(DiceExpressionParserImpl::default()),
        die_roller: Box::new(DieRollerImpl::default()),
    });
    let app = Router::new().route(
        "/roll/{roll_expression}",
        get(roll_dice::roll_dice).with_state(dependencies.clone()),
    );
    let listener = TcpListener::bind(("0.0.0.0", 8080)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
