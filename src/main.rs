mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day_trait;

use day_trait::AdventDay;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run -- <day> <part>");
        return;
    }

    let day = &args[1];
    let part = &args[2];

    match day.as_str() {
        "day01" => run_day(&day01::Day01, part),
        "day02" => run_day(&day02::Day02, part),
        "day03" => run_day(&day03::Day03, part),
        "day04" => run_day(&day04::Day04, part),
        "day05" => run_day(&day05::Day05, part),
        _ => println!("Day not recognized"),
    }
}

fn run_day<T: AdventDay>(day: &T, part: &str) {
    match part {
        "part1" => {
            if let Err(e) = day.part1() {
                println!("Error: {e:?}");
            }
        }
        "part2" => {
            if let Err(e) = day.part2() {
                println!("Error: {e:?}");
            }
        }
        _ => println!("Please specify 'part1' or 'part2'"),
    }
}
