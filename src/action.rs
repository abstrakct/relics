use serde::{
    // de::{self, Deserializer, Visitor},
    Deserialize,
    Serialize,
};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Action {
    Tick,
    Quit,
    Suspend,
    Render,
    Resize(u16, u16),
    GenerateWorld,
    StartNewGame,
    NextMenuItem,
    PrevMenuItem,
    SelectMenuItem,
    PlayerMove { x: i32, y: i32 },
    Error(String),
}
