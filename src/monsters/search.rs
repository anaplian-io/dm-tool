use crate::monsters::{Monster, Tokenize};
use crate::utilities::index::invert_index;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct MonsterSearch {
    monster_map: Arc<HashMap<String, Monster>>,
    key_index: Arc<HashMap<String, HashSet<String>>>,
}

impl MonsterSearch {
    pub fn from_map(monster_map: Arc<HashMap<String, Monster>>) -> MonsterSearch {
        let tokens_map: HashMap<String, HashSet<String>> = monster_map
            .iter()
            .map(|k| (k.0.clone(), k.1.tokenize()))
            .collect();
        let key_index = Arc::new(invert_index(&tokens_map));
        MonsterSearch {
            monster_map,
            key_index,
        }
    }

    pub fn search(&self, terms: &[&str]) -> Vec<Monster> {
        let resolved_monster_names = terms
            .iter()
            .map(|term| term.to_lowercase())
            .flat_map(|term| self.key_index.get(&term))
            .map(|key_set| key_set.iter().collect::<HashSet<&String>>())
            .reduce(|first, second| first.intersection(&second).cloned().collect())
            .unwrap_or_else(HashSet::new);
        resolved_monster_names
            .iter()
            .flat_map(|name| self.monster_map.get(*name))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::index::vec_to_map;
    use crate::utilities::load_from_json::load_from_json;

    #[test]
    fn loads_from_json() {
        let monster_map = Arc::new(vec_to_map(
            &load_from_json::<Vec<Monster>>("user_data/monsters.json"),
            |monster| monster.name.clone(),
        ));
        let monster_search = MonsterSearch::from_map(monster_map);
        let evil_dragons = monster_search.search(&["eVil", "Dragon", "young"]);

        assert!(evil_dragons.iter().all(|d| d.creature_type == "dragon"));
        assert!(evil_dragons.iter().all(|d| d.alignment.contains("evil")));
    }
}
