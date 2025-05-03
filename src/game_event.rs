use bevy_ecs::event::Event;
use serde::{
    // de::{self, Deserializer, Visitor},
    Deserialize,
    Serialize,
};
use strum::Display;

/// Trait for types that can be converted to and from events
pub trait EventConvertible: Clone + Send + Sync + Default + 'static {
    /// Convert from a string representation (used in config files)
    fn from_str(s: &str) -> Option<Self>;
    /// Convert to a string representation (used in config files)
    fn to_string(&self) -> String;
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum GameEvent {
    // Tick,
    Quit,
    // Suspend,
    // Render,
    // Resize(u16, u16),
    GenerateWorld,
    StartNewGame,
    NextMenuItem,
    PrevMenuItem,
    SelectMenuItem,
    PlayerMove { x: i32, y: i32 },
    // Error(String),
}

impl Default for GameEvent {
    fn default() -> Self {
        GameEvent::Quit
    }
}

impl EventConvertible for GameEvent {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Quit" => Some(GameEvent::Quit),
            "NextMenuItem" => Some(GameEvent::NextMenuItem),
            "PrevMenuItem" => Some(GameEvent::PrevMenuItem),
            "SelectMenuItem" => Some(GameEvent::SelectMenuItem),
            "GenerateWorld" => Some(GameEvent::GenerateWorld),
            "StartNewGame" => Some(GameEvent::StartNewGame),
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            GameEvent::Quit => "Quit".to_string(),
            GameEvent::NextMenuItem => "NextMenuItem".to_string(),
            GameEvent::PrevMenuItem => "PrevMenuItem".to_string(),
            GameEvent::SelectMenuItem => "SelectMenuItem".to_string(),
            GameEvent::GenerateWorld => "GenerateWorld".to_string(),
            GameEvent::StartNewGame => "StartNewGame".to_string(),
            GameEvent::PlayerMove { x, y } => format!("PlayerMove {{ x: {}, y: {} }}", x, y),
        }
    }
}
