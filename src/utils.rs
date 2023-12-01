// file_reader.rs

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_file_line_by_line<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}
