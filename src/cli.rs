use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, action, help = "Set path to an input .txt file")]
    pub file: bool,
    #[clap(short = 'F', long, action, help = "Flip the input stream")]
    pub flip: bool,
    #[clap(value_parser, help = "The proposition(s) in valid LaTex")]
    pub input: String,
    #[clap(short, long, value_parser, help = "Set path to an output .txt file")]
    pub output: Option<String>,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    //// Apply DeMorgan's law to the input
    #[clap(about = "Apply DeMorgan's law to the input")]
    Demorg,
    #[clap(about = "Convert the input to a normalized form")]
    Normal,
    #[clap(skip, about = "Analyze the input (True/False/Ambiguous)")]
    Analyze,
    #[clap(about = "Simplify the input (duplicates, tautoligies, contradictions)")]
    Simplify,
}
