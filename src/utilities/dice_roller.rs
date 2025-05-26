use crate::utilities::Die;
use crate::utilities::{DiceRoller, DieRoller, Roll};
use std::sync::Arc;

pub struct DiceRollerImpl {
    die_roller: Arc<dyn DieRoller + Send + Sync>,
}

impl DiceRollerImpl {
    pub fn new(die_roller: Arc<dyn DieRoller + Send + Sync>) -> Self {
        Self { die_roller }
    }
}

impl DiceRoller for DiceRollerImpl {
    fn roll(&self, dice: &[(Die, i32)]) -> (Vec<Roll>, i32) {
        let rolls = dice
            .iter()
            .flat_map(|(die, n)| match die {
                Die::Raw => vec![Roll {
                    die: Die::Raw,
                    value: *n,
                }],
                die => (0..*n)
                    .map(|_| Roll {
                        die: die.clone(),
                        value: self.die_roller.roll(die),
                    })
                    .collect::<Vec<_>>(),
            })
            .collect::<Vec<Roll>>();
        let total = rolls
            .iter()
            .map(|roll| roll.value)
            .reduce(|a, b| a + b)
            .unwrap_or(0);
        (rolls, total)
    }
}
