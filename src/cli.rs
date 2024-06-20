use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub folder: PathBuf,

    #[arg(short, long, default_value = "10")]
    pub max_depth: usize,

    #[arg(short, long)]
    pub report: bool,
}