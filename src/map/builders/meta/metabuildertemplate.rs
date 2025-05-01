use super::{BuilderMap, MetaMapBuilder};

pub struct Template;

impl MetaMapBuilder for Template {
    fn build(&mut self, build_data: &mut BuilderMap) {
        self.build_map(build_data);
    }
}

impl Template {
    pub fn new() -> Box<Template> {
        Box::new(Template {})
    }

    pub fn build_map(&mut self, _build_data: &mut BuilderMap) {
        log::debug!("In Template meta map builder build_map()");
    }
}
