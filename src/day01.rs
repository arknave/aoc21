use crate::day::Day;
use std::error::Error;
use std::io::{self, BufRead};

fn solve<T>(iter: &[T], offset: usize) -> u32
where
    T: std::cmp::PartialOrd,
{
    iter.windows(offset + 1)
        .map(|window| (window[0] < window[offset]) as u32)
        .sum()
}

pub struct Day01 {
    nums: Vec<i64>,
}

impl Day for Day01 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let nums: Vec<i64> = reader
            .lines()
            .map(|line_res| line_res.map(|line| line.parse()))
            .collect::<io::Result<Result<Vec<i64>, _>>>()??;

        Ok(Self { nums: nums })
    }

    fn part1(&self) -> String {
        solve(&self.nums, 1).to_string()
    }

    fn part2(&self) -> String {
        solve(&self.nums, 3).to_string()
    }
}
