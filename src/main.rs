use std::{env, io};
use crate::day01::day01;
use crate::day02::day02;

mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: usize;

    if let Some(day_arg) = args.get(1) {
        day = day_arg.parse().expect("Please provide the day as a number");
    } else {
        let mut day_arg = String::new();
        println!("Input day (0 to run all): ");
        io::stdin()
            .read_line(&mut day_arg)
            .expect("Failed to read line");
        println!();
        day = day_arg.trim().parse().expect("Please provide the day as a number")
    }

    match day {
        0 => {
            println!("Day 1:");
            day01();
            println!("Day 2:");
            day02();
        }
        1 => day01(),
        2 => day02(),
        _ => {
            println!("Day not found")
        }
    }
}

