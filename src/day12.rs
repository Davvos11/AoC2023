use std::collections::HashMap;

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

        let mut memo = Memo::new();
        let x = memo.count_possible(chars_1, &groups, 0, 0);
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

        let y = memo.count_possible(chars_2, &groups_2, 0, 0);
        result2 += y;
    }

    (result1 as isize, result2 as isize)
}

struct Memo<'a> {
    memo: HashMap<(Vec<char>, &'a [usize], usize), u64>,
}

impl<'a> Memo<'a> {
    fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    fn count_possible(&mut self, chars: Vec<char>, groups: &'a [usize], i: usize, current_group: usize) -> u64 {
        let chars_from = Vec::from(&chars[i..]);
        // Lookup branch
        if let Some(&result) = self.memo.get(&(chars_from.clone(), groups, current_group)) {
            return result;
        }

        // If not found in memo, calculate the result
        let calculate = || {
            let mut count = 0;
            let mut current_group = current_group;
            if let Some(&char) = chars.get(i) {
                match char {
                    '?' => {
                        let mut new_chars = chars.clone();
                        new_chars[i] = '#';
                        count += self.count_possible(new_chars.clone(), groups, i, current_group);
                        new_chars[i] = '.';
                        count += self.count_possible(new_chars, groups, i, current_group);
                        return count;
                    }
                    '.' => {
                        // If we just finished a group:
                        if i > 0 {
                            if let Some(&'#') = chars.get(i - 1) {
                                // Check if this group complies with the next needed group
                                return if current_group == *groups.first().unwrap() {
                                    // Found a matching group! Continue with the next groups
                                    self.count_possible(chars, &groups[1..], i + 1, 0)
                                } else {
                                    // Not possible, backtrack
                                    0
                                }
                            }
                        }
                    }
                    '#' => {
                        // Check if the group we are forming is not too big
                        current_group += 1;
                        if groups.is_empty() || current_group > *groups.first().unwrap() {
                            // Not possible, backtrack
                            return 0;
                        }
                    }
                    _ => { panic!("") }
                }
            } else {
                // End of row, check if the last group formed is big enough
                return if groups.len() > 1 {
                    // Not possible, backtrack
                    0
                } else if groups.is_empty() ||
                    groups.len() == 1 && current_group == *groups.first().unwrap() {
                    // Found a matching group or no group when not needed! Possible!
                    // println!("{:?}", chars);
                    1
                } else {
                    // Not possible, backtrack
                    0
                }
            }

            // Continue to next character
            self.count_possible(chars, groups, i + 1, current_group)
        };

        // Save and return result
        let result = calculate();
        self.memo.insert((chars_from, groups, current_group), result);
        result
    }
}