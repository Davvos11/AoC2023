pub fn day12(input: &str) -> (isize, isize) {
    let mut result1 = 0;
    let result2 = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let chars: Vec<_> = split.next().unwrap().chars().collect();
        let groups: Vec<usize> = split.next().unwrap().split(',')
            .map(|c|c.parse().unwrap()).collect();

        let possible = count_possible(chars, &groups);

        result1 += possible;
    }

    (result1 as isize, result2)
}

fn count_possible(chars: Vec<char>, groups: &Vec<usize>) -> u32 {
    let mut count = 0;
    for (i, &char) in chars.iter().enumerate() {
        if char == '?' {
            let mut new_chars = chars.clone();
            new_chars[i] = '.';
            count += count_possible(new_chars.clone(), groups);
            new_chars[i] = '#';
            count += count_possible(new_chars, groups);
            return count;
        }
    }

    let actual_groups: Vec<_> =
        chars.split(|&c| c == '.')
            .filter_map(|g| if !g.is_empty() { Some(g.len()) } else { None })
            .collect();
    if actual_groups == *groups { 1 } else { 0 }
}