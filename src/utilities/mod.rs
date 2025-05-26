pub mod die_roller;

use crate::state::Die;

pub trait DieRoller {
    fn roll(&self, die: &Die) -> i32;
}
