use regex::Regex;

use crate::day_trait::AdventDay;
use std::{
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

        let re = Regex::new(r":\s([\d\s]+)\|([\d\s]+)").unwrap();
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
        Ok(())
    }
}
