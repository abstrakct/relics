// cli.rs - parse CLI arguments
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
pub struct CliArgs {
    /// Set seed for random number generator
    #[clap(short, long, default_value_t = 0)]
    pub seed: u64,
}
