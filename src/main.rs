use std::{env, io};
use crate::day01::day01;
use crate::day02::day02;

mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: &str;
    let mut day_arg = String::new();

    if let Some(day_arg) = args.get(1) {
        day = day_arg;
    } else {
        println!("Input day (0 to run latest, a to run all): ");
        io::stdin()
            .read_line(&mut day_arg)
            .expect("Failed to read line");
        println!();
        day = day_arg.trim();
    }

    let days = [day01, day02];

    match day {
        "a" => {
            for (i, day) in days.iter().enumerate() {
                println!("Day {}:", i + 1);
                day();
            }
        }
        "0" => {
            if let Some(day) = days.last() {
                println!("Day {}:", days.len());
                day();
            }
        }
        num => {
            let num: usize = num.parse().expect("Please provide the day as a number, '0' for latest or 'a' for all");
            println!("Day {}:", num);
            if let Some(day) = days.get(num - 1) {
                day();
            } else {
                println!("Day not found")
            }
        }
    }
}

