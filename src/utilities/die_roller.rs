use crate::state::Die;
use crate::utilities::DieRoller;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::thread::LocalKey;

pub struct DieRollerImpl {
    random: &'static LocalKey<RefCell<ThreadRng>>,
}

thread_local! {
    static RNG: RefCell<ThreadRng> = RefCell::new(rand::rng());
}

impl DieRollerImpl {
    fn new(random: &'static LocalKey<RefCell<ThreadRng>>) -> Self {
        Self { random }
    }

    pub fn default() -> Self {
        Self::new(&RNG)
    }
}

impl DieRoller for DieRollerImpl {
    fn roll(self: &Self, die: &Die) -> u32 {
        let get_local_inclusive_random = |n: u32| {
            self.random
                .with(|cell| cell.borrow_mut().random_range(1..=n))
        };
        match die {
            Die::D4 => get_local_inclusive_random(4),
            Die::D6 => get_local_inclusive_random(6),
            Die::D8 => get_local_inclusive_random(8),
            Die::D10 => get_local_inclusive_random(10),
            Die::D12 => get_local_inclusive_random(12),
            Die::D20 => get_local_inclusive_random(20),
            Die::RAW => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_d4() {
        let roller = DieRollerImpl::default();

        assert!(matches!(roller.roll(&Die::D4), 1..=4));
    }

    #[test]
    fn test_roll_d6() {
        let roller = DieRollerImpl::default();

        assert!(matches!(roller.roll(&Die::D6), 1..=6));
    }

    #[test]
    fn test_roll_d8() {
        let roller = DieRollerImpl::default();

        assert!(matches!(roller.roll(&Die::D8), 1..=8));
    }

    #[test]
    fn test_roll_d10() {
        let roller = DieRollerImpl::default();

        assert!(matches!(roller.roll(&Die::D10), 1..=10));
    }

    #[test]
    fn test_roll_d12() {
        let roller = DieRollerImpl::default();

        assert!(matches!(roller.roll(&Die::D12), 1..=12));
    }

    #[test]
    fn test_roll_d20() {
        let roller = DieRollerImpl::default();

        assert!(matches!(roller.roll(&Die::D20), 1..=20));
    }

    #[test]
    fn test_roll_raw() {
        let roller = DieRollerImpl::default();

        assert_eq!(roller.roll(&Die::RAW), 0)
    }
}
