use clap::Parser as _;

#[derive(Debug, Clone, clap::Parser)]
#[command(version, about)]
pub struct Args {
    /// Number of pipes.
    #[arg(short, long, default_value = "8")]
    pub number: u32,
    /// Ticks per second.
    #[arg(short, long, default_value = "60")]
    pub tickrate: u32,
    /// Reset after amount of ticks. Set to 0 to never reset.
    #[arg(short, long, value_name = "TICKS", default_value = "600")]
    pub reset: u32,
    /// Probability of a straight pipe fitting.
    #[arg(
        short,
        long = "straight-probability",
        value_name = "PROBABILITY",
        default_value = "0.9"
    )]
    pub straight_prob: f64,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}
