use aho_corasick::AhoCorasick;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let first_solution = part1().unwrap();
    let second_solution = part2().unwrap();

    println!("Part 1: {first_solution}");
    println!("Part 2: {second_solution}");
}

fn part1() -> io::Result<i32> {
    let path = "data/calibration.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"\d").unwrap();

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<String> = re
            .find_iter(&line)
            .filter_map(|m| Some(m.as_str().to_string()))
            .collect();

        if let (Some(first), Some(last)) = (numbers.first(), numbers.last()) {
            let calibration_value = format!("{first}{last}").parse::<i32>().unwrap_or(0);
            sum += calibration_value;
        }
    }

    Ok(sum)
}

fn part2() -> io::Result<i32> {
    let path = "data/calibration.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let patterns = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let ac = AhoCorasick::new(patterns).unwrap();

    let word_to_num = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let mut matches: Vec<String> = Vec::new();

        for mat in ac.find_overlapping_iter(&line) {
            let matched_str = &line[mat.start()..mat.end()];
            let num_str = word_to_num.get(matched_str).unwrap_or(&matched_str);
            matches.push(num_str.to_string());
        }

        if let (Some(first), Some(last)) = (matches.first(), matches.last()) {
            let calibration_string = format!("{first}{last}");
            sum += calibration_string.parse::<i32>().unwrap_or(0);
        }
    }

    Ok(sum)
}
