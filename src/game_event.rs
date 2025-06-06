use bevy_ecs::event::Event;
use serde::{
    // de::{self, Deserializer, Visitor},
    Deserialize,
    Serialize,
};
use strum::Display;

#[derive(Event, Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum GameEvent {
    Quit,
    ShowMainMenu,
    GenerateWorld,
    StartNewGame,
    NextMenuItem,
    PrevMenuItem,
    SelectMenuItem,
    PlayerMoveAbsolute { x: i32, y: i32 },
    PlayerMoveRelative { dx: i32, dy: i32 },
    // Suspend,
    // Tick,
    // Render,
    // Resize(u16, u16),
    // Error(String),
}
