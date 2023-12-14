use std::collections::HashMap;
use crate::day14::Rock::*;

pub fn day14(input: &str) -> (isize, isize) {
    let result2 = 0;

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    // Parse rocks
    // rocks[col][row] = rock
    let mut rocks = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            let rock = match char {
                'O' => Round,
                '#' => Cube,
                '.' => { continue; }
                _ => panic!("Parse error"),
            };
            rocks.entry(j).or_insert(HashMap::new()).insert(i, rock);
        }
    }

    // For each rock, move it as North-y as possible
    for c in 0..width {
        if let Some(col) = rocks.get_mut(&c) {
            let mut next_empty = 0;
            for r in 0..height {
                if let Some(&rock) = col.get(&r) {
                    match rock {
                        Round => {
                            if next_empty < r {
                                col.insert(next_empty, rock);
                                col.remove(&r);
                            }
                            if next_empty <= r {
                                next_empty += 1;
                            }
                        }
                        Cube => { next_empty = r + 1; }
                    }
                }
            }
        }
    }

    let result1: usize = rocks.values().map(|col| {
            col.iter().filter_map(|(i, &rock)| if rock == Round { Some(height-i) } else { None })
                .sum::<usize>()
        }).sum();

    (result1 as isize, result2)
}



#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Rock {
    Round,
    Cube,
}
