use crate::dice::{DiceRoller, Die};
use crate::stats::{AdvantageType, StatRoll, StatRoller};
use std::cmp::{max, min};
use std::sync::Arc;

pub struct StatRollerImpl {
    dice_roller: Arc<dyn DiceRoller + Sync + Send>,
}

impl StatRollerImpl {
    pub fn new(dice_roller: Arc<dyn DiceRoller + Sync + Send>) -> Self {
        Self { dice_roller }
    }
}

impl StatRoller for StatRollerImpl {
    fn roll_stat(&self, modifier: i32, advantage_status: &Option<&AdvantageType>) -> StatRoll {
        let die_roll_expression = [(Die::D20, 1), (Die::Raw, modifier)];
        let (first_roll, first_roll_total) = self.dice_roller.roll(&die_roll_expression);
        let (second_roll, result) = match advantage_status {
            None => (None, first_roll_total),
            Some(advantage_type) => {
                let (second_roll, second_roll_total) = self.dice_roller.roll(&die_roll_expression);
                match advantage_type {
                    AdvantageType::Advantage => {
                        (Some(second_roll), max(first_roll_total, second_roll_total))
                    }
                    AdvantageType::Disadvantage => {
                        (Some(second_roll), min(first_roll_total, second_roll_total))
                    }
                }
            }
        };
        StatRoll {
            first_roll,
            second_roll,
            result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice::Roll;
    use crate::dice::dice_roller::DiceRollerImpl;
    use crate::dice::die_roller::DieRollerImpl;
    use std::sync::Arc;

    #[test]
    fn test_roll_stat_no_advantage() {
        let dice_roller = Arc::new(DiceRollerImpl::new(Arc::new(DieRollerImpl::default())));
        let stat_roller = StatRollerImpl::new(dice_roller);
        let result = stat_roller.roll_stat(0, &None);
        assert!(result.result <= 20 && result.result >= 0);
        assert_eq!(result.first_roll.first().unwrap().value, result.result);
    }

    #[test]
    fn test_roll_stat_with_advantage() {
        let dice_roller = Arc::new(DiceRollerImpl::new(Arc::new(DieRollerImpl::default())));
        let stat_roller = StatRollerImpl::new(dice_roller);
        let result = stat_roller.roll_stat(3, &Some(&AdvantageType::Advantage));
        assert_eq!(
            result.result,
            max(
                total_roll(&result.first_roll),
                total_roll(&result.second_roll.unwrap())
            )
        );
    }

    #[test]
    fn test_roll_stat_with_disadvantage() {
        let dice_roller = Arc::new(DiceRollerImpl::new(Arc::new(DieRollerImpl::default())));
        let stat_roller = StatRollerImpl::new(dice_roller);
        let result = stat_roller.roll_stat(3, &Some(&AdvantageType::Disadvantage));
        assert_eq!(
            result.result,
            min(
                total_roll(&result.first_roll),
                total_roll(&result.second_roll.unwrap())
            )
        );
    }

    fn total_roll(rolls: &[Roll]) -> i32 {
        rolls
            .iter()
            .map(|roll| roll.value)
            .reduce(|a, b| a + b)
            .unwrap()
    }
}
