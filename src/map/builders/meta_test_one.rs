use super::{BuilderMap, MetaMapBuilder};

pub struct TestOne;

impl MetaMapBuilder for TestOne {
    fn build(&mut self, build_data: &mut BuilderMap) {
        self.build_map(build_data);
    }
}

impl TestOne {
    pub fn new() -> Box<TestOne> {
        Box::new(TestOne {})
    }

    pub fn build_map(&mut self, build_data: &mut BuilderMap) {
        log::debug!("In TestOne meta map builder build_map()");
    }
}
