use bevy_ecs::{entity::Entity, event::Event};

// This file holds all events (except for GameEvent)

// TODO: add intents, either as separate events, or one enum
// TODO: should be entity move, not player move??

#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub struct PlayerMoveEvent {
    pub x: i32,
    pub y: i32,
}

#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub enum IntentEvent {
    MoveRelative { entity: Entity, dx: i32, dy: i32 },
    MoveAbsolute { entity: Entity, x: i32, y: i32 },
    PlayerMoveRelative { dx: i32, dy: i32 },
}
