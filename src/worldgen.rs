use crate::{
    CFG, GameEvent, GameState,
    component::Position,
    game::CurrentGameData,
    map::{Map, Maps, generate_builder_chain},
    player, utils,
};
use bevy::log::*;
use bevy::prelude::*;

fn generate_maps(first: usize, last: usize) -> (Maps, Position) {
    let mut maps = Maps::new();
    let mut dungeon_entry = Position::default();

    // Add an empty map at index 0
    maps.map.push(Map::new(0, "Zero", 1, 1));

    for i in first..=last {
        info!("Generating map D:{}", i);
        let name = format!("D:{}", i);

        let mut builder = generate_builder_chain(i, &name, 80, 50);
        builder.build_map();

        if let Some(p) = builder.get_dungeon_entry() {
            debug!("Found dungeon entry in map {} at {},{}", p.map, p.x, p.y);
            dungeon_entry = p;
        }

        maps.map.push(builder.get_map());
    }

    (maps, dungeon_entry)
}

pub fn generate_world(world: &mut World) {
    info!("Starting world generation");

    let cfg = CFG.lock().unwrap();
    let first_map: usize = 1;
    let last_map: usize = cfg.config.world.max_levels as usize;
    std::mem::drop(cfg);

    info!("Deleting any existing maps");
    world.remove_resource::<Maps>();

    info!("Deleting any existing entities");
    world.clear_entities();

    info!("Generating maps");
    let (maps, dungeon_entry) = generate_maps(first_map, last_map);

    let gamedata = CurrentGameData {
        current_map: first_map,
        player_pos: dungeon_entry,
    };

    info!("Inserting resources");
    world.insert_resource(maps);
    world.insert_resource(gamedata);

    info!("Spawning player entity");
    let player = player::spawn(world);
    utils::print_entity_components(world, player);

    // Change game state
    let mut game_state = world.resource_mut::<NextState<GameState>>();
    game_state.set(GameState::Menu); // will use default MenuState which is MainMenu

    world.send_event(GameEvent::NextMenuItem);
}
