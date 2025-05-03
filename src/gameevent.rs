use bevy_ecs::event::Event;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Event, Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum GameEvent {
    // Generic events
    Quit,
    // World events
    GenerateWorld,
    // Menu events
    StartNewGame,
    NextMenuItem,
    PrevMenuItem,
    SelectMenuItem,
    // Player events
    PlayerMove { x: i32, y: i32 },
}
