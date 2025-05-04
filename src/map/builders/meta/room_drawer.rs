use bevy::log::{debug, info};

use super::{BuilderMap, MetaMapBuilder};
use crate::{
    map::{FLOOR_TILE, MapRect},
    rng,
    utils::distance2d_pythagoras,
};

/// Meta map builder which translates rooms to tiles.
pub struct RoomDrawer;

impl MetaMapBuilder for RoomDrawer {
    fn build(&mut self, build_data: &mut BuilderMap) {
        self.build_map(build_data);
    }
}

impl RoomDrawer {
    pub fn new() -> Box<RoomDrawer> {
        Box::new(RoomDrawer {})
    }

    fn rectangle(&mut self, build_data: &mut BuilderMap, room: &MapRect) {
        for y in room.y1..=room.y2 {
            for x in room.x1..=room.x2 {
                build_data.map.define_tile(x, y, FLOOR_TILE);
            }
        }
    }

    fn circle(&mut self, build_data: &mut BuilderMap, room: &MapRect) {
        let radius = i32::min(room.x2 - room.x1, room.y2 - room.y1) as f32 / 2.0;
        let center = room.center();
        for y in room.y1..=room.y2 {
            for x in room.x1..=room.x2 {
                let distance = distance2d_pythagoras(center, (x, y));
                if distance <= radius {
                    build_data.map.define_tile(x, y, FLOOR_TILE);
                }
            }
        }
    }

    pub fn build_map(&mut self, build_data: &mut BuilderMap) {
        debug!("In RoomDrawer meta map builder build_map()");

        let rooms: Vec<MapRect>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            // TODO: this should be a handleable error
            panic!("RoomDrawer requires rooms!");
        }

        for room in rooms.iter() {
            info!("building room in rectangle shape: {:?}", room);
            let shape = rng::roll_str("1d3");
            match shape {
                1 => self.circle(build_data, room),
                _ => self.rectangle(build_data, room),
            }
        }
    }
}
