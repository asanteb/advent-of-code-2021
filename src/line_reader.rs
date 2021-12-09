use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};


pub fn vec_from_files(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File does not exist");
    let buffer = BufReader::new(file);

    buffer.lines()
        .map(|line| line.expect("Could not parse"))
        .collect()
}