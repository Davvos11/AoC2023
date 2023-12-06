use crate::utils::parse_spaced_string;

pub fn day06(input: &str) -> (i32, i32) {
    let mut result1 = 1;

    // Part 1:
    let mut lines = input.lines();
    let times = parse_spaced_string(lines.next().unwrap());
    let distance = parse_spaced_string(lines.next().unwrap());
    let races: Vec<(u32, u32)> = times.zip(distance).collect();

    for (time, record) in races {
        // distance = time_held * (time - time_held)
        result1 *= (0..time).filter(|&time_held| {
            time_held * (time - time_held) > record
        }).count();
    }

    // Part 2:
    let mut lines = input.lines();
    let time = get_number(lines.next().unwrap());
    let record = get_number(lines.next().unwrap());

    let result2 = (0..time).filter(|&time_held| {
        time_held * (time - time_held) > record
    }).count();

    (result1 as i32, result2 as i32)
}

fn get_number(string: &str) -> u64 {
    string.chars().filter(|c|c.is_ascii_digit()).collect::<String>().parse().unwrap()
}