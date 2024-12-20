use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input() -> Vec<String> {
    let file = File::open("./resources/input.txt").expect("File not found");

    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

fn main() {}