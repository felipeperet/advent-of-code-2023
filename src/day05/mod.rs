use std::fs::read_to_string;

use crate::day_trait::AdventDay;

pub struct Day05;

impl AdventDay for Day05 {
    fn part1(&self) -> std::io::Result<()> {
        let path = "src/day05/input.txt";
        let input = read_to_string(path).unwrap();

        let mut vec: Vec<u32> = Vec::new();

        for (i, _) in input.lines().enumerate() {
            vec.push((i + 1) as u32);
        }

        let xs: Vec<u32> = vec.into_iter().map(|x| x * 2).collect();

        println!("{:?}", xs);
        Ok(())
    }

    fn part2(&self) -> std::io::Result<()> {
        Ok(())
    }
}
