use crate::day::Day;
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct ParsePatternError(String);

impl fmt::Display for ParsePatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for ParsePatternError {}

#[derive(Debug)]
struct Pattern {
    clues: Vec<u8>,
    output: Vec<u8>,
}

impl FromStr for Pattern {
    type Err = ParsePatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn to_byte(s: &str) -> u8 {
            s.as_bytes()
                .iter()
                .fold(0, |acc, c| acc | (1 << (c - b'a')))
        }

        let parts: Vec<&str> = s.split(" | ").collect();
        if parts.len() != 2 {
            return Err(ParsePatternError(s.to_string()));
        }
        let clues: Vec<u8> = parts[0].split_whitespace().map(to_byte).collect();
        let output: Vec<u8> = parts[1].split_whitespace().map(to_byte).collect();

        if clues.len() != 10 || output.len() != 4 {
            return Err(ParsePatternError(s.to_string()));
        }

        Ok(Self {
            clues: clues,
            output: output,
        })
    }
}

impl Pattern {
    fn solve(&self) -> u64 {
        // segs: digits
        // 2: 1
        // 3: 7
        // 4: 4
        // 5: 2, 3, 5
        // 6: 0, 6, 9
        // 7: 8

        // from 1 and 7, can figure out top bar and right two
        // 5ers:
        // 3 is the only 5 digit one that shares all segments with 1
        // 5 shares more segments with 4 than 2 does
        //
        // 6ers:
        // 1 is a subset of 0, 9
        // 4 is a subset of 9

        // TODO: add some kind of check to ensure the iterator has length 1
        let one = self
            .clues
            .iter()
            .filter(|segs| segs.count_ones() == 2)
            .next()
            .expect("Should have a 1");
        let four = self
            .clues
            .iter()
            .filter(|segs| segs.count_ones() == 4)
            .next()
            .expect("Should have a 4");

        let lookup = |segs: u8| -> u64 {
            match (
                segs.count_ones(),
                (segs & one).count_ones(),
                (segs & four).count_ones(),
            ) {
                (6, 2, 3) => 0,
                (2, _, _) => 1,
                (5, 1, 2) => 2,
                (5, 2, _) => 3,
                (4, _, _) => 4,
                (5, 1, 3) => 5,
                (6, 1, _) => 6,
                (3, _, _) => 7,
                (7, _, _) => 8,
                (6, 2, 4) => 9,
                _ => unreachable!(),
            }
        };

        self.output
            .iter()
            .fold(0, |acc, &segs| 10 * acc + lookup(segs))
    }
}

const UNIQUE_LENS: [u32; 4] = [2, 4, 3, 7];

pub struct Day08 {
    patterns: Vec<Pattern>,
}

impl Day for Day08 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let patterns: Vec<Pattern> = reader
            .lines()
            .map(|line_res| line_res.map(|line| line.parse()))
            .collect::<io::Result<Result<Vec<Pattern>, _>>>()??;

        Ok(Self { patterns: patterns })
    }

    fn part1(&self) -> String {
        self.patterns
            .iter()
            .map(|pattern| {
                pattern
                    .output
                    .iter()
                    .filter(|segs| UNIQUE_LENS.contains(&segs.count_ones()))
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.patterns
            .iter()
            .map(|pattern| pattern.solve())
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day08.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day08::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "476");
        assert_eq!(day.part2(), "1011823");
    }
}
