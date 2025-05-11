// use bevy::log::debug;
use bevy_ecs::prelude::*;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::{
    gamelogic::{RollResult, RollResultType, Rollable},
    rng,
};

//----------------//
// Helper structs //
//----------------//

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    pub current: i32,
    pub max: i32,
}

#[allow(dead_code)]
impl Pool {
    /// Creates a new `Pool` with the specified maximum capacity.
    ///
    /// # Arguments
    ///
    /// * `max` - The maximum capacity of the pool, which is also set as the initial current value.
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    /// Creates a new `Pool` with the specified initial current value and maximum capacity.
    ///
    /// # Arguments
    ///
    /// * `current` - The initial current value of the pool.
    /// * `max` - The maximum capacity of the pool.
    ///
    pub fn init(current: i32, max: i32) -> Self {
        Self { current, max }
    }

    /// Decreases the current value of the pool by the specified amount.
    /// Current value can't go below 0.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to decrease the pool by.
    ///
    pub fn decrease(&mut self, amount: i32) {
        self.current -= amount;
        if self.current < 0 {
            self.current = 0;
        }
    }

    /// Increases the current value of the pool by the specified amount.
    /// Current value cannot exceed the maximum capacity.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to increase the pool by.
    pub fn increase(&mut self, amount: i32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    /// Sets the current value of the pool to 0.
    pub fn empty(&mut self) {
        self.current = 0;
    }

    /// Checks if the pool is empty.
    ///
    /// # Returns
    ///
    /// * `true` if the current value of the pool is 0, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    /// Checks if the pool is full.
    ///
    /// # Returns
    ///
    /// * `true` if the current value of the pool is equal to the maximum capacity, otherwise `false`.
    pub fn is_full(&self) -> bool {
        self.current == self.max
    }

    /// Returns the percentage of the pool that is filled.
    ///
    /// # Returns
    ///
    /// * A value between 0.0 and 1.0 representing the percentage of the pool that is filled.
    pub fn current_percent(&self) -> f32 {
        self.current as f32 / self.max as f32
    }

    /// Sets the current value of the pool to the specified amount.
    ///
    /// If the specified amount is less than 0, the current value is set to 0.
    /// If the specified amount is greater than the maximum capacity, the current value is set to the maximum capacity.
    pub fn set(&mut self, amount: i32) {
        self.current = amount;
        if self.current < 0 {
            self.current = 0;
        }
        if self.current > self.max {
            self.current = self.max;
        }
    }

    /// Sets the current value of the pool to the maximum capacity.
    pub fn set_max(&mut self) {
        self.current = self.max;
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub base: i32,
    pub modifiers: i32,
    pub bonus: i32,
}

impl Rollable for Attribute {
    fn check(&self, roll: i32) -> RollResult {
        let great_success = (self.base as f32 * 0.4).max(3.0) as i32;
        let extreme_success = (self.base as f32 * 0.1).max(2.0) as i32;

        let mut great_failure = ((100 - self.base) as f32 * 0.4).max(2.0) as i32;
        let mut extreme_failure = ((100 - self.base) as f32 * 0.1).max(1.0) as i32;

        if great_failure < 2 {
            great_failure = 2;
        }
        if extreme_failure < 1 {
            extreme_failure = 1;
        }

        println!("base is {}, roll is {}", self.base, roll);
        println!("threshold for great success is {}", great_success);
        println!("threshold for extreme success is {}", extreme_success);
        println!("threshold for great failure is {}", 100 - great_failure);
        println!("threshold for extreme failure is {}", 100 - extreme_failure);

        if roll <= self.base {
            if roll == 1 {
                RollResult::Success(RollResultType::Critical)
            } else if roll <= extreme_success {
                RollResult::Success(RollResultType::Extreme)
            } else if roll <= great_success {
                RollResult::Success(RollResultType::Great)
            } else {
                RollResult::Success(RollResultType::Normal)
            }
        } else if roll > self.base {
            if roll == 100 {
                RollResult::Failure(RollResultType::Critical)
            } else if (100 - roll) < extreme_failure {
                RollResult::Failure(RollResultType::Extreme)
            } else if (100 - roll) < great_failure {
                RollResult::Failure(RollResultType::Great)
            } else {
                RollResult::Failure(RollResultType::Normal)
            }
        } else {
            RollResult::Failure(RollResultType::Normal)
        }
    }

    fn roll(&self) -> RollResult {
        let roll = rng::roll_str("1d100");
        self.check(roll)
    }
}

//-------------------//
// Marker components //
//-------------------//

#[derive(Component, Default, Serialize, Deserialize, Debug)]
#[require(Position, Render, Name, DetailedName, Stats, Attributes)]
pub struct Player;

/// Indicates that an entity is a sentient being.
#[derive(Component, Default, Serialize, Deserialize, Debug)]
pub struct Sentient;

/// Indicates that an entity has a physical body.
#[derive(Component, Default, Serialize, Deserialize, Debug)]
pub struct Corporeal;

/// Indicates that an entity has a mind/consciousness.
#[derive(Component, Default, Serialize, Deserialize, Debug)]
pub struct Mental;

/// Indicates that an entity has a soul/spirit.
#[derive(Component, Default, Serialize, Deserialize, Debug)]
pub struct Spiritual;

//----------------------//
// Components with data //
//----------------------//

#[derive(Component, Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map: i32,
}

#[derive(Component, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Render {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
    pub order: i32,
    pub always: bool,
}

#[derive(Component, Default, Serialize, Deserialize, Debug)]
pub struct DetailedName {
    pub base: String,
    pub full: String,
}

/// Stats component. Holds all stats an entity can have.
/// These stats change more or less frequently.
#[derive(Component, Default, Serialize, Deserialize)]
pub struct Stats {
    /// Hitpoints
    pub hp: Pool,
    /// Magic points
    pub mp: Pool,
    /// Current experience points
    pub xp: i32,
    /// Current defense value
    pub dv: i32,
    /// Current protection value
    pub pv: i32,
    /// Current level (experience level)
    pub lv: i32,
}

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Attributes {
    pub str: Attribute,
    pub dex: Attribute,
    pub con: Attribute,
    pub int: Attribute,
}

/// This component describes an entity's *intent* to perform some action.
/// The entity may or may not be able to do it, or succeed if able.
/// The intent system must find out those things, and produce appropriate events.
#[derive(Component, Debug, Serialize, Deserialize)]
pub enum Intent {
    MoveRelative { dx: i32, dy: i32 },
    MoveAbsolute { x: i32, y: i32 },
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Speed {
    pub speed: f32,
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Energy {
    pub energy: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_pool_is_correct() {
        let pool = Pool::new(100);
        assert_eq!(pool.current, 100);
        assert_eq!(pool.max, 100);
        assert!(pool.is_full());
        assert!(!pool.is_empty());
    }

    #[test]
    fn pool_decrease() {
        let mut pool = Pool::new(100);
        pool.decrease(10);
        assert_eq!(pool.current, 90);
        assert_eq!(pool.max, 100);
    }

    #[test]
    fn pool_increase() {
        let mut pool = Pool::new(100);
        pool.decrease(10);
        pool.increase(10);
        assert_eq!(pool.current, 100);
        assert_eq!(pool.max, 100);
    }

    #[test]
    fn pool_increase_doesnt_go_above_max() {
        let mut pool = Pool::new(100);
        pool.decrease(10);
        pool.increase(20);
        assert_eq!(pool.current, 100);
        assert_eq!(pool.max, 100);
    }

    #[test]
    fn pool_decrease_doesnt_go_below_zero() {
        let mut pool = Pool::new(100);
        pool.decrease(150);
        assert_eq!(pool.current, 0);
        assert_eq!(pool.max, 100);
    }

    #[test]
    fn pool_percentage() {
        let mut pool = Pool::new(100);
        assert_eq!(pool.current_percent(), 1.0);
        pool.decrease(50);
        assert_eq!(pool.current_percent(), 0.5);
    }

    #[test]
    fn pool_set_doesnt_go_above_max() {
        let mut pool = Pool::new(100);
        pool.set(50);
        assert_eq!(pool.current, 50);
        assert_eq!(pool.max, 100);
        pool.set(500);
        assert_eq!(pool.current, 100);
        assert_eq!(pool.max, 100);
    }

    #[test]
    fn pool_set_doesnt_go_below_zero() {
        let mut pool = Pool::new(100);
        pool.set(-50);
        assert_eq!(pool.current, 0);
        assert_eq!(pool.max, 100);
    }

    #[test]
    fn pool_can_be_emptied() {
        let mut pool = Pool::new(100);
        pool.empty();
        assert!(pool.is_empty());
    }

    #[test]
    fn pool_can_be_filled() {
        let mut pool = Pool::new(100);
        pool.empty();
        pool.set_max();
        assert!(pool.is_full());
    }

    #[test]
    fn attribute_roll_test() {
        let a = Attribute {
            base: 90,
            modifiers: 0,
            bonus: 0,
        };

        let result = a.check(89);
        println!("result: {:?}", result);
        assert_eq!(result, RollResult::Success(RollResultType::Normal));
    }
}
