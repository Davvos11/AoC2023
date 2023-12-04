use std::collections::HashSet;
use std::fs::read_to_string;

pub fn day04() {
    let input = read_to_string("static/input04.txt").expect("Cannot read input file");
    let input = input.trim();

    let mut result1 = 0;
    let mut copies = vec![1_usize; input.lines().count()];

    for (i, line)  in input.lines().enumerate() {
        let mut numbers = line.split(|c| c == '|' || c == ':');
        numbers.next(); // Ignore card number (use enumerate value instead)

        let winning_nums: HashSet<usize> = string_to_int_set(numbers.next().unwrap())
            .collect();
        let nums = string_to_int_set(numbers.next().unwrap());

        let wins = nums.filter(|x| winning_nums.contains(x)).count();
        if wins > 0 {
            result1 += 2_usize.pow((wins - 1) as u32);
        }

        // Get the amount of copies that the current card has
        let current_copies = copies[i];

        // Loop through the cards below this one (for the amount of wins of this card)
        copies[i+1..=i+wins].iter_mut().for_each(|card| *card += current_copies);
    }

    let result2: usize = copies.iter().sum();

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}

fn string_to_int_set(string: &str) -> impl Iterator<Item = usize> + '_ {
    string.split_whitespace().filter_map(|s|s.parse().ok())
}