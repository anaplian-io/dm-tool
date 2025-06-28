use crate::monsters::Monster;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct GetMonsterDependencies {
    pub(crate) monster_map: Arc<HashMap<String, Monster>>,
}

pub async fn get_monster(
    Path(monster_name): Path<String>,
    State(dependencies): State<GetMonsterDependencies>,
) -> Result<Json<Monster>, (StatusCode, String)> {
    match dependencies.monster_map.get(&monster_name.to_lowercase()) {
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Monster `{monster_name}` not found"),
        )),
        Some(monster) => Ok(Json(monster.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monsters::{Challenge, Size, Skills, Speed, Stats};
    use axum::http::StatusCode;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_monster_found() {
        let mut monster_map = HashMap::new();
        let monster = Monster {
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
        };
        monster_map.insert("test_monster".to_string(), monster.clone());
        let dependencies = GetMonsterDependencies {
            monster_map: Arc::new(monster_map),
        };
        let result = get_monster(
            Path("test_monster".to_string()),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_ok());
        let json = result.unwrap();
        assert_eq!(json.name, "Test Monster");
    }

    #[tokio::test]
    async fn test_get_monster_not_found() {
        let dependencies = GetMonsterDependencies {
            monster_map: Arc::new(HashMap::new()),
        };
        let result = get_monster(
            Path("non_existent_monster".to_string()),
            State(dependencies.clone()),
        )
        .await;

        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(message.contains("not found"));
    }
}
