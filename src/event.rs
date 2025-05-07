use bevy_ecs::event::Event;

// This file holds all events (except for GameEvent)

// TODO: add intents, either as separate events, or one enum
// TODO: should be entity move, not player move??

#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub struct PlayerMoveRelativeEvent {
    pub dx: i32,
    pub dy: i32,
}
