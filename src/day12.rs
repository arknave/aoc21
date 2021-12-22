use crate::day::Day;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};
use std::iter;

const START: &'static str = "start";
const END: &'static str = "end";

struct Solver<'a> {
    freq: HashMap<&'a str, u32>,
    edges: &'a [(String, String)],
    has_double: bool,
}

fn is_lower(s: &str) -> bool {
    s.bytes().all(|c| b'a' <= c && c <= b'z')
}

impl<'a> Solver<'a> {
    fn new(edges: &'a [(String, String)], can_double: bool) -> Self {
        Self {
            freq: iter::once((START, 1)).collect(),
            edges: edges,
            has_double: !can_double,
        }
    }

    fn freq_of(&self, node: &'a str) -> u32 {
        *self.freq.get(node).unwrap_or(&0)
    }

    fn can_move(&self, node: &'a str) -> bool {
        if node == START || node == END {
            self.freq_of(node) == 0
        } else if is_lower(node) {
            let cur_freq = self.freq_of(node);
            cur_freq == 0 || (!self.has_double && cur_freq == 1)
        } else {
            true
        }
    }

    fn enter(&mut self, node: &'a str) {
        let new_freq = self.freq_of(node) + 1;

        self.freq.insert(node, new_freq);
        if new_freq == 2 && is_lower(node) {
            self.has_double = true;
        }
    }

    fn leave(&mut self, node: &'a str) {
        let old_freq = self.freq_of(node);

        self.freq.insert(node, old_freq - 1);
        if old_freq == 2 && is_lower(node) {
            self.has_double = false;
        }
    }

    fn dfs(&mut self, cur_node: &'a str) -> u32 {
        if cur_node == END {
            1
        } else {
            self.edges
                .iter()
                .filter_map(|(u, v)| {
                    if u == cur_node {
                        Some(v)
                    } else if v == cur_node {
                        Some(u)
                    } else {
                        None
                    }
                })
                .map(|next_node| {
                    if self.can_move(next_node) {
                        self.enter(next_node);
                        let res = self.dfs(next_node);
                        self.leave(next_node);

                        res
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

pub struct Day12 {
    edges: Vec<(String, String)>,
}

impl Day12 {
    fn count_paths(&self, can_double: bool) -> u32 {
        let mut solver = Solver::new(&self.edges, can_double);

        solver.dfs(START)
    }
}

impl Day for Day12 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let edges: Vec<(String, String)> = reader
            .lines()
            .map(|line_res| {
                line_res.and_then(|line| {
                    let parts: Vec<&str> = line.split('-').collect();
                    if parts.len() != 2 {
                        Err(io::Error::from(io::ErrorKind::InvalidInput))
                    } else {
                        Ok((parts[0].to_string(), parts[1].to_string()))
                    }
                })
            })
            .collect::<io::Result<Vec<_>>>()?;

        Ok(Self { edges: edges })
    }

    fn part1(&self) -> String {
        self.count_paths(false).to_string()
    }

    fn part2(&self) -> String {
        self.count_paths(true).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day12.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day12::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "3802");
        assert_eq!(day.part2(), "99448");
    }
}
