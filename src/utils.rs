use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

/// Returns the lines of [filename]
pub fn get_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}