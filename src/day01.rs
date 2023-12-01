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
        let mut first_digit = 0;
        let mut last_digit = 0;
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
            if input.ends_with(word.0) {
                return (Some(word.1), &input[..(input.len() - word.0.len())]);
            }
        } else {
            if input.starts_with(word.0) {
                return (Some(word.1), &input[word.0.len()..]);
            }
        };
    }

    let chars: Vec<char> = input.chars().collect();
    if end {
        let last_char = chars.last().unwrap();
        if last_char.is_ascii_digit() {
            return (Some(last_char.to_digit(10).unwrap()), &input[..(input.len() - 1)]);
        }
    }
    else {
        let first_char = chars.first().unwrap();
        if first_char.is_ascii_digit() {
            return (Some(first_char.to_digit(10).unwrap()), &input[1..]);
        }
    }

    if end {
        (None, &input[..(input.len() - 1)])
    } else {
        (None, &input[1..])
    }
}