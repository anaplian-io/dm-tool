use crate::monsters::{Actions, AttackRoll, DamageRoll, Monster, Tokenize};
use std::collections::HashSet;
use std::iter::once;

impl Tokenize for Monster {
    fn tokenize(&self) -> HashSet<String> {
        once(self.name.clone())
            .chain(once(self.size.to_string()))
            .chain(once(self.creature_type.to_string()))
            .chain(once(self.alignment.clone()))
            .chain(self.languages.clone())
            .chain(self.traits.clone())
            .chain(self.actions.iter().flat_map(|action| action.tokenize()))
            .flat_map(|token| token.split(' ').map(|s| s.to_string()).collect::<Vec<_>>())
            .map(|token| {
                token
                    .chars()
                    .filter(|ch| ch.is_alphanumeric())
                    .collect::<String>()
            })
            .map(|token| token.to_lowercase())
            .filter(|token| !token.is_empty())
            .collect()
    }
}

impl Tokenize for Actions {
    fn tokenize(&self) -> HashSet<String> {
        self.list
            .iter()
            .chain(self.attack_rolls.tokenize().iter())
            .map(|s| s.to_string())
            .collect()
    }
}

impl Tokenize for Vec<AttackRoll> {
    fn tokenize(&self) -> HashSet<String> {
        self.iter().flat_map(|roll| roll.tokenize()).collect()
    }
}

impl Tokenize for AttackRoll {
    fn tokenize(&self) -> HashSet<String> {
        once(self.name.clone())
            .chain(once(self.attack_type.to_string()))
            .chain(self.damage.tokenize())
            .collect()
    }
}

impl Tokenize for Vec<DamageRoll> {
    fn tokenize(&self) -> HashSet<String> {
        self.iter()
            .map(|damage_roll| damage_roll.damage_type.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::MONSTERS_JSON_PATH;
    use crate::utilities::load_from_json::load_from_json;

    #[test]
    fn tokenize_monster() {
        let monsters = load_from_json::<Vec<Monster>>(MONSTERS_JSON_PATH);
        let monster: &Monster = monsters.first().unwrap();
        let monster_tokens = monster.tokenize();

        let mut monster_token_chars: Box<dyn Iterator<Item = char>> =
            Box::new(monster_tokens.iter().flat_map(|s| s.chars()));
        assert!(monster_token_chars.all(|ch| ch.is_alphanumeric()));
        assert!(monster_token_chars.all(|ch| ch.is_lowercase()));
    }
}
