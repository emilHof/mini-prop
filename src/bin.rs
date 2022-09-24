use std::io::BufRead;
use clap::{Parser, Subcommand};
use mini_prop_lib::operators::Proposition;
use mini_prop_lib::procs::demorg;
use mini_prop_lib::stream::TokenStream;

fn read_file_line_by_line(filepath: &str) -> Result<std::io::BufReader<std::fs::File>, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, action)]
    file: bool,
    #[clap(value_parser)]
    input: String,
    #[clap(short, long, value_parser)]
    output: Option<String>,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Demorg,
    Analyze,
    Simplify,
}

fn run(args: Args) {
    let input = if args.file {
        read_file_line_by_line(&args.input).expect("a valid file path").lines().into_iter().map(|line| {
            TryInto::<Proposition>::try_into(
                TryInto::<TokenStream>::try_into(
                    line.expect("the input to be of parseable form")
                ).expect("propositions to be of valid form")
            ).expect("proposition to be of proper form")
        }).collect::<Vec<Proposition>>()
    } else {
        vec![
            TryInto::<Proposition>::try_into(
                TryInto::<TokenStream>::try_into(args.input).expect("proposition to be of valid form")
            ).expect("proposition to be of proper form")
        ]
    };

    let output = match args.command {
        Commands::Demorg => input.into_iter().map(|prop| demorg(prop).into()).collect::<Vec<String>>(),
        Commands::Analyze => unimplemented!(),
        Commands::Simplify => unimplemented!(),
    };

    if let Some(location) = args.output {
        unimplemented!();
    } else {
        output.into_iter().for_each(|out| println!("{}", out));
    }
}

fn main() {
    let args = Args::parse();
    run(args);
}

#[cfg(test)]
mod test_bin {
    use super::*;
    use mini_prop_lib::{stream, operators, procs::demorg};

    #[test]
    fn test_read_and_parse() {
        use std::io::prelude::*;

        let input = read_file_line_by_line("test_cases.txt").unwrap();

        for line in input.lines() {
            let line = line.ok().unwrap();
            println!("{}", line);
            let stream: stream::TokenStream = line.try_into().ok().unwrap();
            println!("{:?}", stream);
            let mut prop: operators::Proposition = stream.try_into().ok().unwrap();
            prop = demorg(prop);

           println!("{:?}", prop);
        }
    }
}
