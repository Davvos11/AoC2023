use std::collections::HashSet;
use std::ops::Add;
use crate::utils::parse_spaced_string;

pub fn day05(input: &str) -> (i32, i32) {
    (part01(input) as i32, part02(input) as i32)
}

fn part01(input: &str) -> isize {
    let mut sections = input.split("\n\n");

    // Read seeds (= sources) from first line
    let mut sources: HashSet<isize> = parse_spaced_string(
        sections.next().unwrap().strip_prefix("seeds: ").unwrap()
    ).collect();

    for section in sections {
        let mut lines = section.lines();
        // Assuming that the category maps are given in order, so no need to do anything with the title
        let _title = lines.next().unwrap();

        let mut destinations = HashSet::new();

        for line in lines {
            let instructions: Vec<isize> = parse_spaced_string(line).collect();
            let [dest_start, source_start, len] = instructions[..] else { panic!("Could not parse") };

            let mut mapped_sources = HashSet::new();

            // If one of the ranges includes one of our sources, save the destination
            for source in sources.clone() {
                let difference = source - source_start;
                if difference > 0 && difference < len {
                    destinations.insert(dest_start + difference);
                    mapped_sources.insert(source);
                }
            }

            sources = sources.difference(&mapped_sources).cloned().collect();
        }

        // Add all sources that haven't been mapped to a new destination and
        // move the new set to sources set for next iteration
        sources = sources.union(&destinations).cloned().collect();
    }

    *sources.iter().min().unwrap()
}


fn part02(input: &str) -> isize {
    let mut sections = input.split("\n\n");

    // Read seeds (= sources) from first line
    let mut sources = parse_spaced_string::<isize>(
        sections.next().unwrap().strip_prefix("seeds: ").unwrap()
    );

    // Parse into tuples
    let mut source_ranges = HashSet::new();
    while let Some(first) = sources.next() {
        if let Some(second) = sources.next() {
            source_ranges.insert((first, second + first));
        }
    }

    for section in sections {
        let mut lines = section.lines();
        // Assuming that the category maps are given in order, so no need to do anything with the title
        let _title = lines.next().unwrap();

        let mut dest_ranges = HashSet::new();

        for line in lines {
            let instructions: Vec<isize> = parse_spaced_string(line).collect();
            let [dst_ins_start, src_ins_start, ins_len]
                = instructions[..] else { panic!("Could not parse") };

            let src_ins_range = start_len_to_range(src_ins_start, ins_len);

            for src_range in source_ranges.clone() {
                let int_diff = range_intersect_difference(src_range, src_ins_range);
                // If our source is (partly) in the range of the instructions:
                if let Some((intersection, differences)) = int_diff {
                    // Save mapped destination range
                    let shift = dst_ins_start - src_ins_start;
                    dest_ranges.insert(shift_range(intersection, shift));
                    // Remove from original list
                    source_ranges.remove(&src_range);
                    // Save differences (ranges outside the instruction range) (if any) to list
                    for difference in differences {
                        source_ranges.insert(difference);
                    }
                }
            }
        }

        // Move to next iteration
        source_ranges = source_ranges.union(&dest_ranges).cloned().collect();
    }

    // Get range with lowest start value
    *source_ranges.iter().map(|(start, _end)|start).min().unwrap()
}

fn start_len_to_range<T: Add<Output = T> + Copy>(start: T, len: T) -> (T, T) {
    (start, start + len)
}

fn range_intersect_difference<T: Ord + Copy>(range1: (T, T), range2: (T, T)) -> Option<((T, T), Vec<(T, T)>)> {
    let mut difference = Vec::new();
    let mut intersection = None;

    // Check if range1 is entirely before range2
    // or if range1 is entirely after range2
    if range1.1 <= range2.0 || range1.0 >= range2.1 {
        difference.push((range1.0, range1.1));
    }
    // Range1 partially overlaps with range2
    else {
        // Add the portion before the overlap
        if range1.0 < range2.0 {
            difference.push((range1.0, range2.0));
        }
        // Add the portion after the overlap
        if range1.1 > range2.1 {
            difference.push((range2.1, range1.1));
        }

        // Calculate and store the intersection
        let start = range1.0.max(range2.0);
        let end = range1.1.min(range2.1);

        intersection = Some((start, end));
    }

    // If there is an intersection, return intersection and difference
    // Otherwise return None
    intersection.map(|int|{
        (int, difference)
    })
}

fn shift_range<T: Add<Output = T> + Copy>(range: (T, T), shift: T) -> (T, T) {
    (range.0 + shift, range.1 + shift)
}
