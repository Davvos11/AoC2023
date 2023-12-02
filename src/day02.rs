use std::fs::read_to_string;
use regex::Regex;

// (r,g,b)
const GAME: [i32; 3] = [12, 13, 14];

pub fn day02() {
    let input = read_to_string("static/input02.txt").expect("Cannot read input file");
    let id_pattern = Regex::new(r"Game (\d+): ").unwrap();
    let cube_pattern = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut result_01 = 0;
    let mut result_02 = 0;

    for line in input.lines() {
        let capt = id_pattern.captures(line).unwrap();
        let id: i32 = capt.get(1).unwrap().as_str().parse().unwrap();
        let prefix = capt.get(0).unwrap().as_str();
        let line = line.strip_prefix(prefix).unwrap();

        let sets = line.split(',');
        let mut possble = true;
        let mut minimum = [0, 0, 0];

        for set in sets {
            for cube in cube_pattern.captures_iter(set) {
                if let Some(colour) = cube.get(2) {
                    if let Some(amount) = cube.get(1) {
                        let amount: i32 = amount.as_str().parse().unwrap();
                        let index = match colour.as_str() {
                            "red" => 0,
                            "green" => 1,
                            "blue" => 2,
                            _ => { panic!("Colour not found") }
                        };

                        // Part 1
                        if amount > GAME[index] {
                            possble = false;
                        }
                        // Part 2
                        if minimum[index] < amount {
                            minimum[index] = amount;
                        }
                    }
                }
            }
        }

        if possble {
            result_01 += id;
        }
        result_02 += minimum.iter().fold(1, |acc, &x| acc * x);
    }

    println!("Part 1: {result_01}\nPart 2: {result_02}");
}