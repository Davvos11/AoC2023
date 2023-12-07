use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::Chars;

pub fn day07(input: &str) -> (i32, i32) {
    let mut result1 = 0;
    let result2 = 0;

    // Parse hands
    let mut hands: Vec<(Chars, u32)> = input.lines().map(|line| {
        let [hand, bid] = line.split_whitespace().collect::<Vec<&str>>()[..] else { panic!("Parse error") };
        (hand.chars(), bid.parse().unwrap())
    }).collect();
    // Sort hands
    hands.sort_by(|(hand_a, _), (hand_b, _)| cmp_hand(hand_a, hand_b));

    // Loop through sorted hands (from worst to best) and calculate the rank
    for (i, &(_, bid)) in hands.iter().enumerate() {
        result1 += (i+1) * bid as usize;
    }

    // dbg!(hands);

    (result1 as i32, result2)
}

const CARD_ORDER: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

fn cmp_hand(hand1: &Chars, hand2: &Chars) -> Ordering {
    match rank_hand(hand1).cmp(&rank_hand(hand2)) {
        // If equal, compare the highest cards
        Ordering::Equal => cmp_draw(hand1, hand2),
        // If greater or smaller, just return
        ordering => ordering,
    }
}

fn rank_hand(hand: &Chars) -> u32 {
    // Count the cards
    let cards_counted =
        hand.clone().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });
    // Get the maximum number of one card in this hand
    let max_count = cards_counted.values().max().unwrap();
    // Get the number of unique cards
    let unique_cards_count = cards_counted.len();

    match max_count {
        5 => 7, // Five of a kind
        4 => 6, // Four of a kind
        3 => {
            if unique_cards_count == 2 { 5 } // Full house
            else { 4 } // Three of a kind
        }
        2 => {
            if unique_cards_count == 3 { 3 } // Two pair
            else { 2 } // One pair
        }
        1 => 1, // High card
        _ => panic!("Parse error")
    }
}

fn cmp_draw(hand1: &Chars, hand2: &Chars) -> Ordering {
    // Map cards to a number (lower = better)
    let hand1 = hand1.clone().map(|card| CARD_ORDER.iter().position(|&x| x == card));
    let hand2 = hand2.clone().map(|card| CARD_ORDER.iter().position(|&x| x == card));

    // Since CARD_ORDER will give better cards a _lower_ number,
    // we compare 2 to 1 here but it will give the result as if we compared 1 to 2 (since lower numbers are better)
    hand2.cmp(hand1)
}