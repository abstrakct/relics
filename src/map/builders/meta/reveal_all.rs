use super::{BuilderMap, MetaMapBuilder};

pub struct RevealAll;

impl MetaMapBuilder for RevealAll {
    fn build(&mut self, build_data: &mut BuilderMap) {
        self.build_map(build_data);
    }
}

impl RevealAll {
    pub fn new() -> Box<RevealAll> {
        Box::new(RevealAll {})
    }

    pub fn build_map(&mut self, build_data: &mut BuilderMap) {
        build_data.map.reveal_all();
    }
}
