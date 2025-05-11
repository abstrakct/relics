// cli.rs - parse CLI arguments
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
pub struct CliArgs {
    /// Set seed for random number generator
    #[clap(short, long, default_value_t = 0)]
    pub seed: u64,
    /// Perform various statistics calculations / tests
    #[clap(short = 't', long, default_value_t = false)]
    pub stats: bool,
}
