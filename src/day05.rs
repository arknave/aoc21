use crate::day::Day;
use crate::util::*;

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};
use std::str::FromStr;

fn signum(x: i64) -> i64 {
    // TODO: Try and replace with match somehow?
    // Or if we use deps, use the num crate version
    if x > 0 {
        1
    } else if x == 0 {
        0
    } else {
        -1
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Line {
    p0: Point,
    p1: Point,
}

impl FromStr for Line {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ps: Vec<Point> = s
            .split(" -> ")
            .map(|p| p.parse())
            .collect::<Result<_, _>>()?;

        assert!(ps.len() == 2);
        let p0 = ps[0];
        let p1 = ps[1];

        Ok(Line { p0: p0, p1: p1 })
    }
}

impl Line {
    fn get_points(&self) -> Vec<Point> {
        let dx = signum(self.p1.x - self.p0.x);
        let dy = signum(self.p1.y - self.p0.y);

        let mut res = vec![self.p0];
        let mut p = self.p0;
        while p != self.p1 {
            p.x += dx;
            p.y += dy;
            res.push(p);
        }

        res
    }
}

fn solve(lines: &[Line]) -> i64 {
    let mut freq = HashMap::new();
    for line in lines.iter() {
        for pt in line.get_points() {
            freq.insert(pt, freq.get(&pt).unwrap_or(&0) + 1);
        }
    }

    freq.values().filter(|&&v| v > 1).count() as i64
}

pub struct Day05 {
    lines: Vec<Line>,
}

impl Day for Day05 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let lines = reader
            .lines()
            .map(|line_res| line_res.map(|line| line.parse()))
            .collect::<io::Result<Result<Vec<_>, _>>>()??;

        Ok(Self { lines: lines })
    }

    fn part1(&self) -> String {
        let hv_lines: Vec<Line> = self
            .lines
            .iter()
            .filter(|l| l.p0.x == l.p1.x || l.p0.y == l.p1.y)
            .cloned()
            .collect();

        solve(&hv_lines).to_string()
    }

    fn part2(&self) -> String {
        solve(&self.lines).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day05.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day05::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "5585");
        assert_eq!(day.part2(), "17193");
    }
}
