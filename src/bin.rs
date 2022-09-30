mod cli;
mod io;

use std::io::BufRead;
use mini_prop_lib::operators::Proposition;
use mini_prop_lib::stream::TokenStream;
use clap::Parser;

use cli::{Args, Commands};
use io::read_file_line_by_line;

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
        Commands::Demorg => input.into_iter().map(|prop| prop.demorg().into()).collect::<Vec<String>>(),
        Commands::Normal => input.into_iter().map(|prop| prop.demorg().normal().into()).collect::<Vec<String>>(),
        Commands::Simplify => unimplemented!(), 
        Commands::Analyze => unimplemented!(),
    };

    if let Some(filepath) = args.output {
        output.into_iter().for_each(|out| io::write_to_file(out, filepath.as_str()));
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
    use mini_prop_lib::{stream, operators}; 

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
            prop = prop.demorg();

           println!("{:?}", prop);
        }
    }
}
