mod dice;
mod handlers;
mod monsters;
mod utilities;

use crate::dice::DiceRoller;
use crate::dice::dice_roller::DiceRollerImpl;
use crate::dice::die_roller::DieRollerImpl;
use crate::handlers::get_monster::GetMonsterDependencies;
use crate::handlers::list_monsters::ListMonstersDependencies;
use crate::handlers::roll_saving_throw::RollSavingThrowDependencies;
use crate::handlers::{get_monster, list_dice, list_monsters, roll_saving_throw};
use crate::monsters::Monster;
use crate::monsters::search::MonsterSearch;
use crate::utilities::MONSTERS_JSON_PATH;
use crate::utilities::index::vec_to_map;
use crate::utilities::load_from_json::load_from_json;
use axum::Router;
use axum::routing::get;
use dice::DiceExpressionParser;
use dice::dice_expression_parser::DiceExpressionParserImpl;
use handlers::roll_dice;
use handlers::roll_dice::RollDiceHandlerDependencies;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let dependencies = build_dependencies();
    let app = Router::new()
        .route(
            "/v1/dice/roll/{roll_expression}",
            get(roll_dice::roll_dice).with_state(RollDiceHandlerDependencies {
                dice_expression_parser: dependencies.dice_expression_parser.clone(),
                dice_roller: dependencies.dice_roller.clone(),
            }),
        )
        .route("/v1/dice/list", get(list_dice::list_dice))
        .route(
            "/v1/monsters",
            get(list_monsters::list_monsters).with_state(ListMonstersDependencies {
                monsters: dependencies.monsters.clone(),
                monster_search: dependencies.monster_search.clone(),
            }),
        )
        .route(
            "/v1/monsters/{monster_name}",
            get(get_monster::get_monster).with_state(GetMonsterDependencies {
                monster_map: dependencies.monster_map.clone(),
            }),
        )
        .route(
            "/v1/monsters/{monster_name}/roll/throw/{stat}",
            get(roll_saving_throw::roll_saving_throw).with_state(RollSavingThrowDependencies {
                monster_map: dependencies.monster_map.clone(),
                dice_roller: dependencies.dice_roller.clone(),
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
    monster_map: Arc<HashMap<String, Monster>>,
}

fn build_dependencies() -> Dependencies {
    let dice_expression_parser = Arc::new(DiceExpressionParserImpl::default());
    let die_roller = Arc::new(DieRollerImpl::default());
    let dice_roller = Arc::new(DiceRollerImpl::new(die_roller.clone()));
    let monsters = Arc::new(load_from_json::<Vec<Monster>>(MONSTERS_JSON_PATH));
    let monster_map = Arc::new(vec_to_map(&monsters, |monster| monster.name.to_lowercase()));
    let monster_search = Arc::new(MonsterSearch::from_map(monster_map.clone()));
    Dependencies {
        dice_expression_parser,
        dice_roller,
        monsters,
        monster_search,
        monster_map,
    }
}
