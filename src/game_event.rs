use bevy_ecs::event::Event;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use strum::Display;

/// Trait for types that can be converted to and from events
pub trait EventConvertible: Clone + Send + Sync + Default + DeserializeOwned + 'static {
    /// Convert from a JSON value (used in config files)
    fn from_value(value: Value) -> Option<Self>;
    /// Convert to a JSON value (used in config files)
    fn to_value(&self) -> Value;
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
    fn from_value(value: Value) -> Option<Self> {
        match value {
            Value::String(s) => match s.as_str() {
                "Quit" => Some(GameEvent::Quit),
                "NextMenuItem" => Some(GameEvent::NextMenuItem),
                "PrevMenuItem" => Some(GameEvent::PrevMenuItem),
                "SelectMenuItem" => Some(GameEvent::SelectMenuItem),
                "GenerateWorld" => Some(GameEvent::GenerateWorld),
                "StartNewGame" => Some(GameEvent::StartNewGame),
                _ => None,
            },
            Value::Object(obj) => {
                if let Some(player_move) = obj.get("PlayerMove") {
                    if let Some(x) = player_move.get("x").and_then(|v| v.as_i64()) {
                        if let Some(y) = player_move.get("y").and_then(|v| v.as_i64()) {
                            return Some(GameEvent::PlayerMove {
                                x: x as i32,
                                y: y as i32,
                            });
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn to_value(&self) -> Value {
        match self {
            GameEvent::Quit => Value::String("Quit".to_string()),
            GameEvent::NextMenuItem => Value::String("NextMenuItem".to_string()),
            GameEvent::PrevMenuItem => Value::String("PrevMenuItem".to_string()),
            GameEvent::SelectMenuItem => Value::String("SelectMenuItem".to_string()),
            GameEvent::GenerateWorld => Value::String("GenerateWorld".to_string()),
            GameEvent::StartNewGame => Value::String("StartNewGame".to_string()),
            GameEvent::PlayerMove { x, y } => {
                let mut obj = serde_json::Map::new();
                let mut move_obj = serde_json::Map::new();
                move_obj.insert("x".to_string(), Value::Number((*x).into()));
                move_obj.insert("y".to_string(), Value::Number((*y).into()));
                obj.insert("PlayerMove".to_string(), Value::Object(move_obj));
                Value::Object(obj)
            }
        }
    }
}
