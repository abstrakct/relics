use bevy_ecs::event::Event;

// This file holds all events (except for GameEvent)

// TODO: should be entity move, not player move??

#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub struct PlayerMoveRelativeEvent {
    pub dx: i32,
    pub dy: i32,
}

#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub struct PlayerSpentEnergy(pub i32);
