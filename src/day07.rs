use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::Chars;

pub fn day07(input: &str) -> (i32, i32) {
    // Parse hands
    let mut hands: Vec<(Chars, u32)> = input.lines().map(|line| {
        let [hand, bid] = line.split_whitespace().collect::<Vec<&str>>()[..] else { panic!("Parse error") };
        (hand.chars(), bid.parse().unwrap())
    }).collect();

    let result1 = calculate_winnings(&mut hands, false);
    let result2 = calculate_winnings(&mut hands, true);

    (result1 as i32, result2 as i32)
}

fn calculate_winnings(hands: &mut [(Chars, u32)], part_two: bool) -> usize {
    // Sort hands
    hands.sort_by(|(hand_a, _), (hand_b, _)| cmp_hand(hand_a, hand_b, part_two));

    // Loop through sorted hands (from worst to best) and calculate the rank
    hands.iter().enumerate().map(|(i, &(_, bid))| {
        (i + 1) * bid as usize
    }).sum()
}

const CARD_ORDER: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARD_ORDER2: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

fn cmp_hand(hand1: &Chars, hand2: &Chars, part_two: bool) -> Ordering {
    match rank_hand(hand1, part_two).cmp(&rank_hand(hand2, part_two)) {
        // If equal, compare the highest cards
        Ordering::Equal => cmp_draw(hand1, hand2, part_two),
        // If greater or smaller, just return
        ordering => ordering,
    }
}

fn rank_hand(hand: &Chars, part_two: bool) -> u32 {
    // Count the cards
    let cards_counted =
        hand.clone().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });
    // Get the maximum number of one card in this hand
    let max_count = cards_counted.iter()
        // If this is part 2, ignore the joker in this
        .filter(|(&card, _)| !part_two || card != 'J')
        .map(|(_, &count)| count)
        .max().unwrap_or(0);
    let mut max_count = max_count;
    // Get the number of unique cards
    let mut unique_cards_count = cards_counted.len();
    // Get the amount of jokers
    let joker_count = *cards_counted.get(&'J').unwrap_or(&0);

    // Use joker as highest card
    if part_two && joker_count > 0 {
        max_count += joker_count;
        unique_cards_count -= 1;
    }

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

fn cmp_draw(hand1: &Chars, hand2: &Chars, part_two: bool) -> Ordering {
    let order = if part_two { CARD_ORDER2 } else { CARD_ORDER };
    // Map cards to a number (lower = better)
    let hand1 = hand1.clone().map(|card| order.iter().position(|&x| x == card));
    let hand2 = hand2.clone().map(|card| order.iter().position(|&x| x == card));

    // Since CARD_ORDER will give better cards a _lower_ number,
    // we compare 2 to 1 here but it will give the result as if we compared 1 to 2 (since lower numbers are better)
    hand2.cmp(hand1)
}