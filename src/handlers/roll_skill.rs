use crate::handlers::{MonsterRollerDependencies, StatRollResponse};
use crate::stats::{AdvantageType, SkillType};
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use std::collections::HashMap;

pub async fn roll_skill(
    Path((monster_name, skill)): Path<(String, SkillType)>,
    Query(advantage): Query<HashMap<AdvantageType, String>>,
    State(dependencies): State<MonsterRollerDependencies>,
) -> Result<Json<StatRollResponse>, (StatusCode, String)> {
    let monster_skills = match dependencies.monster_map.get(&monster_name.to_lowercase()) {
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Monster `{}` not found", monster_name),
            ));
        }
        Some(monster) => &monster.skills,
    };
    let modifier = match skill {
        SkillType::Acrobatics => monster_skills.acrobatics,
        SkillType::AnimalHandling => monster_skills.animal_handling,
        SkillType::Arcana => monster_skills.arcana,
        SkillType::Athletics => monster_skills.athletics,
        SkillType::Deception => monster_skills.deception,
        SkillType::History => monster_skills.history,
        SkillType::Insight => monster_skills.insight,
        SkillType::Intimidation => monster_skills.intimidation,
        SkillType::Medicine => monster_skills.medicine,
        SkillType::Nature => monster_skills.nature,
        SkillType::Perception => monster_skills.perception,
        SkillType::Performance => monster_skills.performance,
        SkillType::Persuasion => monster_skills.persuasion,
        SkillType::Religion => monster_skills.religion,
        SkillType::SleightOfHand => monster_skills.sleight_of_hand,
        SkillType::Stealth => monster_skills.stealth,
        SkillType::Survival => monster_skills.survival,
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
    use crate::dice::dice_roller::DiceRollerImpl;
    use crate::dice::die_roller::DieRollerImpl;
    use crate::monsters::{Challenge, Monster, Size, Skills, Speed, Stats};
    use crate::stats::AdvantageType::{Advantage, Disadvantage};
    use crate::stats::StatRoller;
    use crate::stats::stat_roller::StatRollerImpl;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_roll_skill_not_found() {
        let dependencies = MonsterRollerDependencies {
            stats_roller: get_stat_roller(),
            monster_map: Arc::new(HashMap::new()),
        };

        let result = roll_skill(
            Path(("test_monster".to_string(), SkillType::Nature)),
            Query(HashMap::new()),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().0, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_roll_skill_no_advantage() {
        let dependencies = MonsterRollerDependencies {
            stats_roller: get_stat_roller(),
            monster_map: Arc::new(HashMap::from([(
                "test_monster".to_string(),
                get_test_monster(),
            )])),
        };

        let result = roll_skill(
            Path(("test_monster".to_string(), SkillType::Nature)),
            Query(HashMap::new()),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_ok());
        assert!(result.unwrap().second_roll.is_none());
    }

    #[tokio::test]
    async fn test_roll_skill_advantage() {
        let dependencies = MonsterRollerDependencies {
            stats_roller: get_stat_roller(),
            monster_map: Arc::new(HashMap::from([(
                "test_monster".to_string(),
                get_test_monster(),
            )])),
        };

        let result = roll_skill(
            Path(("test_monster".to_string(), SkillType::Nature)),
            Query(HashMap::from([(Advantage, "".to_string())])),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_ok());
        assert!(result.unwrap().second_roll.is_some());
    }

    #[tokio::test]
    async fn test_roll_skill_disadvantage() {
        let dependencies = MonsterRollerDependencies {
            stats_roller: get_stat_roller(),
            monster_map: Arc::new(HashMap::from([(
                "test_monster".to_string(),
                get_test_monster(),
            )])),
        };

        let result = roll_skill(
            Path(("test_monster".to_string(), SkillType::Nature)),
            Query(HashMap::from([(Disadvantage, "".to_string())])),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_ok());
        assert!(result.unwrap().second_roll.is_some());
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
    fn get_stat_roller() -> Arc<dyn StatRoller + Sync + Send> {
        Arc::new(StatRollerImpl::new(Arc::new(DiceRollerImpl::new(
            Arc::new(DieRollerImpl::default()),
        ))))
    }
}
