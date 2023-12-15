pub fn day15(input: &str) -> (isize, isize) {
    let result1: u32 = input.trim().split(',')
        .map(|s|
            s.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
        ).sum();
    let result2 = 0;

    (result1 as isize, result2)
}