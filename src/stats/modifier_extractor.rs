use crate::monsters::Monster;
use crate::stats::{SkillType, StatType};

pub struct ModifierExtractor<T> {
    extractor: fn(&T, &Monster) -> Option<i32>,
}

impl<T> ModifierExtractor<T> {
    pub fn new(f: fn(&T, &Monster) -> Option<i32>) -> Self {
        Self { extractor: f }
    }

    pub fn extract(&self, value: &T, monster: &Monster) -> Option<i32> {
        (self.extractor)(value, monster)
    }
}

pub fn build_saving_throw_modifier_extractor() -> ModifierExtractor<StatType> {
    ModifierExtractor::new(|stat, monster| match stat {
        StatType::Strength => Some(monster.saving_throws.strength),
        StatType::Dexterity => Some(monster.saving_throws.dexterity),
        StatType::Constitution => Some(monster.saving_throws.constitution),
        StatType::Intelligence => Some(monster.saving_throws.intelligence),
        StatType::Wisdom => Some(monster.saving_throws.wisdom),
        StatType::Charisma => Some(monster.saving_throws.charisma),
    })
}

pub fn build_skill_modifier_extractor() -> ModifierExtractor<SkillType> {
    ModifierExtractor::new(|skill, monster| match skill {
        SkillType::Acrobatics => Some(monster.skills.acrobatics),
        SkillType::AnimalHandling => Some(monster.skills.animal_handling),
        SkillType::Arcana => Some(monster.skills.arcana),
        SkillType::Athletics => Some(monster.skills.athletics),
        SkillType::Deception => Some(monster.skills.deception),
        SkillType::History => Some(monster.skills.history),
        SkillType::Insight => Some(monster.skills.insight),
        SkillType::Intimidation => Some(monster.skills.intimidation),
        SkillType::Investigation => Some(monster.skills.investigation),
        SkillType::Medicine => Some(monster.skills.medicine),
        SkillType::Nature => Some(monster.skills.nature),
        SkillType::Perception => Some(monster.skills.perception),
        SkillType::Performance => Some(monster.skills.performance),
        SkillType::Persuasion => Some(monster.skills.persuasion),
        SkillType::Religion => Some(monster.skills.religion),
        SkillType::SleightOfHand => Some(monster.skills.sleight_of_hand),
        SkillType::Stealth => Some(monster.skills.stealth),
        SkillType::Survival => Some(monster.skills.survival),
    })
}

pub fn build_stat_modifier_extractor() -> ModifierExtractor<StatType> {
    ModifierExtractor::new(|stat, monster| match stat {
        StatType::Strength => Some(monster.modifiers.strength),
        StatType::Dexterity => Some(monster.modifiers.dexterity),
        StatType::Constitution => Some(monster.modifiers.constitution),
        StatType::Intelligence => Some(monster.modifiers.intelligence),
        StatType::Wisdom => Some(monster.modifiers.wisdom),
        StatType::Charisma => Some(monster.modifiers.charisma),
    })
}

pub fn build_attack_modifier_extractor() -> ModifierExtractor<usize> {
    ModifierExtractor::new(|action_index, monster| match &monster.actions {
        None => None,
        Some(actions) => actions
            .attack_rolls
            .get(*action_index)
            .map(|attack_roll| attack_roll.hit),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monsters;
    use crate::monsters::{AttackRoll, AttackType, Challenge, Monster, Size, Skills, Speed, Stats};
    use crate::stats::{SkillType, StatType};

    // Helper function to create a Monster instance
    fn create_monster() -> Monster {
        Monster {
            name: "".to_string(),
            ac: 0,
            size: Size::Gargantuan,
            alignment: "".to_string(),
            languages: vec![],
            creature_type: "".to_string(),
            max_hit_points: 0,
            hit_dice: "".to_string(),
            saving_throws: Stats {
                strength: 1,
                dexterity: 2,
                constitution: 3,
                intelligence: 4,
                wisdom: 5,
                charisma: 6,
            },
            skills: Skills {
                acrobatics: 7,
                animal_handling: 8,
                arcana: 9,
                athletics: 10,
                deception: 11,
                history: 12,
                insight: 13,
                intimidation: 14,
                investigation: 0,
                medicine: 15,
                nature: 16,
                perception: 17,
                performance: 18,
                persuasion: 19,
                religion: 20,
                sleight_of_hand: 21,
                stealth: 22,
                survival: 23,
            },
            modifiers: Stats {
                strength: 24,
                dexterity: 25,
                constitution: 26,
                intelligence: 27,
                wisdom: 28,
                charisma: 29,
            },
            actions: Some(monsters::Actions {
                list: vec![],
                attack_rolls: vec![AttackRoll {
                    name: "".to_string(),
                    attack_type: AttackType::MeleeWeapon,
                    reach: 0,
                    hit: 30,

                    damage: vec![],
                }],
            }),
            legendary_actions: vec![],
            reactions: vec![],
            challenge: Challenge {
                rating: "".to_string(),
                xp: 0,
            },
            speed: Speed {
                walk: 0,
                fly: 0,
                swim: 0,
                burrow: 0,
                climb: 0,
                hover: false,
            },
            stats: Stats {
                strength: 0,
                dexterity: 0,
                constitution: 0,
                intelligence: 0,
                wisdom: 0,
                charisma: 0,
            },
            traits: vec![],
            image_url: "".to_string(),
        }
    }

    #[test]
    fn test_saving_throw_extractor() {
        let monster = create_monster();
        let extractor = build_saving_throw_modifier_extractor();

        assert_eq!(extractor.extract(&StatType::Strength, &monster), Some(1));
        assert_eq!(extractor.extract(&StatType::Dexterity, &monster), Some(2));
        assert_eq!(
            extractor.extract(&StatType::Constitution, &monster),
            Some(3)
        );
        assert_eq!(
            extractor.extract(&StatType::Intelligence, &monster),
            Some(4)
        );
        assert_eq!(extractor.extract(&StatType::Wisdom, &monster), Some(5));
        assert_eq!(extractor.extract(&StatType::Charisma, &monster), Some(6));
    }

    #[test]
    fn test_skill_extractor() {
        let monster = create_monster();
        let extractor = build_skill_modifier_extractor();

        assert_eq!(extractor.extract(&SkillType::Acrobatics, &monster), Some(7));
        assert_eq!(
            extractor.extract(&SkillType::AnimalHandling, &monster),
            Some(8)
        );
        assert_eq!(extractor.extract(&SkillType::Arcana, &monster), Some(9));
        assert_eq!(extractor.extract(&SkillType::Athletics, &monster), Some(10));
        assert_eq!(extractor.extract(&SkillType::Deception, &monster), Some(11));
        assert_eq!(extractor.extract(&SkillType::History, &monster), Some(12));
        assert_eq!(extractor.extract(&SkillType::Insight, &monster), Some(13));
        assert_eq!(
            extractor.extract(&SkillType::Intimidation, &monster),
            Some(14)
        );
        assert_eq!(extractor.extract(&SkillType::Medicine, &monster), Some(15));
        assert_eq!(extractor.extract(&SkillType::Nature, &monster), Some(16));
        assert_eq!(
            extractor.extract(&SkillType::Perception, &monster),
            Some(17)
        );
        assert_eq!(
            extractor.extract(&SkillType::Performance, &monster),
            Some(18)
        );
        assert_eq!(
            extractor.extract(&SkillType::Persuasion, &monster),
            Some(19)
        );
        assert_eq!(extractor.extract(&SkillType::Religion, &monster), Some(20));
        assert_eq!(
            extractor.extract(&SkillType::SleightOfHand, &monster),
            Some(21)
        );
        assert_eq!(extractor.extract(&SkillType::Stealth, &monster), Some(22));
        assert_eq!(extractor.extract(&SkillType::Survival, &monster), Some(23));
        assert_eq!(
            extractor.extract(&SkillType::Investigation, &monster),
            Some(0)
        );
    }

    #[test]
    fn test_stat_modifier_extractor() {
        let monster = create_monster();
        let extractor = build_stat_modifier_extractor();

        assert_eq!(extractor.extract(&StatType::Strength, &monster), Some(24));
        assert_eq!(extractor.extract(&StatType::Dexterity, &monster), Some(25));
        assert_eq!(
            extractor.extract(&StatType::Constitution, &monster),
            Some(26)
        );
        assert_eq!(
            extractor.extract(&StatType::Intelligence, &monster),
            Some(27)
        );
        assert_eq!(extractor.extract(&StatType::Wisdom, &monster), Some(28));
        assert_eq!(extractor.extract(&StatType::Charisma, &monster), Some(29));
    }

    #[test]
    fn test_attack_modifier_extractor() {
        let monster = create_monster();
        let extractor = build_attack_modifier_extractor();

        assert_eq!(extractor.extract(&0, &monster), Some(30));
    }
}
