use super::{BuilderMap, MetaMapBuilder};

pub enum RoomSort {
    Leftmost,
    Rightmost,
    Topmost,
    Bottommost,
}

pub struct RoomSorter {
    sort_by: RoomSort,
}

impl MetaMapBuilder for RoomSorter {
    fn build(&mut self, build_data: &mut BuilderMap) {
        self.sorter(build_data);
    }
}

impl RoomSorter {
    pub fn new(sort_by: RoomSort) -> Box<RoomSorter> {
        Box::new(RoomSorter { sort_by })
    }

    pub fn sorter(&mut self, build_data: &mut BuilderMap) {
        match self.sort_by {
            RoomSort::Leftmost => build_data.rooms.as_mut().unwrap().sort_by(|a, b| a.x1.cmp(&b.x1)),
            RoomSort::Rightmost => build_data.rooms.as_mut().unwrap().sort_by(|a, b| b.x2.cmp(&a.x2)),
            RoomSort::Topmost => build_data.rooms.as_mut().unwrap().sort_by(|a, b| a.y1.cmp(&b.y1)),
            RoomSort::Bottommost => build_data.rooms.as_mut().unwrap().sort_by(|a, b| b.y2.cmp(&a.y2)),
        }
    }
}
