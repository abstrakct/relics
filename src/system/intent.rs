use crate::{
    CurrentGameData,
    component::Intent,
    event::{PlayerMoveRelativeEvent, PlayerSpentEnergy},
};
use bevy::prelude::*;

#[allow(clippy::collapsible_if, clippy::single_match)]
pub fn intent_system(
    cgd: Res<CurrentGameData>,
    mut move_queue: EventWriter<PlayerMoveRelativeEvent>,
    mut energy_queue: EventWriter<PlayerSpentEnergy>,
    query: Query<(Entity, &Intent)>,
) {
    for (entity, intent) in query {
        let base_energy_cost = intent.energy_cost();
        debug!(
            "entity {} has intent {:?} - it has base cost of {} energy",
            entity, intent, base_energy_cost
        );

        match *intent {
            Intent::MoveRelative { dx, dy } => {
                if entity == cgd.player.unwrap() {
                    if cgd.maps.map[cgd.player_pos.map as usize].is_walkable(cgd.player_pos.x + dx, cgd.player_pos.y + dy) {
                        debug_once!("entity is player, sending PlayerMoveRelativeEvent");
                        move_queue.write(PlayerMoveRelativeEvent { dx, dy });
                        energy_queue.write(PlayerSpentEnergy(base_energy_cost));
                    }
                }
            }
            _ => {}
        }
    }
}
