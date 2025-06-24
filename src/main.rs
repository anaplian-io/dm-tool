mod dice;
mod handlers;
mod monsters;
mod stats;
mod utilities;

use crate::dice::DiceRoller;
use crate::dice::dice_roller::DiceRollerImpl;
use crate::dice::die_roller::DieRollerImpl;
use crate::handlers::get_monster::GetMonsterDependencies;
use crate::handlers::list_monsters::ListMonstersDependencies;
use crate::handlers::roll_attack::RollAttackDependencies;
use crate::handlers::{get_monster, list_dice, list_monsters, roll_attack, roll_stat};
use crate::monsters::Monster;
use crate::monsters::search::MonsterSearch;
use crate::stats::modifier_extractor::{
    ModifierExtractor, build_attack_modifier_extractor, build_saving_throw_modifier_extractor,
    build_skill_modifier_extractor, build_stat_modifier_extractor,
};
use crate::stats::stat_roller::StatRollerImpl;
use crate::stats::{SkillType, StatRoller, StatType};
use crate::utilities::MONSTERS_JSON_PATH;
use crate::utilities::index::vec_to_map;
use crate::utilities::load_from_json::load_from_json;
use axum::Router;
use axum::routing::get;
use dice::DiceExpressionParser;
use dice::dice_expression_parser::DiceExpressionParserImpl;
use handlers::MonsterRollerDependencies;
use handlers::roll_dice;
use handlers::roll_dice::RollDiceHandlerDependencies;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let dependencies = build_dependencies();
    let app = Router::new()
        .route(
            "/v1/dice/roll/{roll_expression}",
            get(roll_dice::roll_dice)
                .with_state(RollDiceHandlerDependencies {
                    dice_expression_parser: dependencies.dice_expression_parser.clone(),
                    dice_roller: dependencies.dice_roller.clone(),
                })
                .layer(TraceLayer::new_for_http()),
        )
        .route(
            "/v1/dice/list",
            get(list_dice::list_dice).layer(TraceLayer::new_for_http()),
        )
        .route(
            "/v1/monsters",
            get(list_monsters::list_monsters)
                .with_state(ListMonstersDependencies {
                    monsters: dependencies.monsters.clone(),
                    monster_search: dependencies.monster_search.clone(),
                })
                .layer(TraceLayer::new_for_http()),
        )
        .route(
            "/v1/monsters/{monster_name}",
            get(get_monster::get_monster)
                .with_state(GetMonsterDependencies {
                    monster_map: dependencies.monster_map.clone(),
                })
                .layer(TraceLayer::new_for_http()),
        )
        .route(
            "/v1/monsters/{monster_name}/roll/throw/{stat}",
            get(roll_stat::roll_stat)
                .with_state(MonsterRollerDependencies {
                    monster_map: dependencies.monster_map.clone(),
                    stats_roller: dependencies.stat_roller.clone(),
                    modifier_extractor: dependencies.saving_throw_modifier_extractor.clone(),
                })
                .layer(TraceLayer::new_for_http()),
        )
        .route(
            "/v1/monsters/{monster_name}/roll/skill/{skill}",
            get(roll_stat::roll_stat)
                .with_state(MonsterRollerDependencies {
                    monster_map: dependencies.monster_map.clone(),
                    stats_roller: dependencies.stat_roller.clone(),
                    modifier_extractor: dependencies.skill_modifier_extractor.clone(),
                })
                .layer(TraceLayer::new_for_http()),
        )
        .route(
            "/v1/monsters/{monster_name}/roll/stat/{stat}",
            get(roll_stat::roll_stat)
                .with_state(MonsterRollerDependencies {
                    monster_map: dependencies.monster_map.clone(),
                    stats_roller: dependencies.stat_roller.clone(),
                    modifier_extractor: dependencies.stat_modifier_extractor.clone(),
                })
                .layer(TraceLayer::new_for_http()),
        )
        .route(
            "/v1/monsters/{monster_name}/roll/attack/{index}",
            get(roll_stat::roll_stat)
                .with_state(MonsterRollerDependencies {
                    monster_map: dependencies.monster_map.clone(),
                    stats_roller: dependencies.stat_roller.clone(),
                    modifier_extractor: dependencies.attack_modifier_extractor.clone(),
                })
                .layer(TraceLayer::new_for_http()),
        )
        .route(
            "/v1/monsters/{monster_name}/roll/damage/{index}",
            get(roll_attack::roll_attack)
                .with_state(RollAttackDependencies {
                    dice_expression_parser: dependencies.dice_expression_parser.clone(),
                    dice_roller: dependencies.dice_roller.clone(),
                    monster_map: dependencies.monster_map.clone(),
                })
                .layer(TraceLayer::new_for_http()),
        );
    let listener = TcpListener::bind(("0.0.0.0", 8080)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct Dependencies {
    dice_expression_parser: Arc<dyn DiceExpressionParser + Send + Sync>,
    dice_roller: Arc<dyn DiceRoller + Send + Sync>,
    stat_roller: Arc<dyn StatRoller + Send + Sync>,
    monsters: Arc<Vec<Monster>>,
    monster_search: Arc<MonsterSearch>,
    monster_map: Arc<HashMap<String, Monster>>,
    saving_throw_modifier_extractor: Arc<ModifierExtractor<StatType>>,
    skill_modifier_extractor: Arc<ModifierExtractor<SkillType>>,
    stat_modifier_extractor: Arc<ModifierExtractor<StatType>>,
    attack_modifier_extractor: Arc<ModifierExtractor<usize>>,
}

fn build_dependencies() -> Dependencies {
    let dice_expression_parser = Arc::new(DiceExpressionParserImpl::default());
    let die_roller = Arc::new(DieRollerImpl::default());
    let dice_roller = Arc::new(DiceRollerImpl::new(die_roller.clone()));
    let stat_roller = Arc::new(StatRollerImpl::new(dice_roller.clone()));
    let monsters = Arc::new(load_from_json::<Vec<Monster>>(MONSTERS_JSON_PATH));
    let monster_map = Arc::new(vec_to_map(&monsters, |monster| monster.name.to_lowercase()));
    let monster_search = Arc::new(MonsterSearch::from_map(monster_map.clone()));
    let saving_throw_modifier_extractor = Arc::new(build_saving_throw_modifier_extractor());
    let skill_modifier_extractor = Arc::new(build_skill_modifier_extractor());
    let stat_modifier_extractor = Arc::new(build_stat_modifier_extractor());
    let attack_modifier_extractor = Arc::new(build_attack_modifier_extractor());
    Dependencies {
        dice_expression_parser,
        dice_roller,
        stat_roller,
        monsters,
        monster_search,
        monster_map,
        saving_throw_modifier_extractor,
        skill_modifier_extractor,
        stat_modifier_extractor,
        attack_modifier_extractor,
    }
}
