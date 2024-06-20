use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub folder: PathBuf,

    #[clap(short, long, default_value = "10")]
    pub max_depth: usize,
}