mod dice;
mod handlers;
mod monsters;
mod utilities;

use crate::dice::DiceRoller;
use crate::dice::dice_roller::DiceRollerImpl;
use crate::dice::die_roller::DieRollerImpl;
use crate::handlers::list_monsters::ListMonstersDependencies;
use crate::handlers::{list_dice, list_monsters};
use crate::monsters::Monster;
use crate::monsters::search::MonsterSearch;
use crate::utilities::index::vec_to_map;
use crate::utilities::load_from_json::load_from_json;
use axum::Router;
use axum::routing::get;
use dice::DiceExpressionParser;
use dice::dice_expression_parser::DiceExpressionParserImpl;
use handlers::roll_dice;
use handlers::roll_dice::RollDiceHandlerDependencies;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let dependencies = build_dependencies();
    let app = Router::new()
        .route(
            "/v1/dice/roll/{roll_expression}",
            get(roll_dice::roll_dice).with_state(RollDiceHandlerDependencies {
                dice_expression_parser: dependencies.dice_expression_parser,
                dice_roller: dependencies.dice_roller,
            }),
        )
        .route("/v1/dice/list", get(list_dice::list_dice))
        .route(
            "/v1/monsters",
            get(list_monsters::list_monsters).with_state(ListMonstersDependencies {
                monsters: dependencies.monsters,
                monster_search: dependencies.monster_search,
            }),
        );
    let listener = TcpListener::bind(("0.0.0.0", 8080)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct Dependencies {
    dice_expression_parser: Arc<dyn DiceExpressionParser + Send + Sync>,
    dice_roller: Arc<dyn DiceRoller + Send + Sync>,
    monsters: Arc<Vec<Monster>>,
    monster_search: Arc<MonsterSearch>,
}

fn build_dependencies() -> Dependencies {
    let dice_expression_parser = Arc::new(DiceExpressionParserImpl::default());
    let die_roller = Arc::new(DieRollerImpl::default());
    let dice_roller = Arc::new(DiceRollerImpl::new(die_roller.clone()));
    let monsters = Arc::new(load_from_json::<Vec<Monster>>("user_data/monsters.json"));
    let monster_search = Arc::new(MonsterSearch::from_map(Arc::new(vec_to_map(
        &monsters,
        |monster| monster.name.clone(),
    ))));
    Dependencies {
        dice_expression_parser,
        dice_roller,
        monsters,
        monster_search,
    }
}
