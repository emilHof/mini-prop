use std::fs;
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

fn main() {
    let formula = read_as_string("test.txt").unwrap();
    println!("{}", formula);
    let stream: stream::TokenStream = formula.try_into().ok().unwrap();
    println!("{:?}", stream);
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
            let prop: operators::Proposition = stream.try_into().ok().unwrap();

            println!("{:?}", prop);
        }
    }
}
