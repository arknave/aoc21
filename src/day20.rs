use crate::day::Day;
use crate::util::Point;

use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufRead};

pub struct Day20 {
    rules: Vec<bool>,
    start: Vec<Point>,
}

fn score_neighborhood(points: &HashSet<Point>, p: &Point, background: bool) -> usize {
    p.get_adj9().fold(0, |acc, q| {
        2 * acc + usize::from(points.contains(&q) != background)
    })
}

impl Day20 {
    fn run(&self, days: u32) -> u32 {
        // Hardcoded assumptions
        assert!(self.rules.len() == 512);

        // TODO: test over other values of rules[0], rules[511]
        let start_points: HashSet<Point> = self.start.iter().cloned().collect();

        let (background, points) =
            (0..days).fold((false, start_points), |(background, points), _| {
                let next_background = match background {
                    false => self.rules[0],
                    true => self.rules[511],
                };

                let cands: HashSet<Point> = points.iter().flat_map(|p| p.get_adj9()).collect();

                let next_points = cands
                    .into_iter()
                    .filter(|cand| {
                        let idx = score_neighborhood(&points, &cand, background);
                        self.rules[idx] != next_background
                    })
                    .collect();

                (next_background, next_points)
            });

        if background {
            eprintln!("Infinite on cells!");
            u32::MAX
        } else {
            points.len() as u32
        }
    }
}

impl Day for Day20 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut rules = String::new();
        reader.read_line(&mut rules)?;
        let rules: Vec<bool> = rules.trim().bytes().map(|b| b == b'#').collect();

        assert_eq!(rules.len(), 512);

        let grid: Vec<Vec<bool>> = reader
            .lines()
            .skip(1) // Skip the empty line between rules and grid
            .map(|line_res| {
                line_res.map(|line| line.bytes().map(|c| c == b'#').collect::<Vec<bool>>())
            })
            .collect::<io::Result<_>>()?;

        let start: Vec<Point> = grid
            .into_iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(move |(c, cell)| match cell {
                        true => Some(Point::new(r as i64, c as i64)),
                        false => None,
                    })
            })
            .collect();

        Ok(Self {
            rules: rules,
            start: start,
        })
    }

    fn part1(&self) -> String {
        self.run(2).to_string()
    }

    fn part2(&self) -> String {
        self.run(50).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day20.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day20::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "5479");
        assert_eq!(day.part2(), "19012");
    }
}
