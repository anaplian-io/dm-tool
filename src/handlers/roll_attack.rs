use crate::dice::{DiceExpressionParser, DiceRoller, Die, Roll};
use crate::monsters::{DamageType, Monster};
use crate::stats::Critical;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct RollAttackDependencies {
    pub(crate) dice_expression_parser: Arc<dyn DiceExpressionParser + Send + Sync>,
    pub(crate) dice_roller: Arc<dyn DiceRoller + Send + Sync>,
    pub(crate) monster_map: Arc<HashMap<String, Monster>>,
}

#[derive(Serialize)]
pub struct Damage {
    total: i32,
    rolls: Vec<Roll>,
    #[serde(rename = "damageType")]
    damage_type: DamageType,
}

#[derive(Serialize)]
pub struct RollAttackResponse {
    #[serde(rename = "damageRolls")]
    damage_rolls: Vec<Damage>,
    total: i32,
}

pub async fn roll_attack(
    Path((monster_name, attack_index)): Path<(String, usize)>,
    Query(critical): Query<HashMap<Critical, String>>,
    State(dependencies): State<RollAttackDependencies>,
) -> Result<Json<RollAttackResponse>, (StatusCode, String)> {
    let selected_monster = match dependencies.monster_map.get(&monster_name.to_lowercase()) {
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Monster `{}` not found", monster_name),
            ));
        }
        Some(monster) => monster,
    };
    let critical = critical.iter().last().is_some();
    let damage_rolls = selected_monster
        .actions
        .iter()
        .map(|actions| &actions.attack_rolls)
        .flat_map(|attack_rolls| attack_rolls.get(attack_index))
        .flat_map(|attack_roll| attack_roll.damage.clone())
        .map(|damage_roll| {
            (
                dependencies.dice_expression_parser.parse(&damage_roll.roll),
                damage_roll.damage_type,
            )
        })
        .map(|(rolls, damage_type)| (rolls.unwrap(), damage_type))
        .map(|(damage_rolls, damage_type)| {
            let adjusted_damage_rolls = damage_rolls
                .iter()
                .map(|damage_roll| {
                    if !critical {
                        return damage_roll.clone();
                    }
                    let (die, n) = damage_roll;
                    match die {
                        Die::Raw => damage_roll.clone(),
                        die => (die.clone(), n * 2),
                    }
                })
                .collect::<Vec<(Die, i32)>>();
            (adjusted_damage_rolls, damage_type)
        })
        .map(|(required_rolls, damage_type)| {
            (dependencies.dice_roller.roll(&required_rolls), damage_type)
        })
        .map(|((rolls, total), damage_type)| Damage {
            damage_type,
            rolls,
            total,
        })
        .collect::<Vec<_>>();
    let total = damage_rolls
        .iter()
        .map(|roll| roll.total)
        .reduce(|first, second| first + second)
        .unwrap_or(0);
    Ok(Json(RollAttackResponse {
        total,
        damage_rolls,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice::dice_expression_parser::DiceExpressionParserImpl;
    use crate::dice::dice_roller::DiceRollerImpl;
    use crate::dice::die_roller::DieRollerImpl;
    use crate::monsters::{
        Actions, AttackRoll, AttackType, Challenge, DamageRoll, Size, Skills, Speed, Stats,
    };

    #[tokio::test]
    async fn test_monster_not_found() {
        let dependencies = get_dependencies();
        let result = roll_attack(
            Path(("fake-monster".to_string(), 0)),
            Query(HashMap::new()),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_monster_attack_not_found() {
        let dependencies = get_dependencies();
        let result = roll_attack(
            Path(("test monster".to_string(), 1)),
            Query(HashMap::new()),
            State(dependencies.clone()),
        )
        .await
        .unwrap()
        .0;

        assert_eq!(result.damage_rolls.len(), 0);
        assert_eq!(result.total, 0);
    }

    #[tokio::test]
    async fn test_monster_attack() {
        let dependencies = get_dependencies();
        let result = roll_attack(
            Path(("test monster".to_string(), 0)),
            Query(HashMap::new()),
            State(dependencies.clone()),
        )
        .await
        .unwrap()
        .0;

        assert!(result.total > 0);
        assert_eq!(result.damage_rolls.first().unwrap().rolls.len(), 4);
    }

    #[tokio::test]
    async fn test_monster_attack_critical() {
        let dependencies = get_dependencies();
        let mut critical_map: HashMap<Critical, String> = HashMap::new();
        critical_map.insert(Critical::Critical, "".to_string());
        let result = roll_attack(
            Path(("test monster".to_string(), 0)),
            Query(critical_map),
            State(dependencies.clone()),
        )
        .await
        .unwrap()
        .0;

        assert!(result.total > 0);
        assert_eq!(result.damage_rolls.first().unwrap().rolls.len(), 7);
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
            traits: vec![],
            actions: Some(Actions {
                list: vec![],
                attack_rolls: vec![AttackRoll {
                    name: "".to_string(),
                    attack_type: AttackType::MeleeWeapon,
                    reach: 0,
                    hit: 0,
                    damage: vec![DamageRoll {
                        damage_type: DamageType::Bludgeoning,
                        roll: "3d6+8".to_string(),
                    }],
                }],
            }),
            legendary_actions: vec![],
            challenge: Challenge {
                rating: "Medium".to_string(),
                xp: 1000,
            },
            image_url: "https://example.com/monster.jpg".to_string(),
            reactions: vec![],
        }
    }

    fn get_dependencies() -> RollAttackDependencies {
        let mut monster_map = HashMap::new();
        let monster = get_test_monster();
        monster_map.insert("test monster".to_string(), monster.clone());
        let dice_roller = DiceRollerImpl::new(Arc::new(DieRollerImpl::default()));
        let dice_expression_parser = DiceExpressionParserImpl::default();
        RollAttackDependencies {
            dice_expression_parser: Arc::new(dice_expression_parser),
            dice_roller: Arc::new(dice_roller),
            monster_map: Arc::new(monster_map),
        }
    }
}
