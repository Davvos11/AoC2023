use std::collections::HashSet;
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