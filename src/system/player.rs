use crate::{
    CurrentGameData, TurnState,
    component::{Energy, Player, Position, Speed},
    event::{PlayerMoveRelativeEvent, PlayerSpentEnergy},
};
use bevy::prelude::*;

/// Update player position in CurrentGameData resource
pub fn update_player_pos(mut cgd: ResMut<CurrentGameData>, query: Query<&Position, With<Player>>) {
    if let Ok(pos) = query.single() {
        cgd.player_pos = *pos;
    }
}

pub fn player_move_system(
    mut player_move: EventReader<PlayerMoveRelativeEvent>,
    mut query: Query<&mut Position, With<Player>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    for pm in player_move.read() {
        debug_once!("Got PlayerMoveRelativeEvent, moving player");
        if let Ok(mut pos) = query.single_mut() {
            pos.x += pm.dx;
            pos.y += pm.dy;
        }
        next_state.set(TurnState::NotPlayersTurn);
    }
}

pub fn player_spent_energy_system(
    cgd: Res<CurrentGameData>,
    mut energy_queue: EventReader<PlayerSpentEnergy>,
    mut energy_query: Query<(&mut Energy, &Position)>,
    player_query: Query<&Speed, With<Player>>,
) {
    if let Ok(speed) = player_query.single() {
        for e in energy_queue.read() {
            debug!("{:?}", e);
            for (mut energy, pos) in energy_query.iter_mut() {
                if pos.map == cgd.current_map {
                    debug!("Found entity on current map with Energy component: {:?}", energy);
                    energy.energy += (e.0 as f32 * speed.speed) as i32;
                    debug!("Energy component after increase: {:?}", energy);
                }
            }
        }
    }
}
