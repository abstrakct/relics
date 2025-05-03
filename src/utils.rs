use bevy::prelude::{Entity, World, debug};

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
