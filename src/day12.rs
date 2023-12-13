pub fn day12(input: &str) -> (isize, isize) {
    let mut result1 = 0;
    let mut result2 = 0;

    for (i, line) in input.lines().enumerate() {
        let mut split = line.split_whitespace();
        let chars = split.next().unwrap().chars();
        // Part 1
        let chars_1: Vec<_> = chars.clone().collect();
        let groups: Vec<usize> = split.next().unwrap().split(',')
            .map(|c| c.parse().unwrap()).collect();

        let x = count_possible(chars_1, &groups, 0);
        // println!("1: {x}");
        result1 += x;

        // Part 2
        let chars_2 = (1..=5)
            .fold(Vec::new(), |mut acc, i| {
                acc.extend(chars.clone());
                if i < 5 { acc.push('?') };
                acc
            });
        let groups_2: Vec<_> = (0..5)
            .flat_map(|_| groups.clone()).collect();

        let y = count_possible(chars_2, &groups_2, 0);
        result2 += y;
    }

    (result1 as isize, result2 as isize)
}

fn count_possible(chars: Vec<char>, groups: &[usize], i: usize) -> u64 {
    let mut count = 0;
    if let Some(&char) = chars.get(i) {
        match char {
            '?' => {
                let mut new_chars = chars.clone();
                new_chars[i] = '#';
                count += count_possible(new_chars.clone(), groups, i);
                new_chars[i] = '.';
                count += count_possible(new_chars, groups, i);
                return count;
            }
            '.' => {
                // If we just finished a group:
                if i > 0 {
                    if let Some(&'#') = chars.get(i - 1) {
                        // Check if this group complies with the next needed group
                        let group_length = last_group_length(&chars[0..i]);
                        if group_length == *groups.first().unwrap() {
                            // Found a matching group! Continue with the next groups
                            return count_possible(chars, &groups[1..], i + 1);
                        } else {
                            // Not possible, backtrack
                            return 0;
                        }
                    }
                }
            }
            '#' => {
                // Check if the group we are forming is not too big
                let group_length = last_group_length(&chars[0..=i]);
                if groups.is_empty() || group_length > *groups.first().unwrap() {
                    // Not possible, backtrack
                    return 0;
                }
            }
            _ => {panic!("")}
        }
    } else {
        // End of row, check if the last group formed is big enough
        let group_length = last_group_length(&chars[0..i]);
        if groups.len() > 1 {
            // Not possible, backtrack
            return 0;
        } else if groups.is_empty() ||
            groups.len() == 1 && group_length == *groups.first().unwrap() {
            // Found a matching group or no group when not needed! Possible!
            // println!("{:?}", chars);
            return 1;
        } else {
            // Not possible, backtrack
            return 0;
        }
    }

    // Continue to next character
    return count_possible(chars, groups, i + 1);
}

fn last_group_length(chars: &[char]) -> usize {
    chars.iter().rev().take_while(|&c|*c=='#').count()
}