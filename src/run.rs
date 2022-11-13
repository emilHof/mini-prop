use prop_tune::operators::Proposition;
use prop_tune::stream::{flip_stream, TokenStream};
use std::io::BufRead;

use super::cli::{Args, Commands};
use super::io::{read_file_line_by_line, write_to_file};

pub fn run(args: Args) {
    let input = if args.file {
        read_file_line_by_line(&args.input)
            .expect("a valid file path")
            .lines()
            .into_iter()
            // parse the text into a TokenStream
            .map(|line| {
                TryInto::<TokenStream>::try_into(line.expect("the input to be of parseable form"))
                    .expect("propositions to be of valid form")
            })
            .collect()
    } else {
        vec![TryInto::<TokenStream>::try_into(args.input).expect("proposition to be of valid form")]
    }
    .into_iter()
    .map(|mut stream| {
        if args.flip {
            flip_stream(&mut stream);
        }
        stream
    })
    .map(|line| TryInto::<Proposition>::try_into(line).expect("proposition to be of proper form"))
    .collect::<Vec<Proposition>>();

    let output = match args.command {
        Commands::Demorg => input
            .into_iter()
            .map(|prop| prop.demorg().into())
            .collect::<Vec<String>>(),
        Commands::Normal => input
            .into_iter()
            .map(|prop| prop.normal().into())
            .collect::<Vec<String>>(),
        Commands::Simplify => input
            .into_iter()
            .map(|prop| prop.simplify().into())
            .collect::<Vec<String>>(),
        Commands::Analyze => unimplemented!(),
    };

    if let Some(filepath) = args.output {
        output
            .into_iter()
            .for_each(|out| write_to_file(out, filepath.as_str()));
    } else {
        output.into_iter().for_each(|out| println!("{}", out));
    }
}

#[cfg(test)]
mod test_bin {
    use super::*;
    use prop_tune::{operators, stream};

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
