use std::collections::HashMap;

pub fn day03(input: &str) -> (i32, i32) {
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

    let mut result1 = 0;
    let mut result2 = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                let mut gear_numbers = Vec::new();
                // Check for adjacent numbers
                for i in (y - 1)..=(y + 1) {
                    if let Some(numbers_y) = numbers.get_mut(&i) {
                        for j in (x - 1)..=(x + 1) {
                            if let Some(number) = numbers_y.get(&j) {
                                // Save result
                                result1 += number.0;
                                // If this is a gear, save to gear list
                                if char == '*' {
                                    gear_numbers.push(number.0);
                                }

                                // Remove entry so it does not get summed multiple times
                                for k in number.1..number.2 {
                                    numbers_y.remove(&k);
                                }
                            }
                        }
                    }
                }
                // If this is a gear with two part numbers, save result
                if gear_numbers.len() == 2 {
                    result2 += gear_numbers.iter().product::<usize>();
                }
            }
        }
    }

    (result1 as i32, result2 as i32)
}
