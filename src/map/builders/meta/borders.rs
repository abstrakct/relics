use super::{BuilderMap, MetaMapBuilder, TileType};

pub struct Borders;

impl MetaMapBuilder for Borders {
    fn build(&mut self, build_data: &mut BuilderMap) {
        self.build_map(build_data);
    }
}

impl Borders {
    pub fn new() -> Box<Borders> {
        Box::new(Borders {})
    }

    pub fn build_map(&mut self, build_data: &mut BuilderMap) {
        build_data.map.insert_col(0, build_data.height);
        build_data.width += 1;
        build_data.map.width += 1;

        // build_data.map.tile_type.push_col(vec![TileType::Wall; build_data.height]);
        // build_data.width += 1;
        // build_data.map.width += 1;

        // build_data.map.tile_type.insert_row(0, vec![TileType::Wall; build_data.width]);
        // build_data.height += 1;
        // build_data.map.height += 1;

        // build_data.map.tile_type.push_row(vec![TileType::Wall; build_data.width]);
        // build_data.height += 1;
        // build_data.map.height += 1;
    }
}
