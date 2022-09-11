use std::{fs, str::Chars};

fn read_as_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

fn main() {
    let formula = read_as_string("test.txt").unwrap();
    println!("{}", formula);
    let stream: mini_prop_lib::stream::TokenStream = formula.try_into().ok().unwrap();
    println!("{:?}", stream);
}
