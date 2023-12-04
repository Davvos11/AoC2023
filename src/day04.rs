use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn day04() {
    let input = read_to_string("static/input04.txt").expect("Cannot read input file");

    let mut result1 = 0;
    let mut copies = HashMap::new();

    for line  in input.lines() {
        let parts: Vec<&str> = line.split(|c| c == ':' || c == '|')
            .map(|s|s.trim())
            .collect();
        let card_number: usize = get_card_num(parts[0]);
        let winning_nums= string_to_int_set(parts[1]);
        let nums = string_to_int_set(parts[2]);

        let wins = winning_nums.intersection(&nums).count();
        if wins > 0 {
            result1 += 2_usize.pow((wins - 1) as u32);
        }

        // Get the amount of copies that the current card has
        let current_copies = *copies.entry(card_number).or_insert(1);

        // Loop through the cards below this one (for the amount of wins of this card)
        for card_to_copy in card_number+1..=card_number+wins {
            // Add 1 to the cards for every copy of this card
            let count = copies.entry(card_to_copy).or_insert(1);
            *count += current_copies;
        }
    }

    let result2: usize = copies.values().sum();

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}

fn string_to_int_set(string: &str) -> HashSet<usize> {
    string.split_whitespace().map(|s|s.parse().unwrap()).collect()
}

fn get_card_num(string: &str) -> usize {
    string.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>().parse().unwrap()
}