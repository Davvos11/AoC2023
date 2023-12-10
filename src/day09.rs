use crate::utils::parse_spaced_string;

pub fn day09(input: &str) -> (isize, isize) {
    let mut result1 = 0;
    let result2 = 0;

    for line in input.lines() {
        // Construct stack of differences between each layer
        let mut stack = Vec::new();
        let values: Vec<i32> = parse_spaced_string(line).collect();
        stack.push(values);
        while let Some(list) = stack.last() {
            let mut iter = list.iter();
            let mut new = Vec::new();
            let mut prev_value = iter.next().unwrap();
            for value in iter {
                new.push(value - prev_value);
                prev_value = value;
            }
            let all_zero = new.iter().all(|&x| x == 0);
            stack.push(new);
            if all_zero {
                break
            }
        }

        // Extrapolate last value for each layer
        let mut previous_list = stack.pop().unwrap();
        while let Some(mut list) = stack.pop() {
            let new_value = list.last().unwrap() + previous_list.last().unwrap();
            list.push(new_value);
            previous_list = list;
        }

        // Get new extrapolated value
        result1 += previous_list.last().unwrap();
    }

    (result1 as isize, result2)
}