use std::fs::File;
use std::io::{BufRead, BufReader};

static INPUT_PATH: &str = "./resources/input.txt";

pub fn read_input() -> Vec<String> {
    read_input_path(INPUT_PATH.parse().unwrap())
}

pub fn read_input_path(path: String) -> Vec<String> {
    let file = File::open(path).expect("File not found");

    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}