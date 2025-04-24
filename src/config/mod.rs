use log::{debug, info};
use serde::Deserialize;
use std::fs;
use std::sync::Mutex;

mod configmaster;
mod game;
mod player;
mod world;

pub use configmaster::*;
pub use game::*;
pub use player::*;
pub use world::*;

lazy_static! {
    pub static ref CFG: Mutex<ConfigMaster> = Mutex::new(ConfigMaster::new());
}

const CONFIG_DIR: &str = "config";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub game: GameConfig,
    pub player: PlayerConfig,
    pub world: WorldConfig,
}

pub fn load_config(path: Option<&str>) {
    let path = path.unwrap_or(CONFIG_DIR);

    info!("Loading config files from path: '{}'", path);
    debug!("Loading game config file");
    let contents = fs::read_to_string(format!("{}/game.ron", path)).expect("Failed to read game config file");
    let gameconfig: GameConfig = ron::de::from_str(&contents).expect("Failed to parse game config");

    debug!("Loading player config file");
    let contents = fs::read_to_string(format!("{}/player.ron", path)).expect("Failed to read player config file");
    let playerconfig: PlayerConfig = ron::de::from_str(&contents).expect("Failed to parse player config");

    debug!("Loading world config file");
    let contents = fs::read_to_string(format!("{}/world.ron", path)).expect("Failed to read world config file");
    let worldconfig: WorldConfig = ron::de::from_str(&contents).expect("Failed to parse world config");

    CFG.lock().unwrap().load(Config {
        game: gameconfig,
        player: playerconfig,
        world: worldconfig,
    });
}
