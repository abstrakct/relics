use super::{BuilderMap, MetaMapBuilder};
use crate::map::{MapRect, TileType};

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
                build_data.map.set_tiletype(x, y, TileType::Floor);
                build_data.map.set_blocks_view(x, y, true);
                build_data.map.set_walkable(x, y, true);
            }
        }
    }

    pub fn build_map(&mut self, build_data: &mut BuilderMap) {
        log::debug!("In RoomDrawer meta map builder build_map()");

        let rooms: Vec<MapRect>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            // TODO: this should be a handleable error
            panic!("RoomDrawer requires rooms!");
        }

        for room in rooms.iter() {
            log::info!("building room in rectangle shape: {:?}", room);
            self.rectangle(build_data, room);
        }
    }
}
