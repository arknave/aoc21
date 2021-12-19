use std::io::BufRead;
use std::io::Result;

/// A day takes in some reader and outputs two strings, one for each part
pub trait Day {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self>
    where
        Self: Sized;

    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
