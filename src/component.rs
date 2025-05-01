use bevy_ecs::prelude::*;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

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
    pub fn new(max: i32) -> Pool {
        Pool { current: max, max }
    }

    pub fn decrease(&mut self, amount: i32) {
        self.current -= amount;
        if self.current < 0 {
            self.current = 0;
        }
    }

    pub fn increase(&mut self, amount: i32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    pub fn empty(&mut self) {
        self.current = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    pub fn is_full(&self) -> bool {
        self.current == self.max
    }

    pub fn percent(&self) -> f32 {
        self.current as f32 / self.max as f32
    }

    pub fn set(&mut self, amount: i32) {
        self.current = amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }

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

//-------------------//
// Marker components //
//-------------------//

#[derive(Component, Default, Serialize, Deserialize)]
#[require(Position, Renderable, Name, Stats, Attributes)]
pub struct Player;

//----------------------//
// Components with data //
//----------------------//

#[derive(Component, Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub map: usize,
}

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
    pub order: i32,
    pub always: bool,
}

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Name {
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
