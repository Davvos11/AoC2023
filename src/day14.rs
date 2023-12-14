use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use Rock::*;

pub fn day14(input: &str) -> (isize, isize) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut memo = Memo::new();

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


    rocks = memo.tilt(&rocks, cols, rows);
    let result1 = get_load(&rocks, rows);

    // Cycle n times
    // let mut rs = Vec::new();
    for i in 0..1000000000 {
        rocks = memo.cycle(rocks, cols, rows);
    }

    // dbg!(rs);

    let result2 = get_load(&rocks, rows);

    (result1 as isize, result2 as isize)
}

fn get_load(rocks: &BTreeMap<(usize, usize), Rock>, rows: usize) -> usize {
    rocks.iter().filter_map(|((j, i), &rock)| {
        if rock == Round { Some(rows - i) } else { None }
    }).sum()
}

struct Memo {
    tilt_memo: HashMap<BTreeMap<(usize, usize), Rock>, BTreeMap<(usize, usize), Rock>>,
    cycle_memo: HashMap<BTreeMap<(usize, usize), Rock>, BTreeMap<(usize, usize), Rock>>,
    rotate_memo: HashMap<BTreeMap<(usize, usize), Rock>, BTreeMap<(usize, usize), Rock>>,
}

impl Memo {
    fn new() -> Self {
        Self {
            tilt_memo: HashMap::new(),
            cycle_memo: HashMap::new(),
            rotate_memo: HashMap::new(),
        }
    }

    fn cycle(&mut self, rocks: BTreeMap<(usize, usize), Rock>, cols: usize, rows: usize) -> BTreeMap<(usize, usize), Rock> {
        let rocks_before = rocks.clone();
        if let Some(result) = self.cycle_memo.get(&rocks_before) {
            return result.clone()
        }

        let mut rocks = rocks;

        rocks = self.tilt(&rocks, cols, rows);
        rocks = self.rotate(&rocks, cols);
        rocks = self.tilt(&rocks, cols, rows);
        rocks = self.rotate(&rocks, rows);
        rocks = self.tilt(&rocks, cols, rows);
        rocks = self.rotate(&rocks, cols);
        rocks = self.tilt(&rocks, cols, rows);
        rocks = self.rotate(&rocks, rows);

        self.cycle_memo.insert(rocks_before, rocks.clone());
        rocks
    }

    fn tilt(&mut self, rocks: &BTreeMap<(usize, usize), Rock>, width: usize, height: usize) -> BTreeMap<(usize, usize), Rock> {
        let rocks_before = rocks.clone();
        if let Some(result) = self.tilt_memo.get(&rocks_before) {
            return result.clone()
        }

        let mut rocks = rocks_before.clone();

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

        self.tilt_memo.insert(rocks_before, rocks.clone());
        rocks
    }

    fn rotate(&mut self, map: &BTreeMap<(usize, usize), Rock>, cols: usize) -> BTreeMap<(usize, usize), Rock> {
        if let Some(map) = self.rotate_memo.get(map) {
            return map.clone();
        }

        let result: BTreeMap<_, Rock> = map.iter().map(|(&(i, j), &value)| {
            ((cols - 1 - j, i), value)
        }).collect();

        self.rotate_memo.insert(map.clone(), result.clone());
        result
    }
}



#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Rock {
    Round,
    Cube,
}

