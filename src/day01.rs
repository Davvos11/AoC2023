use crate::read_file;

pub fn day01() {
    let input = read_file("static/input01.txt");

    let mut answer = 0;

    // for line in input.lines() {
    //     let digits: Vec<u32> = line.chars()
    //         .filter(|c| c.is_ascii_digit())
    //         .map(|c| c.to_digit(10).unwrap())
    //         .collect();
    //
    //     answer += digits.first().unwrap() * 10 + digits.last().unwrap();
    // }
    //
    // println!("Part 1: {answer}");

    for mut line in input.lines() {
        let first_digit;
        let last_digit;
        loop {
            let parse = word_to_digit(line, false);
            if let Some(digit) = parse.0 {
                first_digit = digit;
                break;
            } else if parse.1.is_empty() {
                panic!("No digits found")
            }
            line = parse.1;
        }

        loop {
            let parse = word_to_digit(line, true);
            if let Some(digit) = parse.0 {
                last_digit = digit;
                break;
            } else if parse.1.is_empty() {
                panic!("No digits found")
            }
            line = parse.1;
        }

        answer += first_digit * 10 + last_digit;
    }

    println!("Part 2: {answer}")
}

const WORDS: [(&str, u32); 9] =
    [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

fn word_to_digit(input: &str, end: bool) -> (Option<u32>, &str) {
    if input.is_empty() { return (None, ""); }

    for word in WORDS {
        if end {
            if let Some(new_input) = input.strip_suffix(word.0) {
                return (Some(word.1), new_input);
            }
        } else if let Some(new_input) = input.strip_prefix(word.0) {
            return (Some(word.1), new_input);
        };
    }

    let chars: Vec<char> = input.chars().collect();
    if end {
        let last_char = chars.last().unwrap();
        if last_char.is_ascii_digit() {
            let suffix = last_char.to_string();
            return (Some(last_char.to_digit(10).unwrap()), input.strip_suffix(&suffix).unwrap());
        }
    } else {
        let first_char = chars.first().unwrap();
        if first_char.is_ascii_digit() {
            let prefix = first_char.to_string();
            return (Some(first_char.to_digit(10).unwrap()), input.strip_prefix(&prefix).unwrap());
        }
    }

    if end {
        (None, &input[..(input.len() - 1)])
    } else {
        (None, &input[1..])
    }
}