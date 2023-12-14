use std::collections::{HashMap};
use std::hash::Hash;
use Rock::*;

pub fn day14(input: &str) -> (isize, isize) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    // Parse rocks
    // rocks[col, row] = rock
    let mut rocks = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            let rock = match char {
                'O' => Round,
                '#' => Cube,
                '.' => { continue; }
                _ => panic!("Parse error"),
            };
            rocks.insert((j, i), rock);
        }
    }


    // Part 1
    rocks = tilt(&rocks, cols, rows);
    let result1 = get_load(&rocks, rows);

    // Part 2
    let pattern = Pattern::new(rocks, cols, rows);
    let result2 = pattern.get(1000000000 - 1);

    (result1 as isize, result2 as isize)
}

struct Pattern {
    initial: Vec<usize>,
    initial_size: usize,
    pattern: Vec<usize>,
    pattern_size: usize,
}

impl Pattern {
    /// The result keeps (mostly) lowering until it reaches a pattern
    /// see: https://i.imgur.com/w0q3ztl.png
    /// This function will provide that pattern
    fn new(rocks: HashMap<(usize, usize), Rock>, cols: usize, rows: usize) -> Self {
        let mut initial = Vec::new();
        let mut pattern = Vec::new();
        let mut rocks = rocks;
        // Keep calculating if we are still going down
        for i in 0.. {
            rocks = cycle(rocks, cols, rows);
            initial.push(get_load(&rocks, rows));
            if i >= 5 && i % 5 == 0 {
                let slope = initial[i] as isize - initial[i-5] as isize;
                if -5 < slope {
                    break;
                }
            }
        }

        let mut buffer = Vec::new();
        // Try to find the pattern
        for window_size in 1.. {
            // Add new values
            for _ in 0..2 {
                rocks = cycle(rocks, cols, rows);
                buffer.push(get_load(&rocks, rows));
            }
            if window_size % 50 == 0 { println!("Warning: window size > {window_size}") }

            let mut windows = buffer.chunks_exact(window_size);
            if let Some(first) = windows.next() {
                let mut repeats = false;
                for window in windows {
                    repeats = first == window;
                    if !repeats {
                        break;
                    }
                }
                if repeats {
                    pattern = Vec::from(first);
                    break;
                }
            }
        }

        Self {
            initial_size: initial.len(),
            pattern_size: pattern.len(),
            initial,
            pattern,
        }
    }

    fn get(&self, i: usize) -> usize {
        if i < self.initial_size {
            self.initial[i]
        } else {
            self.pattern[(i - self.initial_size) % self.pattern_size]
        }
    }
}

fn get_load(rocks: &HashMap<(usize, usize), Rock>, rows: usize) -> usize {
    rocks.iter().filter_map(|((_, i), &rock)| {
        if rock == Round { Some(rows - i) } else { None }
    }).sum()
}

fn cycle(rocks: HashMap<(usize, usize), Rock>, mut cols: usize, mut rows: usize) -> HashMap<(usize, usize), Rock> {
    let mut rocks = rocks;
    for _ in 0..4 {
        rocks = tilt(&rocks, cols, rows);
        rocks = rotate(&rocks, cols);
        (cols, rows) = (rows, cols);
    }
    rocks
}

fn tilt(rocks: &HashMap<(usize, usize), Rock>, width: usize, height: usize) -> HashMap<(usize, usize), Rock> {
    let mut rocks = rocks.clone();

    for c in 0..width {
        let mut next_empty = 0;
        for r in 0..height {
            if let Some(&rock) = rocks.get(&(c, r)) {
                match rock {
                    Round => {
                        if next_empty < r {
                            rocks.insert((c, next_empty), rock);
                            rocks.remove(&(c, r));
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

    rocks
}

fn rotate(map: &HashMap<(usize, usize), Rock>, cols: usize) -> HashMap<(usize, usize), Rock> {
    map.iter().map(|(&(i, j), &value)| {
        ((cols - 1 - j, i), value)
    }).collect()
}


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Rock {
    Round,
    Cube,
}

