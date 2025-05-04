use bevy::prelude::{Entity, World, debug};
use std::cmp::{max, min};

pub fn print_entity_components(world: &World, entity: Entity) {
    debug!("Components for entity {:?}:", entity);

    // Get all components for the entity
    let entity = world.entity(entity);
    let components = entity.archetype().components();

    for component_id in components {
        if let Some(type_info) = world.components().get_info(component_id) {
            debug!("- {}", type_info.name());
        }
    }
}

// Distance algorithms
// Based on / inspired by https://github.com/amethyst/bracket-lib/blob/master/bracket-geometry/src/distance.rs

/// Calculates a Pythagoras distance between two points.
pub fn distance2d_pythagoras(start: (i32, i32), end: (i32, i32)) -> f32 {
    let dx = (max(start.0, end.0) - min(start.0, end.0)) as f32;
    let dy = (max(start.1, end.1) - min(start.1, end.1)) as f32;
    let dsq = (dx * dx) + (dy * dy);
    f32::sqrt(dsq)
}
