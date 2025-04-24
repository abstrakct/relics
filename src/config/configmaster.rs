use crate::{Config, GameConfig, PlayerConfig, WorldConfig};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigMaster {
    pub config: Config,
}

impl ConfigMaster {
    pub fn new() -> ConfigMaster {
        ConfigMaster {
            config: Config {
                game: GameConfig::new(),
                player: PlayerConfig::new(),
                world: WorldConfig::new(),
            },
        }
    }

    pub fn load(&mut self, config: Config) {
        self.config = config;
    }
}

impl Default for ConfigMaster {
    fn default() -> Self {
        Self::new()
    }
}
