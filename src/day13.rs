pub fn day13(input: &str) -> (isize, isize) {
    let mut result1 = 0;
    let mut result2 = 0;

    for pattern in input.split("\n\n") {
        let rows = pattern.lines().count();
        let cols = pattern.lines().next().unwrap().len();

        // Try horizontal
        let chars: Vec<_> = pattern.lines().map(|l| l.chars().collect()).collect();
        let h1 = find_reflection(chars.clone(), false);
        let h2 = find_reflection(chars.clone(), true);
        // Try vertical
        let chars: Vec<_> = (0..cols)
            .map(|i| (0..rows).map(|j| chars[j][i]).collect())
            .collect();
        let v1 = find_reflection(chars.clone(), false);
        let v2 = find_reflection(chars, true);

        result1 += h1 * 100 + v1;
        result2 += h2 * 100 + v2;
    }

    (result1 as isize, result2 as isize)
}

fn find_reflection(rows: Vec<Vec<char>>, use_smudge: bool) -> usize {
    for i in 0..(rows.len()) {
        let mut smudge_used = !use_smudge;

        // Check if the pattern is reflective in row i
        let mut reflective = false;
        for j in 0..=i {
            if let Some(row_1) = rows.get(i - j) {
                if let Some(row_2) = rows.get(i + j + 1) {
                    let differences = count_differences(row_1, row_2);
                    reflective = differences == 0;
                    if !smudge_used {
                        reflective = differences <= 1;
                        smudge_used = differences == 1;
                    };
                    if !reflective {
                        break;
                    }
                } else { break; }
            } else { break; }
        }
        // If it was, return the row index (+1 cuz puzzle starts at 1)
        if reflective && smudge_used {
            return i + 1;
        }
    }

    0
}

fn count_differences(row_1: &[char], row_2: &[char]) -> usize {
    row_1.iter().zip(row_2)
        .filter(|(c1, c2)| c1 != c2)
        .count()
}