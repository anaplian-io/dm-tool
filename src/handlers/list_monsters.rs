use crate::monsters::Monster;
use crate::monsters::search::MonsterSearch;
use axum::Json;
use axum::extract::{Query, State};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct ListMonstersDependencies {
    pub(crate) monsters: Arc<Vec<Monster>>,
    pub(crate) monster_search: Arc<MonsterSearch>,
}

pub async fn list_monsters(
    Query(params): Query<HashMap<String, String>>,
    State(dependencies): State<ListMonstersDependencies>,
) -> Json<Vec<Monster>> {
    params
        .get("query")
        .map(|query| query.split(' ').collect::<Vec<&str>>())
        .map(|search_terms| dependencies.monster_search.search(&search_terms))
        .map(Json)
        .unwrap_or_else(|| Json(dependencies.monsters.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::MONSTERS_JSON_PATH;
    use crate::utilities::index::vec_to_map;
    use crate::utilities::load_from_json::load_from_json;

    fn get_dependencies() -> ListMonstersDependencies {
        let monsters = load_from_json::<Vec<Monster>>(MONSTERS_JSON_PATH);
        let monster_map = Arc::new(vec_to_map(&monsters, |monster| monster.name.clone()));
        let monster_search = MonsterSearch::from_map(monster_map);
        ListMonstersDependencies {
            monsters: Arc::new(monsters),
            monster_search: Arc::new(monster_search),
        }
    }

    #[tokio::test]
    async fn test_list_monsters_all() {
        let state = get_dependencies();
        let result = list_monsters(Query(HashMap::new()), State(state)).await.0;

        assert_eq!(result.first().unwrap().name, "Aboleth");
        assert!(result.len() > 1);
    }

    #[tokio::test]
    async fn test_list_monsters_query() {
        let state = get_dependencies();
        let result = list_monsters(
            Query(HashMap::from([(
                "query".to_string(),
                "young evil dragon".to_string(),
            )])),
            State(state),
        )
        .await
        .0;

        assert!(result.iter().all(|d| d.creature_type == "dragon"));
        assert!(result.iter().all(|d| d.alignment.contains("evil")));
    }
}
