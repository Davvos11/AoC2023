use std::collections::HashMap;
use indexmap::IndexMap;

pub fn day15(input: &str) -> (isize, isize) {
    let hash = |s: &str|
        s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256);
    let steps = input.trim().split(',');

    // Part 1:
    let result1: usize = steps.clone().map(hash).sum();

    // Part 2
    let mut boxes: HashMap<_, IndexMap<&str, _>> = HashMap::new();
    for step in steps {
        let mut split = step.split(|c| c == '-'|| c =='=');
        let label = split.next().unwrap();
        let box_id = hash(label);
        match step.strip_prefix(label).unwrap().chars().next().unwrap() {
            '-' => {
                if let Some(box_) = boxes.get_mut(&box_id) {
                    box_.shift_remove(&label);
                }
            },
            '=' => {
                let focal_len: usize = split.next().unwrap().parse().unwrap();
                let box_ = boxes.entry(box_id).or_default();
                let lens = box_.entry(label).or_default();
                *lens = focal_len;
            },
            _ => panic!("Expected - or ="),
        }
    }

    let result2: usize = boxes.iter().map(|(box_num,b)| {
        b.values().enumerate().map(|(lens_num, focal_len)|{
            (box_num + 1) * (lens_num + 1) * focal_len
        }).sum::<usize>()
    }).sum();


    (result1 as isize, result2 as isize)
}