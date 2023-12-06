use crate::utils::parse_spaced_string;

pub fn day06(input: &str) -> (i32, i32) {
    let mut result1 = 1;

    // Part 1:
    let mut lines = input.lines();
    let times = parse_spaced_string(lines.next().unwrap());
    let records = parse_spaced_string(lines.next().unwrap());
    let races = times.zip(records);

    let result1: usize = races.map(|(t,r)|calculate_race(t,r)).product();

    // Part 2:
    let mut lines = input.lines();
    let time = get_number(lines.next().unwrap());
    let record = get_number(lines.next().unwrap());

    let result2 = calculate_race(time, record);

    (result1 as i32, result2 as i32)
}

fn calculate_race(time: u64, record: u64) -> usize {
    // distance = time_held * (time - time_held)
    (0..time).filter(|&time_held| {
        time_held * (time - time_held) > record
    }).count()
}

fn get_number(string: &str) -> u64 {
    string.chars().filter(|c|c.is_ascii_digit()).collect::<String>().parse().unwrap()
}