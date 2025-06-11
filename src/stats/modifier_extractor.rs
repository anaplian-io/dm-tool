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
