use anyhow::Result;
use clap::Parser;
use flexi_logger::{Duplicate, FileSpec, Logger, WriteMode};
use log::info;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

mod action;
mod cli;
mod config;
mod game;
mod rng;
mod uimode;

pub use action::*;
use cli::CliArgs;
pub use config::*;
pub use game::*;

#[macro_use]
extern crate lazy_static;

pub const VERSION_STRING: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("VERGEN_GIT_DESCRIBE"),
    " (",
    env!("VERGEN_BUILD_TIMESTAMP"),
    ")"
);

async fn tokio_main() -> Result<()> {
    log::debug!("Loading config files");
    config::load_config(Some("config"));

    let args = CliArgs::parse();

    log::debug!("Starting game");
    let mut game = Game::new(60.0, 60.0);

    // build system schedules here I guess

    // insert resources here I guess

    let seed;
    if args.seed == 0 {
        info!("No RNG seed specified - using current unix epoch time.");
        seed = Seed(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    } else {
        seed = Seed(args.seed);
        info!("RNG seed specified on command line: {}", seed.0);
    }
    info!("Setting RNG seed to {}", seed.0);
    rng::reseed(seed.0);
    game.world.insert_resource(seed);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let _logger = Logger::try_with_env_or_str("info")?
        .log_to_file(FileSpec::default().directory("logs").basename(env!("CARGO_PKG_NAME")))
        .write_mode(WriteMode::BufferAndFlush)
        .duplicate_to_stderr(Duplicate::Warn)
        .create_symlink("current-log")
        .format_for_files(flexi_logger::detailed_format)
        .start()?;

    info!("{} {} starting", env!("CARGO_PKG_NAME"), VERSION_STRING);

    if let Err(e) = tokio_main().await {
        eprintln!("{} error: {:?}", env!("CARGO_PKG_NAME"), e);
        Err(e)
    } else {
        Ok(())
    }
}
