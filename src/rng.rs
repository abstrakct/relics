use bracket_random::prelude::RandomNumberGenerator;
use std::sync::Mutex;

lazy_static! {
    static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

pub fn reseed(seed: u64) {
    *RNG.lock().unwrap() = RandomNumberGenerator::seeded(seed);
}

pub fn roll_dice(n: i32, t: i32) -> i32 {
    RNG.lock().unwrap().roll_dice(n, t)
}

// TODO: improve this
pub fn roll_str<T: ToString>(dice: T) -> i32 {
    let result = RNG.lock().unwrap().roll_str(dice);
    if let Ok(result) = result {
        result
    } else {
        println!("{:?}", result);
        0
    }
}

pub fn range(min: i32, max: i32) -> i32 {
    RNG.lock().unwrap().range(min, max)
}

#[allow(dead_code)]
pub fn next_u64() -> u64 {
    RNG.lock().unwrap().next_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_rng_returns_expected_values() {
        reseed(123456789);
        assert_eq!(next_u64(), 4730442134150660564);
        assert_eq!(next_u64(), 12666920420498467706);
    }

    #[test]
    fn range_is_within_range() {
        for _ in 1..=100 {
            let result = range(1, 10);
            assert!(result >= 1);
            assert!(result <= 10);
        }
    }

    #[test]
    fn roll_str_1d10() {
        for _ in 1..=100 {
            let result = roll_str("1d10");
            assert!(result >= 1);
            assert!(result <= 10);
        }
    }

    #[test]
    fn roll_str_2d20plus2() {
        for _ in 1..=100 {
            let result = roll_str("2d20+2");
            assert!(result >= 4);
            assert!(result <= 42);
        }
    }

    #[test]
    fn roll_str_3d6minus5() {
        for _ in 1..=100 {
            let result = roll_str("3d6-5");
            assert!(result >= -2);
            assert!(result <= 13);
        }
    }
}
