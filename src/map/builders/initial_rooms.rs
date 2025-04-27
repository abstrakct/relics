use super::{BuilderMap, InitialMapBuilder};
use crate::map::MapRect;
use crate::rng;
pub struct RoomsBuilder {}

impl InitialMapBuilder for RoomsBuilder {
    fn build(&mut self, build_data: &mut BuilderMap) {
        self.build_map(build_data);
    }
}

impl RoomsBuilder {
    pub fn new() -> Box<RoomsBuilder> {
        Box::new(RoomsBuilder {})
    }

    fn build_map(&mut self, build_data: &mut BuilderMap) {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;
        let mut rooms: Vec<MapRect> = Vec::new();

        for _i in 0..MAX_ROOMS {
            let w = rng::range(MIN_SIZE, MAX_SIZE);
            let h = rng::range(MIN_SIZE, MAX_SIZE);
            let x = rng::roll_dice(1, build_data.map.width as i32 - w - 1) - 1;
            let y = rng::roll_dice(1, build_data.map.height as i32 - h - 1) - 1;
            let new_room = MapRect::new(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                rooms.push(new_room);
            }
        }

        build_data.rooms = Some(rooms);
    }
}
