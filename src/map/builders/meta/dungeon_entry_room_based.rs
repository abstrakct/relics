use bevy::log::debug;

use super::{BuilderMap, MetaMapBuilder};
use crate::{component::Position, map::DUNGEON_ENTRY_TILE};

pub struct DungeonEntryRoomBased;

impl MetaMapBuilder for DungeonEntryRoomBased {
    fn build(&mut self, build_data: &mut BuilderMap) {
        self.build_map(build_data);
    }
}

impl DungeonEntryRoomBased {
    pub fn new() -> Box<DungeonEntryRoomBased> {
        Box::new(DungeonEntryRoomBased {})
    }

    pub fn build_map(&mut self, build_data: &mut BuilderMap) {
        if let Some(rooms) = &build_data.rooms {
            let start_pos = rooms[0].center();
            build_data.entry = Some(Position {
                x: start_pos.0 as usize,
                y: start_pos.1 as usize,
                map: build_data.map.id as usize,
            });
            // let upstairs_idx = build_data.map.idx(start_pos.0, start_pos.1);
            build_data.map.define_tile(start_pos.0, start_pos.1, DUNGEON_ENTRY_TILE);
            // build_data.map.starting_position = build_data.starting_position.unwrap();
            debug!("Added dungeon entry at {},{}", start_pos.0, start_pos.1);
        } else {
            panic!("Can't create room based dungeon entry - no rooms!");
        }
    }
}
