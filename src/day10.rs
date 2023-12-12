use std::collections::HashMap;
use PipeType::*;
use Pipe::*;

pub fn day10(input: &str) -> (isize, isize) {
    // Parse grid
    let mut pipes = HashMap::new();
    let mut pipe_types = HashMap::new();
    let mut start = (-1, -1);
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pipe = parse_pipe(char);
            let coords = (y as i32, x as i32);
            if let StartPipe = pipe {
                start = coords;
            }
            pipes.insert(coords, pipe);
            pipe_types.insert(coords, Unknown);
        }
    }

    // Get possible neighbours from starting pipe
    let possible_neighbours = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut first_neighbour = None;
    for &option in &possible_neighbours {
        let possible_neighbour = get_absolute(option, start);
        let relative_coords = get_relative(possible_neighbour, start);
        if let Some(PipePart(_, [nb1, nb2])) = pipes.get(&possible_neighbour) {
            if *nb1 == relative_coords || *nb2 == relative_coords {
                // Found a neighbour pipe that can connect to the start!
                // (there should be two)
                first_neighbour = Some(possible_neighbour);
                break;
            }
        }
    }

    // Part 1
    // Count the loop length
    let mut prev_absolute = start;
    let mut current_absolute = first_neighbour.unwrap();
    let mut distance = 1;
    loop {
        // Set pipe as part of main loop
        pipe_types.insert(current_absolute, Main);
        // Get pipe information
        let pipe = pipes.get(&current_absolute).unwrap();
        let prev_relative = get_relative(current_absolute, prev_absolute);
        // Get next neighbour (the one that isn't the previous)
        let &next = match pipe {
            Ground => panic!("Reached ground"),
            PipePart(_, [nb1, nb2]) => {
                if *nb1 == prev_relative { nb2 } else if *nb2 == prev_relative { nb1 } else { panic!("Neighbour is not a neighbour") }
            }
            StartPipe => {
                // Detect direction of start pipe (for part 2)
                let nb1 = get_relative(start, first_neighbour.unwrap());
                let nb2 = get_relative(start, prev_absolute);
                pipes.insert(current_absolute, get_type([nb1, nb2]));
                break;
            }
        };
        // Go to next
        prev_absolute = current_absolute;
        current_absolute = get_absolute(next, current_absolute);
        distance += 1;
    }

    let result1 = distance / 2;

    // Part 2
    // Loop through every row from left to right
    for i in 0..height {
        let mut pipes_to_left = 0;
        let mut pipe_pieces = Vec::new();
        for j in 0..width {
            let pipe = pipes.get(&(i, j)).unwrap();
            let pipe_type = pipe_types.get(&(i, j)).unwrap();

            if *pipe_type == Main {
                // If we encounter a main pipe, we flip our status
                // (but not if it is part of a FJ or L7 edge case)
                match pipe {
                    PipePart(c, _) => {
                        match c {
                            '-' => {}
                            &x @ ('L' | 'F' | '7' | 'J') => {
                                pipe_pieces.push(x);
                                let last_corners = pipe_pieces.iter().rev().take(2).rev().collect::<String>();
                                match last_corners.as_str() {
                                    "FJ" => {}
                                    "LJ" => { pipes_to_left += 1; }
                                    "L7" => {}
                                    "F7" => { pipes_to_left += 1; }
                                    &_ => { pipes_to_left += 1; }
                                };
                            }
                            '|' => {
                                pipe_pieces.push('|');
                                pipes_to_left += 1;
                            }
                            &_ => {
                                panic!("Unknown pipe")
                            }
                        }
                    }
                    StartPipe => {
                        panic!("Start pipe should be replaced by now")
                    }
                    Ground => {}
                }
            } else {
                let location = if pipes_to_left % 2 == 0 {
                    Outside
                } else {
                    Inside
                };

                pipe_types.insert((i, j), location);
            }
        }
    }

    let inside: HashMap<_, _> = pipe_types.iter().filter(|&(_, p)| *p == Inside).collect();
    // dbg!(&inside.keys());
    let result2 = inside.len();

    (result1, result2 as isize)
}

fn parse_pipe(pipe: char) -> Pipe {
    match pipe {
        '|' => PipePart('|', [(-1, 0), (1, 0)]),
        '-' => PipePart('-', [(0, -1), (0, 1)]),
        'L' => PipePart('L', [(-1, 0), (0, 1)]),
        'J' => PipePart('J', [(-1, 0), (0, -1)]),
        '7' => PipePart('7', [(0, -1), (1, 0)]),
        'F' => PipePart('F', [(0, 1), (1, 0)]),
        'S' => StartPipe,
        '.' => Ground,
        _ => panic!("Parse error"),
    }
}

fn get_type(mut neighbours: [(i32, i32); 2]) -> Pipe {
    neighbours.sort();
    let char = match neighbours {
        [(-1, 0), (1, 0)] => '|',
        [(0, -1), (0, 1)] => '-',
        [(-1, 0), (0, 1)] => 'L',
        [(-1, 0), (0, -1)] => 'J',
        [(0, -1), (1, 0)] => '7',
        [(0, 1), (1, 0)] => 'F',
        _ => {panic!("Invalid pipe")}
    };
    PipePart(char, neighbours)
}


fn get_absolute(relative: (i32, i32), coords: (i32, i32)) -> (i32, i32) {
    (coords.0 + relative.0, coords.1 + relative.1)
}

fn get_relative(from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
    (to.0 - from.0, to.1 - from.1)
}

#[derive(Debug, Eq, PartialEq)]
enum Pipe {
    /// No neighbours.
    Ground,
    /// Two neighbours.
    PipePart(char, [(i32, i32); 2]),
    /// Starting pipe, all neighbours
    StartPipe,
}

#[derive(Debug, Eq, PartialEq)]
enum PipeType {
    /// Part of the main loop
    Main,
    /// Outside of the main loop
    Outside,
    /// Inside the main loop
    Inside,
    /// Not known if it is enclosed or outside of the main loop yet
    Unknown,
}


