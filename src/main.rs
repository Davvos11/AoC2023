use std::{env, io};
use std::time::Instant;
use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;

mod day01;
mod day02;
mod day03;

const DAYS: [fn(); 3] = [day01, day02, day03];

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

    match day {
        "a" => {
            for (i, day) in DAYS.iter().enumerate() {
                run_day(day, i + 1);
            }
        }
        "0" => {
            if let Some(day) = DAYS.last() {
                run_day(day, DAYS.len());
            }
        }
        num => {
            let num: usize = num.parse().expect("Please provide the day as a number, '0' for latest or 'a' for all");
            if let Some(day) = DAYS.get(num - 1) {
                run_day(day, num);
            } else {
                println!("Day not found")
            }
        }
    }
}

fn run_day(day: &fn(), num: usize) {
    println!("---    Day {num:2}:     ---");
    let start = Instant::now();
    day();
    println!("--- Time: {:3.2?} ---\n", start.elapsed());
}