use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::day01::day01;
use crate::day02::day02;

mod day01;
mod day02;

fn main() {
    day02();
}

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).expect("Cannot open file");

    let mut result = String::new();
    file.read_to_string(&mut result).expect("Cannot read file");

    result
}
