pub fn day13(input: &str) -> (isize, isize) {
    let mut result1 = 0;
    let result2 = 0;

    for pattern in input.split("\n\n") {
        let rows = pattern.lines().count();
        let cols = pattern.lines().next().unwrap().len();

        // Try horizontal
        let chars: Vec<_> = pattern.lines().map(|l|l.chars().collect()).collect();
        let h = find_reflection(chars.clone());
        // Try vertical
        let chars = (0..cols)
            .map(|i| (0..rows).map(|j| chars[j][i]).collect())
            .collect();
        let v = find_reflection(chars);

        result1 += h * 100 + v;
    }

    (result1 as isize, result2)
}

fn find_reflection(rows: Vec<Vec<char>>) -> usize {
    for i in 0..(rows.len()) {
        // Check if the pattern is reflective in row i
        let mut reflective = false;
        for j in 0..=i {
            if let Some(row_1) = rows.get(i - j) {
                if let Some(row_2) = rows.get(i + j + 1) {
                    reflective = row_1 == row_2;
                    if !reflective {
                        break;
                    }
                } else { break; }
            } else { break; }
        }
        // If it was, return the row index (+1 cuz puzzle starts at 1)
        if reflective {
            return i + 1
        }
    }

    0
}