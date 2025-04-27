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
