use std::collections::HashMap;
use Pipe::*;

pub fn day10(input: &str) -> (isize, isize) {
    // Parse grid
    let mut pipes = HashMap::new();
    let mut start = (-1, -1);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pipe = parse_pipe(char);
            if let StartPipe = pipe {
                start = (y as i32, x as i32);
            }
            pipes.insert((y as i32, x as i32), pipe);
        }
    }

    // Get possible neighbours from starting pipe
    let possible = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut first_neighbour = None;
    for option in possible {
        let possible_neighbour = get_absolute(option, start);
        let relative_coords = get_relative(possible_neighbour, start);
        if let Some(Pipe(nb1, nb2)) = pipes.get(&possible_neighbour) {
            if *nb1 == relative_coords || *nb2 == relative_coords {
                // Found a neighbour pipe that can connect to the start!
                // (there should be two)
                first_neighbour = Some(possible_neighbour);
                break
            }
        }
    }

    // Count the loop length
    let mut prev_absolute = start;
    let mut current_absolute = first_neighbour.unwrap();
    let mut distance = 1;
    loop {
        // println!("Node: {:?}, distance: {}", current_absolute, distance);
        let pipe = pipes.get(&current_absolute).unwrap();
        let prev_relative = get_relative(current_absolute, prev_absolute);
        // Get next neighbour (the one that isn't the previous)
        let &next = match pipe {
            Ground => panic!("Reached ground"),
            Pipe(nb1, nb2) => {
                if *nb1 == prev_relative { nb2 }
                else if *nb2 == prev_relative { nb1 }
                else { panic!("Neighbour is not a neighbour") }
            }
            StartPipe => {
                break
            }
        };
        // Go to next
        prev_absolute = current_absolute;
        current_absolute = get_absolute(next, current_absolute);
        distance += 1;
    }

    let result1 = distance / 2;
    let result2 = 0;

    (result1, result2)
}

fn parse_pipe(pipe: char) -> Pipe {
    match pipe {
        '|' => Pipe((-1, 0), (1, 0)),
        '-' => Pipe((0, -1), (0, 1)),
        'L' => Pipe((-1, 0), (0, 1)),
        'J' => Pipe((-1, 0), (0, -1)),
        '7' => Pipe((1, 0), (0, -1)),
        'F' => Pipe((1, 0), (0, 1)),
        'S' => StartPipe,
        '.' => Ground,
        _ => panic!("Parse error"),
    }
}

fn get_absolute(relative: (i32, i32), coords: (i32, i32)) -> (i32, i32) {
    (coords.0 + relative.0, coords.1 + relative.1)
}

fn get_relative(from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
    (to.0 - from.0, to.1 - from.1)
}


enum Pipe {
    /// No neighbours.
    Ground,
    /// Two neighbours.
    Pipe((i32, i32), (i32, i32)),
    /// Starting pipe, all neighbours
    StartPipe,
}

