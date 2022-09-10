use std::{fs, str::Chars};

fn read_as_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

fn main() {
    
}
