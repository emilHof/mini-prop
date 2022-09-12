use std::fs;
use mini_prop_lib::{
    stream,
    operators,
};

fn read_as_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
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

    #[test]
    fn test_read_and_parse() {
        let raw = read_as_string("test.txt").unwrap();
        let stream: stream::TokenStream = raw.try_into().ok().unwrap();
        let prop: operators::Proposition = stream.try_into().ok().unwrap();
        println!("{:?}", prop);
    }
}
