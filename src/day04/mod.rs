use regex::Regex;

use crate::day_trait::AdventDay;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub struct Day04;

fn parse_numbers(m: Option<regex::Match>) -> Vec<i32> {
    m.map_or(Vec::new(), |m| {
        m.as_str()
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect()
    })
}

impl AdventDay for Day04 {
    fn part1(&self) -> io::Result<()> {
        let path = "src/day04/input.txt";
        let file = File::open(path)?;
        let read = BufReader::new(file);

        // Example of Regex Match: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let re = Regex::new(r"([\d\s]+)\|([\d\s]+)").unwrap();
        let mut total = 0;

        for line in read.lines() {
            let line = line?;

            if let Some(caps) = re.captures(&line) {
                let winning_numbers: Vec<i32> = parse_numbers(caps.get(1));
                let received_numbers: Vec<i32> = parse_numbers(caps.get(2));

                let points: i32 = received_numbers
                    .clone()
                    .iter()
                    .filter(|n| winning_numbers.contains(n))
                    .collect::<Vec<&i32>>()
                    .len()
                    .try_into()
                    .unwrap();

                if points <= 2 {
                    total += points
                } else {
                    total += 2i32.pow((points - 1) as u32)
                };
            }
        }

        println!("Answer: {total}");
        Ok(())
    }

    fn part2(&self) -> io::Result<()> {
        let path = "src/day04/input.txt";
        let file = File::open(path)?;
        let read = BufReader::new(file);

        let mut card_multipliers: HashMap<i32, i32> = HashMap::new();

        // Example of Regex Match: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let re = Regex::new(r"Card\s+(\d+):\s([\d\s]+)\|([\d\s]+)").unwrap();

        for (i, line) in read.lines().enumerate() {
            let line = line?;

            card_multipliers.entry(i as i32 + 1).or_insert(1);

            if let Some(caps) = re.captures(&line) {
                let card_number: i32 = caps[1].parse().unwrap();
                let winning_numbers: Vec<i32> = parse_numbers(caps.get(2));
                let received_numbers: Vec<i32> = parse_numbers(caps.get(3));

                let points: i32 = received_numbers
                    .iter()
                    .filter(|n| winning_numbers.contains(n))
                    .count() as i32;

                for i in 1..=points {
                    card_multipliers.entry(card_number + i).or_insert(1);

                    card_multipliers.insert(
                        card_number + i,
                        card_multipliers[&(card_number + i)] + card_multipliers[&card_number],
                    );
                }
            }
        }

        let total_value_sum: i32 = card_multipliers.values().sum();

        println!("Answer: {total_value_sum}");
        Ok(())
    }
}
