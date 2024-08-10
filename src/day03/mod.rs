use crate::day_trait::AdventDay;
use regex::Regex;
use std::{
    collections::HashSet,
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
        let re = Regex::new(r"[^a-zA-Z0-9.]").unwrap();

        fn get_int(
            cs: &Vec<char>,
            col: usize,
            row: usize,
            seen_positions: &mut HashSet<(usize, usize)>,
        ) -> i32 {
            let mut number: Vec<char> = Vec::new();
            let mut index = col;

            while cs[index].is_numeric() {
                if !seen_positions.insert((row, index)) {
                    break;
                }

                number.insert(0, cs[index]);

                if index == 0 {
                    break;
                }

                index -= 1;
            }

            index = col + 1;

            while index < cs.len() && cs[index].is_numeric() {
                if !seen_positions.insert((row, index)) {
                    break;
                }

                number.push(cs[index]);
                index += 1;
            }

            number.iter().collect::<String>().parse().unwrap()
        }

        let mut adjacent_numbers: Vec<i32> = Vec::new();

        for (i, row) in matrix.iter().enumerate() {
            let row_str: String = row.iter().collect();

            for mat in re.find_iter(&row_str) {
                let col = mat.start();

                let mut seen_positions: HashSet<(usize, usize)> = HashSet::new();
                let mut counter = 0;

                for (dx, dy) in directions {
                    let posx = (i as isize + dx) as usize;
                    let posy = (col as isize + dy) as usize;

                    if posx < matrix.len() && posy < matrix[posx].len() {
                        if matrix[posx][posy].is_numeric()
                            && !seen_positions.contains(&(posx as usize, posy as usize))
                        {
                            let number = get_int(&matrix[posx], posy, posx, &mut seen_positions);

                            adjacent_numbers.push(number);
                            counter += 1;
                        }
                    }
                }
                if counter != 2 {
                    for _ in 1..=counter {
                        adjacent_numbers.pop();
                    }
                }
            }
        }

        let mut sum = 0;
        let mut i = 0;
        while i < adjacent_numbers.len() {
            sum += adjacent_numbers[i] * adjacent_numbers[i + 1];
            i += 2;
        }

        println!("Answer: {sum}");
        Ok(())
    }
}
