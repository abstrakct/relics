use crate::{
    CFG, GameEvent, GameState,
    game::CurrentGameData,
    map::{Map, Maps, generate_builder_chain},
};
use bevy::prelude::*;

fn generate_maps(first: usize, last: usize) -> Maps {
    let mut maps = Maps::new();

    // Add an empty map at index 0
    maps.map.push(Map::new(0, "Zero", 1, 1));

    for i in first..=last {
        log::info!("Generating map D:{}", i);
        let name = format!("D:{}", i);

        let mut builder = generate_builder_chain(i, &name, 80, 50);
        builder.build_map();

        maps.map.push(builder.get_map());
    }

    maps
}

pub fn generate_world(world: &mut World) {
    log::info!("Starting world generation");

    let cfg = CFG.lock().unwrap();
    let first_map: usize = 1;
    let last_map: usize = cfg.config.world.max_levels as usize;
    std::mem::drop(cfg);

    log::info!("Deleting any existing maps");
    world.remove_resource::<Maps>();

    log::info!("Deleting any existing entities");
    world.clear_entities();

    log::info!("Generating maps");
    let maps = generate_maps(first_map, last_map);

    let gamestate = CurrentGameData { current_map: first_map };

    log::info!("Inserting resources");
    world.insert_resource(maps);
    world.insert_resource(gamestate);

    let mut game_state = world.resource_mut::<NextState<GameState>>();
    game_state.set(GameState::Menu); // will use default MenuState which is MainMenu

    world.send_event(GameEvent::NextMenuItem);
}
