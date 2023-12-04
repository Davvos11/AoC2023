use std::collections::HashSet;
use std::fs::read_to_string;

pub fn day04() {
    let input = read_to_string("static/input04.txt").expect("Cannot read input file");

    let mut result1 = 0;

    for line  in input.lines() {
        let parts: Vec<&str> = line.split(|c| c == ':' || c == '|')
            .map(|s|s.trim())
            .collect();
        let winning_nums= string_to_int_set(parts[1]);
        let nums = string_to_int_set(parts[2]);

        let wins = winning_nums.intersection(&nums).count();
        if wins > 0 {
            result1 += (2 as usize).pow((wins - 1) as u32);
        }
    }

    println!("Part 1: {result1}")
}

fn string_to_int_set(string: &str) -> HashSet<usize> {
    string.split_whitespace().map(|s|s.parse().unwrap()).collect()
}