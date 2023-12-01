use crate::read_file;

pub fn day01() {
    let input = read_file("static/example01.txt");

    let mut answer = 0;

    for line in input.lines() {
        let digits: Vec<u32> = line.chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        answer += digits.first().unwrap() * 10 + digits.last().unwrap();
    }

    println!("Part 1: {answer}");
}