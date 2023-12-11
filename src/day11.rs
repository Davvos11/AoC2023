use std::collections::HashSet;

pub fn day11(input: &str) -> (isize, isize) {
    let mut result1 = 0;
    let mut result2 = 0;

    // Get rows anc columns that don't contain galaxies
    let empty_rows: HashSet<_> =
        input.lines().enumerate()
            .filter_map(|(i, l)| if !l.contains('#') { Some(i) } else { None })
            .collect();
    let width = input.lines().next().unwrap().len();
    let empty_cols: HashSet<_> =
        (0..width).filter(|&i| {
            let col: Vec<_> = input.lines()
                .map(|l| l.chars().collect::<Vec<_>>()[i]).collect();
            !col.contains(&'#')
        }).collect();

    // Get galaxy coordinates
    let mut galaxies1 = Vec::new();
    let mut galaxies2 = Vec::new();
    let mut i_offset = 0;
    for (i, line) in input.lines().enumerate() {
        if empty_rows.contains(&i) { i_offset += 1 }
        let mut j_offset = 0;

        for (j, char) in line.chars().enumerate() {
            if empty_cols.contains(&j) { j_offset += 1 }

            if char == '#' {
                galaxies1.push((i + i_offset, j + j_offset));
                galaxies2.push((i + i_offset * 999_999, j + j_offset * 999_999));
            }
        }
    }

    let galaxies = galaxies1.iter().zip(&galaxies2);

    // Calculate distances
    for (i, galaxy1) in galaxies.clone().enumerate() {
        for (j, galaxy2) in galaxies.clone().enumerate() {
            if i >= j { continue }
            result1 += manhattan_distance(*galaxy1.0, *galaxy2.0);
            result2 += manhattan_distance(*galaxy1.1, *galaxy2.1);
        }
    }

    (result1 as isize, result2 as isize)
}

fn manhattan_distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    (x2 as isize - x1 as isize).unsigned_abs() + (y2 as isize - y1 as isize).unsigned_abs()
}