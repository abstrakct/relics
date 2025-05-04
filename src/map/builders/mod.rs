use bevy::log::{debug, info};
use meta::dungeon_entry_room_based::DungeonEntryRoomBased;

mod initial;
mod meta;

use super::{Map, MapRect, TileType};
use crate::{component::Position, rng};
use initial::{empty_map::EmptyMapBuilder, rooms::RoomsBuilder};
use meta::{borders::Borders, reveal_all::RevealAll, room_drawer::RoomDrawer, room_sorter::*};

pub struct BuilderMap {
    pub map: Map,
    pub width: usize,
    pub height: usize,
    pub rooms: Option<Vec<MapRect>>,
    pub entry: Option<Position>,
}

pub trait InitialMapBuilder {
    fn build(&mut self, build_data: &mut BuilderMap);

    fn debug_log(&self) {
        debug!("InitialMapBuilder: {:?}", std::any::type_name::<Self>());
    }
}

pub trait MetaMapBuilder {
    fn build(&mut self, build_data: &mut BuilderMap);

    fn debug_log(&self) {
        debug!("Next MetaMapBuilder: {:?}", std::any::type_name::<Self>());
    }
}

pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data: BuilderMap,
}

impl BuilderChain {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            starter: None,
            builders: Vec::new(),
            build_data: BuilderMap {
                map: Map::new(0, "New Map", width, height),
                width,
                height,
                rooms: None,
                entry: None,
            },
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) -> &mut Self {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("BuilderChain already has a starter!"),
        }
        self
    }

    pub fn add(&mut self, metabuilder: Box<dyn MetaMapBuilder>) -> &mut Self {
        self.builders.push(metabuilder);
        self
    }

    pub fn build_map(&mut self) {
        match &mut self.starter {
            None => panic!("BuilderChain has no starter!"),
            Some(starter) => {
                starter.debug_log();
                starter.build(&mut self.build_data)
            }
        }

        for metabuilder in self.builders.iter_mut() {
            metabuilder.debug_log();
            metabuilder.build(&mut self.build_data);
        }
    }

    pub fn get_map(&mut self) -> Map {
        self.build_data.map.clone()
    }

    pub fn get_dungeon_entry(&self) -> Option<Position> {
        for ((y, x), tile) in self.build_data.map.tiles.indexed_iter() {
            if tile.tile_type == TileType::DungeonEntry {
                return Some(Position {
                    x,
                    y,
                    map: self.build_data.map.id as usize,
                });
            }
        }

        None
    }
}

fn empty_map_builder(builder: &mut BuilderChain) {
    builder
        .start_with(EmptyMapBuilder::new())
        .add(RoomDrawer::new())
        .add(Borders::new())
        .add(DungeonEntryRoomBased::new())
        .add(RevealAll::new());
}

fn random_rooms_builder(builder: &mut BuilderChain) {
    builder.start_with(RoomsBuilder::new());

    let sort = rng::roll_str("1d4");
    match sort {
        1 => {
            builder.add(RoomSorter::new(RoomSort::Leftmost));
        }
        2 => {
            builder.add(RoomSorter::new(RoomSort::Rightmost));
        }
        3 => {
            builder.add(RoomSorter::new(RoomSort::Topmost));
        }
        _ => {
            builder.add(RoomSorter::new(RoomSort::Bottommost));
        }
    }

    builder.add(RoomDrawer::new()).add(Borders::new());

    if builder.build_data.map.id == 0 {
        builder.add(DungeonEntryRoomBased::new());
    }

    #[cfg(debug_assertions)]
    {
        builder.add(RevealAll::new());
    }
}

pub fn random_builder(map_id: usize, map_name: &str, width: usize, height: usize) -> BuilderChain {
    let mut builder = BuilderChain::new(width, height);

    let map_type = rng::roll_str("1d2");
    match map_type {
        1 => {
            empty_map_builder(&mut builder);
        }
        _ => {
            random_rooms_builder(&mut builder);
        }
    }

    builder.build_data.map.id = map_id as i32;
    builder.build_data.map.name = map_name.into();
    builder
}

pub fn generate_builder_chain(map_id: usize, map_name: &str, width: usize, height: usize) -> BuilderChain {
    info!("Building map: {map_name} using random_builder");
    random_builder(map_id, map_name, width, height)
}
