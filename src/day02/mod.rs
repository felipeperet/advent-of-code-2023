use crate::day_trait::AdventDay;
use regex::Regex;
use std::cmp::Ord;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Day02;

impl AdventDay for Day02 {
    fn part1(&self) -> io::Result<()> {
        let path = "src/day02/input.txt";
        let file = File::open(path)?;
        let read = BufReader::new(file);

        let re_id = Regex::new(r"\d+").unwrap();
        let re_round = Regex::new(r"(\d+)\s(red|green|blue)").unwrap();

        let mut sum = 0;

        for line in read.lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(2, ':').collect();

            let game_id = re_id.find(parts[0]).unwrap().as_str().parse().unwrap_or(0);
            let mut rounds = parts[1].split(';');

            let all_rounds_valid = rounds.all(|round| {
                let colors = re_round.captures_iter(round).fold(
                    (None, None, None),
                    |(red, green, blue), cap| {
                        let value = cap[1].parse().unwrap_or(0);
                        match &cap[2] {
                            "red" => (Some(value), green, blue),
                            "green" => (red, Some(value), blue),
                            "blue" => (red, green, Some(value)),
                            _ => (red, green, blue),
                        }
                    },
                );
                colors.0.map_or(true, |v| v <= 12)
                    && colors.1.map_or(true, |v| v <= 13)
                    && colors.2.map_or(true, |v| v <= 14)
            });

            if all_rounds_valid {
                sum += game_id;
            }
        }

        println!("Answer: {sum}");
        Ok(())
    }

    fn part2(&self) -> io::Result<()> {
        let path = "src/day02/input.txt";
        let file = File::open(path)?;
        let read = BufReader::new(file);

        let re_round = Regex::new(r"(\d+)\s(red|green|blue)").unwrap();

        let mut sum = 0;

        for line in read.lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(2, ':').collect();

            let mut line_max = (0, 0, 0);

            let rounds = parts[1].split(';');

            rounds.for_each(|round| {
                let max_values =
                    re_round
                        .captures_iter(round)
                        .fold(line_max, |(red, green, blue), cap| {
                            let value = cap[1].parse().unwrap_or(0);
                            match &cap[2] {
                                "red" => (red.max(value), green, blue),
                                "green" => (red, green.max(value), blue),
                                "blue" => (red, green, blue.max(value)),
                                _ => (red, green, blue),
                            }
                        });
                line_max = (
                    line_max.0.max(max_values.0),
                    line_max.1.max(max_values.1),
                    line_max.2.max(max_values.2),
                );
            });

            sum += line_max.0 * line_max.1 * line_max.2;
        }

        println!("Answer: {sum}");
        Ok(())
    }
}
