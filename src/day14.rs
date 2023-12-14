use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use Rock::*;

pub fn day14(input: &str) -> (isize, isize) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    // Parse rocks
    // rocks[col, row] = rock
    let mut rocks = BTreeMap::new();
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


    rocks = tilt(&rocks, cols, rows);
    let result1 = get_load(&rocks, rows);

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
    fn new(rocks: BTreeMap<(usize, usize), Rock>, cols: usize, rows: usize) -> Self {
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
                    dbg!(i);
                    break;
                }
            }
        }
        // Calculate a large number of times
        let mut buffer = Vec::new();
        for _ in 0..100 {
            rocks = cycle(rocks, cols, rows);
            buffer.push(get_load(&rocks, rows));
        }
        // Try to find the pattern
        for window_size in 1.. {
            if window_size > 50 { panic!("Could not find") }
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

fn get_load(rocks: &BTreeMap<(usize, usize), Rock>, rows: usize) -> usize {
    rocks.iter().filter_map(|((j, i), &rock)| {
        if rock == Round { Some(rows - i) } else { None }
    }).sum()
}

fn cycle(rocks: BTreeMap<(usize, usize), Rock>, cols: usize, rows: usize) -> BTreeMap<(usize, usize), Rock> {
    let mut rocks = rocks;

    rocks = tilt(&rocks, cols, rows);
    rocks = rotate(&rocks, cols);
    rocks = tilt(&rocks, cols, rows);
    rocks = rotate(&rocks, rows);
    rocks = tilt(&rocks, cols, rows);
    rocks = rotate(&rocks, cols);
    rocks = tilt(&rocks, cols, rows);
    rocks = rotate(&rocks, rows);
    rocks
}

fn tilt(rocks: &BTreeMap<(usize, usize), Rock>, width: usize, height: usize) -> BTreeMap<(usize, usize), Rock> {
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

fn rotate(map: &BTreeMap<(usize, usize), Rock>, cols: usize) -> BTreeMap<(usize, usize), Rock> {
    map.iter().map(|(&(i, j), &value)| {
        ((cols - 1 - j, i), value)
    }).collect()
}


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Rock {
    Round,
    Cube,
}

