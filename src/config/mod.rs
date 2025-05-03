use bevy::log::{debug, info};
use serde::Deserialize;
use std::fs;
use std::sync::Mutex;

mod configmaster;
mod game;
mod player;
mod ui;
mod world;

pub use configmaster::*;
pub use game::*;
pub use player::*;
pub use ui::*;
pub use world::*;

lazy_static! {
    pub static ref CFG: Mutex<ConfigMaster> = Mutex::new(ConfigMaster::new());
}

const CONFIG_DIR: &str = "config";
const DATA_DIR: &str = "data";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub game: GameConfig,
    pub player: PlayerConfig,
    pub world: WorldConfig,
}

pub fn load_config(config_path: Option<&str>, data_path: Option<&str>) {
    let config_path = config_path.unwrap_or(CONFIG_DIR);
    let data_path = data_path.unwrap_or(DATA_DIR);

    info!("Loading config files from path: '{}'", config_path);
    debug!("Loading game config file");
    let contents = fs::read_to_string(format!("{}/game.ron", config_path)).expect("Failed to read game config file");
    let gameconfig: GameConfig = ron::de::from_str(&contents).expect("Failed to parse game config file");

    debug!("Loading player config file");
    let contents = fs::read_to_string(format!("{}/player.ron", config_path)).expect("Failed to read player config file");
    let playerconfig: PlayerConfig = ron::de::from_str(&contents).expect("Failed to parse player config file");

    debug!("Loading world data file");
    let contents = fs::read_to_string(format!("{}/world.ron", data_path)).expect("Failed to read world data file");
    let worldconfig: WorldConfig = ron::de::from_str(&contents).expect("Failed to parse world data file");

    CFG.lock().unwrap().load(Config {
        game: gameconfig,
        player: playerconfig,
        world: worldconfig,
    });
}
