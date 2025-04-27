use super::{BuilderMap, InitialMapBuilder};
use crate::map::MapRect;

pub struct EmptyMapBuilder;

impl InitialMapBuilder for EmptyMapBuilder {
    fn build(&mut self, build_data: &mut BuilderMap) {
        self.build_map(build_data);
    }
}

impl EmptyMapBuilder {
    pub fn new() -> Box<EmptyMapBuilder> {
        Box::new(EmptyMapBuilder {})
    }

    fn build_map(&mut self, build_data: &mut BuilderMap) {
        // Create one big room, leave a 1-tile border
        let rooms: Vec<MapRect> = vec![MapRect::new(1, 1, build_data.width as i32 - 2, build_data.height as i32 - 2)];
        build_data.rooms = Some(rooms);
    }
}
