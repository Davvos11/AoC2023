use std::fs::read_to_string;
use regex::Regex;

// (r,g,b)
const GAME: (i32, i32, i32) = (12, 13, 14);

pub fn day02() {
    let input = read_to_string("static/input02.txt").unwrap();
    let id_pattern = Regex::new(r"Game (\d+): ").unwrap();
    let cube_pattern = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut result = 0;

    for line in input.lines() {
        let capt = id_pattern.captures(line).unwrap();
        let id: i32 = capt.get(1).unwrap().as_str().parse().unwrap();
        let prefix = capt.get(0).unwrap().as_str();
        let line = line.strip_prefix(prefix).unwrap();

        let sets = line.split(',');
        let mut possble = true;

        'game: for set in sets {
            for cube in cube_pattern.captures_iter(set) {
                if let Some(colour) = cube.get(2) {
                    if let Some(amount) = cube.get(1) {
                        let amount: i32 = amount.as_str().parse().unwrap();
                        let allowed_amount = match colour.as_str() {
                            "red" => GAME.0,
                            "green" => GAME.1,
                            "blue" => GAME.2,
                            _ => { panic!("Colour not found") }
                        };
                        if amount > allowed_amount {
                            possble = false;
                            break 'game;
                        }
                    }
                }
            }
        }

        if possble {
            result += id;
        }
    }

    println!("Part 1: {result}")
}