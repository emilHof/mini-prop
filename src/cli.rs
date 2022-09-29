use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub(super) struct Args {
    #[clap(short, long, action)]
    pub file: bool,
    #[clap(value_parser)]
    pub input: String,
    #[clap(short, long, value_parser)]
    pub output: Option<String>,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(super) enum Commands {
    Demorg,
    Normal,
    Analyze,
    Simplify,
}
