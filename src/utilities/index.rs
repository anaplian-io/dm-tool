use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn vec_to_map<T, U, F>(vec: &[T], mut key_extractor: F) -> HashMap<U, T>
where
    F: FnMut(&T) -> U,
    U: Eq + Hash,
    T: Clone,
{
    vec.iter()
        .map(|item| (key_extractor(item), item.clone()))
        .collect()
}

pub fn invert_index<K, V>(token_map: &HashMap<V, HashSet<K>>) -> HashMap<K, HashSet<V>>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    token_map
        .iter()
        .flat_map(|(doc_id, tokens)| {
            tokens
                .iter()
                .map(move |token| (token.clone(), doc_id.clone()))
        })
        .fold(HashMap::new(), |mut acc, (token, document_id)| {
            acc.entry(token)
                .and_modify(|s| {
                    s.insert(document_id.clone());
                })
                .or_insert_with(|| {
                    let mut s = HashSet::new();
                    s.insert(document_id);
                    s
                });
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monsters::{Monster, Tokenize};
    use crate::utilities::load_from_json::load_from_json;

    #[derive(Clone, Debug, PartialEq)]
    struct SmallLittleStruct {
        index: usize,
        data: String,
    }

    #[test]
    fn test_vec_to_arc_map() {
        let vector: Vec<SmallLittleStruct> = vec![
            SmallLittleStruct {
                index: 0,
                data: "data-0".to_string(),
            },
            SmallLittleStruct {
                index: 2,
                data: "data-2".to_string(),
            },
            SmallLittleStruct {
                index: 1,
                data: "data-1".to_string(),
            },
        ];
        let map = vec_to_map(&vector, |item| item.index);
        let mut keys = map.keys().collect::<Vec<_>>();
        keys.sort();

        assert_eq!(keys, vec![&0, &1, &2]);
        assert_eq!(
            map.get(&2),
            Some(&SmallLittleStruct {
                index: 2,
                data: "data-2".to_string(),
            })
        );
    }

    #[test]
    fn test_invert_index() {
        let monsters = load_from_json::<Vec<Monster>>("user_data/monsters.json");
        let monster_map = vec_to_map(&monsters, |monster| monster.name.to_lowercase());
        let monster_tokens: HashMap<String, HashSet<String>> = monster_map
            .iter()
            .map(|k| (k.0.clone(), k.1.tokenize()))
            .collect();
        let inverted_index = invert_index(&monster_tokens);
        let resolved_monsters = inverted_index
            .get("heartstone")
            .iter()
            .flat_map(|names| names.iter().collect::<Vec<&String>>())
            .flat_map(|name| monster_map.get(name))
            .collect::<Vec<_>>();

        assert_eq!(resolved_monsters.first().unwrap().name, "Night Hag");
    }
}
