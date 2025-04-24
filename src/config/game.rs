use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GameConfig {
    pub name: String,
}

impl GameConfig {
    pub fn new() -> GameConfig {
        GameConfig {
            name: "Rusty Legions".to_string(),
        }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self::new()
    }
}
