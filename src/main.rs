use anyhow::Result;
use flexi_logger::{Duplicate, FileSpec, Logger, WriteMode};
use log::info;
use std::env;

mod action;
mod config;
mod game;

pub use action::*;
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
