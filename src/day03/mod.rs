use crate::day_trait::AdventDay;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub struct Day03;

impl AdventDay for Day03 {
    fn part1(&self) -> io::Result<()> {
        let path = "src/day03/input.txt";
        let file = File::open(path)?;
        let read = BufReader::new(file);

        let matrix: Vec<Vec<char>> = read
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.chars().collect())
            .collect();

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let number_regex = Regex::new(r"\d+").unwrap();

        let mut adjacent_numbers: Vec<String> = Vec::new();

        for (i, row) in matrix.iter().enumerate() {
            let row_str: String = row.iter().collect();

            for mat in number_regex.find_iter(&row_str) {
                let number = mat.as_str().to_string();
                let mut should_push = false;

                for (j, _) in number.chars().enumerate() {
                    let digit_col = mat.start() + j;

                    should_push = directions.iter().any(|(dx, dy)| {
                        let posx = i as isize + dx;
                        let posy = digit_col as isize + dy;

                        if posx >= 0
                            && posx < matrix.len() as isize
                            && posy >= 0
                            && posy < matrix[posx as usize].len() as isize
                        {
                            let adj_char = matrix[posx as usize][posy as usize];
                            !adj_char.is_alphanumeric() && adj_char != '.'
                        } else {
                            false
                        }
                    });

                    if should_push {
                        break;
                    }
                }
                if should_push {
                    adjacent_numbers.push(number)
                }
            }
        }

        let total_sum: i32 = adjacent_numbers
            .iter()
            .map(|x| x.parse::<i32>())
            .filter_map(Result::ok)
            .sum();

        println!("Answer: {total_sum}");

        Ok(())
    }

    fn part2(&self) -> io::Result<()> {
        Ok(())
    }
}
