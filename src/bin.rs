use std::fs;
use std::env;
use mini_prop_lib::operators::Proposition;
use mini_prop_lib::procs::demorg;
use mini_prop_lib::stream::TokenStream;
use mini_prop_lib::{
    stream,
    operators,
};

fn read_as_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

fn read_file_line_by_line(filepath: &str) -> Result<std::io::BufReader<std::fs::File>, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

#[derive(Debug)]
enum Config {
    CLI(String),
    File(String),
}

#[derive(Debug)]
struct ParseError;

impl Config {
    fn parse(args: Vec<String>) -> Result<Config, ParseError> {
        if args.len() < 2 {
            return Err(ParseError)
        }

        match args[1].as_str() {
            "file" => {
                if args.len() < 3 {
                    return Err(ParseError)
                } else {
                    return Ok(Config::File(args[2].clone()));
                }
            },
            s => return Ok(Config::CLI(s.to_string().clone())),
        }
    }
}

fn run(config: Config) {
    match config {
        Config::CLI(raw_prop) => {
            let parsed_prop: Proposition = TryInto::<TokenStream>::try_into(raw_prop).expect("malformed proposition").try_into().expect("invalid proposition");
            println!("{}", parsed_prop);
        },
        Config::File(path) => {
            use std::io::prelude::*;

            let input = read_file_line_by_line(&path).unwrap();

            for line in input.lines() {
                let line = line.ok().unwrap();
                let parsed_prop: Proposition = demorg(TryInto::<TokenStream>::try_into(line).expect("malformed proposition").try_into().expect("invalid proposition"));
                println!("{}", parsed_prop);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse(args).expect("invalid parameters");
    run(config)
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
