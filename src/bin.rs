mod cli;
mod io;
mod run;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    run::run(args);
}
