use crate::{
    CFG,
    game::Game,
    map::{Map, Maps, generate_builder_chain},
};
// use bevy_ecs::prelude::*;

fn generate_maps(game: &mut Game, first: usize, last: usize) {
    let mut maps = Maps::new();

    // Add an empty map at index 0
    maps.map.push(Map::new(0, "Zero", 1, 1));

    for i in first..=last {
        log::info!("Generating map D:{}", i);
        let name = format!("D:{}", i);

        let mut builder = generate_builder_chain(i, &name, 80, 50);
        builder.build_map();

        // maps.map.push(map);
    }

    // game.world.insert_resource(maps);
}

pub fn generate_world(game: &mut Game) {
    log::info!("Starting world generation");

    let cfg = CFG.lock().unwrap();
    let first_map: usize = 1;
    let last_map: usize = cfg.config.world.max_levels as usize;
    std::mem::drop(cfg);

    log::info!("Deleting any existing maps");
    game.world.remove_resource::<Maps>();

    log::info!("Deleting any existing entities");
    game.world.clear_entities();

    log::info!("Generating maps");
    generate_maps(game, first_map, last_map);
}
