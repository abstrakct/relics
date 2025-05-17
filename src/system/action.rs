use crate::{
    // TurnState,
    component::{Energy, PerformAction, Position, SpendEnergy},
};
use bevy::prelude::*;

pub fn movement_action_system(mut query: Query<(&mut Position, &PerformAction)>) {
    for (mut position, action) in query.iter_mut() {
        debug!("Performing movement_action: {:?}", action);
        match action {
            PerformAction::MoveAbsolute { x, y } => {
                position.x = *x;
                position.y = *y;
            }
            PerformAction::MoveRelative { dx, dy } => {
                position.x += dx;
                position.y += dy;
            }
        }
    }
}

pub fn spend_energy_system(query: Query<(&mut Energy, &SpendEnergy)>) {
    for (mut energy, spend) in query {
        debug!("Spending {} out of {} energy", spend.0, energy.energy);
        energy.energy -= spend.0;
    }
}
