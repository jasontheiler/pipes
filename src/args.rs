use clap::Parser as _;
use crossterm::style;

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
    #[arg(short, long, value_name = "TICKS", default_value = "900")]
    pub reset: u32,
    /// Probability of a straight pipe fitting.
    #[arg(
        short,
        long = "straight-probability",
        value_name = "PROBABILITY",
        default_value = "0.9"
    )]
    pub straight_prob: f64,
    /// Colors of the pipes.
    #[arg(
        short,
        long,
        value_delimiter = ',',
        value_parser = parse_color,
        default_value = "red,green,yellow,blue,magenta,cyan,white,black",
    )]
    pub colors: Vec<style::Color>,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}

fn parse_color(s: &str) -> anyhow::Result<style::Color> {
    style::Color::try_from(s).map_err(|()| anyhow::format_err!("invalid color"))
}
