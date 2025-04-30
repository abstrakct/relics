use super::{BORDER_TILE, BuilderMap, MetaMapBuilder, Tile, TileType};

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
        build_data
            .map
            .tiles
            .insert_col(0, vec![BORDER_TILE.clone(); build_data.height]);
        build_data.width += 1;
        build_data.map.width += 1;

        build_data.map.tiles.push_col(vec![BORDER_TILE.clone(); build_data.height]);
        build_data.width += 1;
        build_data.map.width += 1;

        build_data
            .map
            .tiles
            .insert_row(0, vec![BORDER_TILE.clone(); build_data.width]);
        build_data.height += 1;
        build_data.map.height += 1;

        build_data.map.tiles.push_row(vec![BORDER_TILE; build_data.width]);
        build_data.height += 1;
        build_data.map.height += 1;
    }
}
