fn main() {
    println!("Hello, world!");
mod config;
pub use config::*;

#[macro_use]
extern crate lazy_static;

async fn tokio_main() -> Result<()> {
    config::load_config(Some("config"));

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: {:?}", env!("CARGO_PKG_NAME"), e);
        Err(e)
    } else {
        Ok(())
    }
}
