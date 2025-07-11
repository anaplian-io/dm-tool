use crate::handlers::{MonsterRollerDependencies, StatRollResponse};
use crate::stats::AdvantageType;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use std::collections::HashMap;

pub async fn roll_stat<T>(
    Path((monster_name, stat)): Path<(String, T)>,
    Query(advantage): Query<HashMap<AdvantageType, String>>,
    State(dependencies): State<MonsterRollerDependencies<T>>,
) -> Result<Json<StatRollResponse>, (StatusCode, String)> {
    let selected_monster = match dependencies.monster_map.get(&monster_name.to_lowercase()) {
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Monster `{monster_name}` not found"),
            ));
        }
        Some(monster) => monster,
    };
    let modifier = match dependencies
        .modifier_extractor
        .extract(&stat, selected_monster)
    {
        None => {
            return Err((StatusCode::NOT_FOUND, "No stat modifier found".to_string()));
        }
        Some(modifier) => modifier,
    };
    let rolls = dependencies
        .stats_roller
        .roll_stat(modifier, &advantage.keys().last());
    Ok(Json(StatRollResponse {
        first_roll: rolls.first_roll,
        second_roll: rolls.second_roll,
        result: rolls.result,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice::Roll;
    use crate::dice::dice_roller::DiceRollerImpl;
    use crate::dice::die_roller::DieRollerImpl;
    use crate::handlers::MonsterRollerDependencies;
    use crate::handlers::roll_stat::roll_stat;
    use crate::monsters::{Challenge, Monster, Size, Skills, Speed, Stats};
    use crate::stats::StatType;
    use crate::stats::modifier_extractor::{
        ModifierExtractor, build_saving_throw_modifier_extractor,
    };
    use crate::stats::stat_roller::StatRollerImpl;
    use axum::http::StatusCode;
    use std::cmp::{max, min};
    use std::collections::HashMap;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_roll_saving_throw_found() {
        let mut monster_map = HashMap::new();
        let monster = get_test_monster();
        monster_map.insert("test_monster".to_string(), monster.clone());

        let dependencies = MonsterRollerDependencies {
            monster_map: Arc::new(monster_map),
            stats_roller: Arc::new(StatRollerImpl::new(Arc::new(DiceRollerImpl::new(
                Arc::new(DieRollerImpl::default()),
            )))),
            modifier_extractor: get_modifier_extractor(),
        };

        let result = roll_stat(
            Path(("test_monster".to_string(), StatType::Wisdom)),
            Query(HashMap::new()),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_ok());
        let result = result.unwrap().0;
        assert!(result.second_roll.is_none());
    }

    #[tokio::test]
    async fn test_roll_saving_throw_monster_not_found() {
        let dependencies = MonsterRollerDependencies {
            monster_map: Arc::new(HashMap::new()),
            stats_roller: Arc::new(StatRollerImpl::new(Arc::new(DiceRollerImpl::new(
                Arc::new(DieRollerImpl::default()),
            )))),
            modifier_extractor: get_modifier_extractor(),
        };

        let result = roll_stat(
            Path(("test_monster".to_string(), StatType::Wisdom)),
            Query(HashMap::new()),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().0, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_roll_saving_throw_modifier_not_found() {
        let mut monster_map = HashMap::new();
        let monster = get_test_monster();
        monster_map.insert("test_monster".to_string(), monster.clone());

        let dependencies = MonsterRollerDependencies {
            monster_map: Arc::new(monster_map),
            stats_roller: Arc::new(StatRollerImpl::new(Arc::new(DiceRollerImpl::new(
                Arc::new(DieRollerImpl::default()),
            )))),
            modifier_extractor: Arc::new(ModifierExtractor::new(|_, _| None)),
        };

        let result = roll_stat(
            Path(("test_monster".to_string(), StatType::Wisdom)),
            Query(HashMap::new()),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().0, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_roll_saving_throw_with_advantage() {
        let mut monster_map = HashMap::new();
        let monster = get_test_monster();
        monster_map.insert("test_monster".to_string(), monster.clone());

        let dependencies = MonsterRollerDependencies {
            monster_map: Arc::new(monster_map),
            stats_roller: Arc::new(StatRollerImpl::new(Arc::new(DiceRollerImpl::new(
                Arc::new(DieRollerImpl::default()),
            )))),
            modifier_extractor: get_modifier_extractor(),
        };

        let result = roll_stat(
            Path(("test_monster".to_string(), StatType::Wisdom)),
            Query(
                [(AdvantageType::Advantage, "".to_string())]
                    .into_iter()
                    .collect(),
            ),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_ok());
        let unwrapped_result = result.unwrap().0;
        assert!(unwrapped_result.second_roll.is_some());
        assert_eq!(
            unwrapped_result.result,
            max(
                total_roll(&unwrapped_result.first_roll),
                total_roll(&unwrapped_result.second_roll.unwrap())
            )
        );
    }

    #[tokio::test]
    async fn test_roll_saving_throw_with_disadvantage() {
        let mut monster_map = HashMap::new();
        let monster = get_test_monster();
        monster_map.insert("test_monster".to_string(), monster.clone());

        let dependencies = MonsterRollerDependencies {
            monster_map: Arc::new(monster_map),
            stats_roller: Arc::new(StatRollerImpl::new(Arc::new(DiceRollerImpl::new(
                Arc::new(DieRollerImpl::default()),
            )))),
            modifier_extractor: get_modifier_extractor(),
        };

        let result = roll_stat(
            Path(("test_monster".to_string(), StatType::Wisdom)),
            Query(
                [(AdvantageType::Disadvantage, "".to_string())]
                    .into_iter()
                    .collect(),
            ),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_ok());
        let result = result.unwrap().0;
        assert!(result.second_roll.is_some());
        assert_eq!(
            result.result,
            min(
                total_roll(&result.first_roll),
                total_roll(&result.second_roll.unwrap())
            )
        );
    }

    fn get_test_monster() -> Monster {
        Monster {
            name: "Test Monster".to_string(),
            ac: 10,
            size: Size::Medium,
            alignment: "Neutral".to_string(),
            languages: vec!["Common".to_string()],
            creature_type: "".to_string(),
            max_hit_points: 100,
            hit_dice: "10d10".to_string(),
            speed: Speed {
                walk: 30,
                fly: 0,
                swim: 0,
                burrow: 0,
                climb: 0,
                hover: false,
            },
            modifiers: Stats {
                strength: 10,
                dexterity: 12,
                constitution: 14,
                intelligence: 10,
                wisdom: 13,
                charisma: 8,
            },
            stats: Stats {
                strength: 10,
                dexterity: 12,
                constitution: 14,
                intelligence: 10,
                wisdom: 13,
                charisma: 8,
            },
            saving_throws: Stats {
                strength: 10,
                dexterity: 12,
                constitution: 14,
                intelligence: 10,
                wisdom: 13,
                charisma: 8,
            },
            skills: Skills {
                acrobatics: 5,
                animal_handling: 5,
                arcana: 3,
                athletics: 5,
                deception: 3,
                history: 2,
                insight: 4,
                intimidation: 4,
                investigation: 3,
                medicine: 4,
                nature: 2,
                perception: 4,
                performance: 3,
                persuasion: 3,
                religion: 2,
                sleight_of_hand: 5,
                stealth: 0,
                survival: 4,
            },
            traits: vec!["Darkvision".to_string(), "Keen Senses".to_string()],
            actions: None,
            legendary_actions: vec![],
            challenge: Challenge {
                rating: "Medium".to_string(),
                xp: 1000,
            },
            image_url: "https://example.com/monster.jpg".to_string(),
            reactions: vec![],
        }
    }

    fn total_roll(rolls: &[Roll]) -> i32 {
        rolls
            .iter()
            .map(|roll| roll.value)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn get_modifier_extractor() -> Arc<ModifierExtractor<StatType>> {
        Arc::new(build_saving_throw_modifier_extractor())
    }
}
