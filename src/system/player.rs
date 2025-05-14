use crate::{
    CurrentGameData,
    component::{Energy, Player, Position, Speed},
    event::{PlayerMoveRelativeEvent, PlayerSpentEnergy},
};
use bevy::prelude::*;

/// Update player position in CurrentGameData resource
pub fn update_player_pos(mut cgd: ResMut<CurrentGameData>, query: Query<(&Player, &Position)>) {
    if let Ok((_player, pos)) = query.single() {
        cgd.player_pos = *pos;
    }
}

pub fn player_move_system(
    mut player_move: EventReader<PlayerMoveRelativeEvent>,
    mut query: Query<(&Player, &mut Position)>,
) {
    for pm in player_move.read() {
        debug_once!("Got PlayerMoveRelativeEvent, moving player");
        if let Ok((_entity, mut pos)) = query.single_mut() {
            pos.x += pm.dx;
            pos.y += pm.dy;
        }
    }
}

pub fn player_spent_energy_system(
    mut energy_queue: EventReader<PlayerSpentEnergy>,
    mut query: Query<&mut Energy>,
    player_query: Query<(&Player, &Speed)>,
) {
    if let Ok((_, speed)) = player_query.single() {
        for e in energy_queue.read() {
            debug!("{:?}", e);
            for mut comp in query.iter_mut() {
                debug!("Found entity with Energy component: {:?}", comp);
                comp.energy += (e.0 as f32 * speed.speed) as i32;
                debug!("Energy component after increase: {:?}", comp);
            }
        }
    }
}
