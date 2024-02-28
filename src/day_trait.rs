use std::io;

pub trait AdventDay {
    fn part1(&self) -> io::Result<()>;
    fn part2(&self) -> io::Result<()>;
}
