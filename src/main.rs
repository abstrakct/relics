use anyhow::Result;
use flexi_logger::{Duplicate, FileSpec, Logger, WriteMode};
use log::info;
use std::env;

// use std::time::{SystemTime, UNIX_EPOCH};

mod config;
pub use config::*;

#[macro_use]
extern crate lazy_static;

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
        .create_symlink("last-log")
        .format_for_files(flexi_logger::detailed_format)
        .start()?;

    info!("{} starting", env!("CARGO_PKG_NAME"));

    if let Err(e) = tokio_main().await {
        eprintln!("{} error: {:?}", env!("CARGO_PKG_NAME"), e);
        Err(e)
    } else {
        Ok(())
    }
}
