pub fn read_file_line_by_line(filepath: &str) -> Result<std::io::BufReader<std::fs::File>, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(reader)
}
