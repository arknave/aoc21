use crate::day::Day;
use std::error::Error;
use std::io::{self, BufRead};

const fn is_open(token: u8) -> bool {
    (token == b'(') || (token == b'[') || (token == b'{') || (token == b'<')
}

// TODO: proper error handling in const functions
const fn illegal_table(token: u8) -> u64 {
    match token {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => u64::MAX,
    }
}

const fn incomplete_table(token: u8) -> u64 {
    match token {
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => u64::MAX,
    }
}

const fn match_close(token: u8) -> u8 {
    match token {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => u8::MAX,
    }
}

fn parse_chunk(chunk: &str) -> Result<Vec<u8>, u8> {
    let mut stk = vec![];
    for c in chunk.bytes() {
        if is_open(c) {
            stk.push(c);
        } else {
            if stk.pop() != Some(match_close(c)) {
                return Err(c);
            }
        }
    }

    Ok(stk)
}

fn error_score(chunk: &str) -> u64 {
    match parse_chunk(chunk) {
        Ok(_) => 0,
        Err(token) => illegal_table(token),
    }
}

fn incomplete_score(chunk: &str) -> Option<u64> {
    parse_chunk(chunk).ok().map(|rem| {
        rem.iter()
            .rev()
            .fold(0, |acc, &token| 5 * acc + incomplete_table(token))
    })
}

pub struct Day10 {
    chunks: Vec<String>,
}

impl Day for Day10 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let chunks: Vec<String> = reader.lines().collect::<io::Result<Vec<_>>>()?;

        Ok(Self { chunks })
    }

    fn part1(&self) -> String {
        self.chunks
            .iter()
            .map(|chunk| error_score(chunk))
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut scores: Vec<u64> = self
            .chunks
            .iter()
            .filter_map(|chunk| incomplete_score(chunk))
            .collect();

        assert!(scores.len() % 2 == 1);

        scores.sort();
        scores[scores.len() / 2].to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day10.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day10::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "399153");
        assert_eq!(day.part2(), "2995077699");
    }
}
