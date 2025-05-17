use bevy_ecs::event::Event;

// This file holds all events (except for GameEvent)

// We must convert this to an enum of events or actions

#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub struct PlayerMoveRelativeEvent {
    pub dx: i32,
    pub dy: i32,
}

#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub struct PlayerSpentEnergy(pub i32);

#[allow(dead_code)]
#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub enum EntityAction {
    MoveRelative,
    ASdfDdoijfwe,
}
