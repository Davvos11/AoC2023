
// (r,g,b)
const GAME: [usize; 3] = [12, 13, 14];

pub fn day02(input: &str) -> (isize, isize) {
    let mut result_01 = 0;
    let mut result_02 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(|c| c == ':' || c == ';')
            .map(|s| s.trim())
            .collect();
        let id = get_game_num(parts[0]);

        let sets = &parts[1..];
        let mut possble = true;
        let mut minimum = [0, 0, 0];

        for set in sets {
            for cube in set.split(',') {
                let (colour, amount) = get_colour_and_amount(cube);
                let index = match colour.as_str() {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => { panic!("Colour not found") }
                };

                // Part 1
                if amount > GAME[index] {
                    possble = false;
                }
                // Part 2
                if minimum[index] < amount {
                    minimum[index] = amount;
                }
            }
        }

        if possble {
            result_01 += id;
        }
        result_02 += minimum.iter().product::<usize>();
    }

    (result_01 as isize, result_02 as isize)
}

fn get_game_num(string: &str) -> usize {
    string.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>().parse().unwrap()
}

fn get_colour_and_amount(string: &str) -> (String, usize) {
    let (digits, non_digits): (String, String) = string.chars()
        .partition(|c| c.is_ascii_digit());

    let colour = non_digits.trim();
    let amount = digits.parse().unwrap();

    (colour.to_string(), amount)
}