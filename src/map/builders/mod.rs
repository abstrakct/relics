mod initial_empty_map;
mod initial_rooms;
mod meta_room_drawer;
mod meta_test_one;

use super::{Map, MapRect};
use crate::rng;
use initial_empty_map::EmptyMapBuilder;
use initial_rooms::RoomsBuilder;
use meta_room_drawer::RoomDrawer;
use meta_test_one::TestOne;

pub struct BuilderMap {
    pub map: Map,
    pub width: usize,
    pub height: usize,
    pub rooms: Option<Vec<MapRect>>,
}

pub trait InitialMapBuilder {
    fn build(&mut self, build_data: &mut BuilderMap);

    fn debug_log(&self) {
        log::debug!("InitialMapBuilder: {:?}", std::any::type_name::<Self>());
    }
}

pub trait MetaMapBuilder {
    fn build(&mut self, build_data: &mut BuilderMap);

    fn debug_log(&self) {
        log::debug!("Next MetaMapBuilder: {:?}", std::any::type_name::<Self>());
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
}

fn empty_map_builder(builder: &mut BuilderChain) {
    builder
        .start_with(EmptyMapBuilder::new())
        .add(TestOne::new())
        .add(RoomDrawer::new());
}

fn random_rooms_builder(builder: &mut BuilderChain) {
    builder
        .start_with(RoomsBuilder::new())
        .add(TestOne::new())
        .add(RoomDrawer::new());
}

pub fn random_builder(map_id: usize, map_name: &str, width: usize, height: usize) -> BuilderChain {
    let mut builder = BuilderChain::new(width, height);

    let map_type = rng::roll_str("1d2");
    match map_type {
        1 => {
            random_rooms_builder(&mut builder);
        }
        _ => {
            empty_map_builder(&mut builder);
        }
    }

    builder.build_data.map.id = map_id as i32;
    builder.build_data.map.name = map_name.into();
    builder
}

pub fn generate_builder_chain(map_id: usize, map_name: &str, width: usize, height: usize) -> BuilderChain {
    log::info!("Building map: {map_name}");
    random_builder(map_id, map_name, width, height)
}
