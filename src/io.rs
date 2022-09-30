pub fn read_file_line_by_line(filepath: &str) -> Result<std::io::BufReader<std::fs::File>, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

pub fn write_to_file(data: String, filepath: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(filepath).expect("Unable to open file");
    match data.len() < 80 {
        true => file.write_all(format!("{}\n", data).as_bytes()).expect("unable to write to file"),
        false =>  file.write_all(format!("{}\n", data).as_bytes()).expect("unable to write to file"),
    };
}
