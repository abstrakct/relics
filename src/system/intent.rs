use crate::{
    CurrentGameData, PerformAction, SpendEnergy,
    component::{Energy, Intent, Player, Position, Speed},
    event::{PlayerMoveRelativeEvent, PlayerSpentEnergy},
    rng,
};
use bevy::prelude::*;

pub fn produce_intents_system(
    cgd: Res<CurrentGameData>,
    query: Query<(Entity, &Position), Without<Player>>,
    mut commands: Commands,
) {
    // Here we decide what an entity wants to do next.
    // For now the only possibility is move to a random tile nearby.
    // Later we add more, including combat and other stuff.
    for (e, pos) in query {
        if pos.map == cgd.current_map {
            debug!("Trying to produce new intent for {e:?} on current map");
            let dx = rng::range(-1, 1);
            let dy = rng::range(-1, 1);
            if cgd.maps.map[pos.map].is_walkable(pos.x + dx, pos.y + dy) {
                debug!("Adding new intent for {e:?}: MoveRelative {dx},{dy}");
                commands.entity(e).insert(Intent::MoveRelative { dx, dy });
            } else {
                debug!("Adding new intent for {e:?}: Nothing");
                commands.entity(e).insert(Intent::Nothing);
            }
        }
    }
}

#[allow(clippy::collapsible_if, clippy::single_match)]
pub fn process_intents_system(
    cgd: Res<CurrentGameData>,
    query: Query<(Entity, &Intent, &Energy, &Speed, Option<&Player>)>,
    mut move_queue: EventWriter<PlayerMoveRelativeEvent>,
    mut energy_queue: EventWriter<PlayerSpentEnergy>,
    mut commands: Commands,
) {
    for (entity, intent, energy, speed, player) in query {
        let base_energy_cost = intent.energy_cost();

        debug!("Entity has intent {:?} with base cost of {} energy", intent, base_energy_cost);

        match *intent {
            Intent::MoveRelative { dx, dy } => {
                if let Some(_player) = player {
                    if cgd.maps.map[cgd.player_pos.map].is_walkable(cgd.player_pos.x + dx, cgd.player_pos.y + dy) {
                        debug!("Entity is player, sending PlayerMoveRelativeEvent");
                        move_queue.write(PlayerMoveRelativeEvent { dx, dy });
                        energy_queue.write(PlayerSpentEnergy(base_energy_cost));
                    }
                } else {
                    let needs = (base_energy_cost as f32 * speed.speed) as i32;
                    debug!(
                        "Entity has speed {} and needs {} energy (has {}) to perform intended action!",
                        speed.speed, needs, energy.energy
                    );
                    if energy.energy >= needs {
                        // maybe use commands and add a PerformAction component?
                        // or should all npcs rather observe certain events?
                        commands
                            .entity(entity)
                            .insert((PerformAction::MoveRelative { dx, dy }, SpendEnergy(needs)));
                    }
                }
            }
            _ => {}
        }
    }
}
