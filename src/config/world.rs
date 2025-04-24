use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WorldConfig {
    pub name: String,
    pub max_levels: i32,
}

impl WorldConfig {
    pub fn new() -> WorldConfig {
        WorldConfig {
            name: "The World".to_string(),
            max_levels: 10,
        }
    }
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self::new()
    }
}
