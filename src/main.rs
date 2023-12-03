use std::{env, io};
use std::time::Instant;
use crate::day01::day01;
use crate::day02::day02;

mod day01;
mod day02;
mod day03;

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
                run_day(day, i + 1);
            }
        }
        "0" => {
            if let Some(day) = days.last() {
                run_day(day, days.len());
            }
        }
        num => {
            let num: usize = num.parse().expect("Please provide the day as a number, '0' for latest or 'a' for all");
            if let Some(day) = days.get(num - 1) {
                run_day(day, num);
            } else {
                println!("Day not found")
            }
        }
    }
}

fn run_day(day: &fn(), num: usize) {
    println!("---    Day {num:2}:    ---");
    let start = Instant::now();
    day();
    println!("--- Time: {:.2?} ---\n", start.elapsed());
}