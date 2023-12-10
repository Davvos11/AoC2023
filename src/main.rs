use std::{env, io};
use std::fs::read_to_string;
use std::time::Instant;
use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;
use crate::day06::day06;
use crate::day07::day07;
use crate::day08::day08;
use crate::day09::day09;
use crate::day10::day10;

mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

type Day = (fn(&str) -> (isize, isize), &'static str);

const DAYS: [Day; 10] = [
    (day01, "static/input01.txt"),
    (day02, "static/input02.txt"),
    (day03, "static/input03.txt"),
    (day04, "static/input04.txt"),
    (day05, "static/input05.txt"),
    (day06, "static/input06.txt"),
    (day07, "static/input07.txt"),
    (day08, "static/input08.txt"),
    (day09, "static/input09.txt"),
    (day10, "static/input10.txt"),
];

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

fn run_day(day: &Day, num: usize) {
    println!("---    Day {num:2}:     ---");
    let input = read_to_string(day.1).expect("Could not read input file");
    let start = Instant::now();
    let (result1, result2) = day.0(&input);
    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
    println!("--- Time: {:3.2?} ---\n", start.elapsed());
}