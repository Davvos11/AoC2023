use std::collections::HashMap;
use std::fs::read_to_string;

pub fn day03() {
    let input = read_to_string("static/input03.txt").expect("Cannot read input file");
    // numbers[y][x] = (number, start_x, end_x)
    let mut numbers: HashMap<usize, HashMap<usize, (usize, usize, usize)>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        // Pad line so that digits at the end of a line also get processed
        let line = format!("{line}.");

        let mut digits_buffer = String::new();
        for (x, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                // Add to digits buffer
                digits_buffer.push(char);
            } else {
                // Save number for each character
                let start_x = x - digits_buffer.len();
                for i in start_x..x {
                    let digits = digits_buffer.parse().unwrap();
                    let numbers_y = numbers.entry(y).or_default();
                    numbers_y.insert(i, (digits, start_x, x));
                }
                // Reset buffer
                digits_buffer = String::new();
            }
        }
    }

    let mut result = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                // Check for adjacent numbers
                for i in (y - 1)..=(y + 1) {
                    if let Some(numbers_y) = numbers.get_mut(&i) {
                        for j in (x - 1)..=(x + 1) {
                            if let Some(number) = numbers_y.get(&j) {
                                // Save result
                                result += number.0;
                                // Remove entry so it does not get summed multiple times
                                for k in number.1..number.2 {
                                    numbers_y.remove(&k);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {result}");
}
