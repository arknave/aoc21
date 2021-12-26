use crate::day::Day;

use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufRead};
use std::iter;

// TODO: write some grid module if this is useful elsewhere
type Grid = Vec<Vec<u8>>;

const fn inc(x: usize, m: usize) -> usize {
    if x + 1 >= m {
        0
    } else {
        x + 1
    }
}

pub struct Day25 {
    n: usize,
    m: usize,
    easts: Vec<(usize, usize)>,
    souths: Vec<(usize, usize)>,
}

impl Day25 {
    #[allow(dead_code)]
    fn dump(&self, easts: &HashSet<(usize, usize)>, souths: &HashSet<(usize, usize)>) -> String {
        (0..self.n)
            .flat_map(|r| {
                (0..self.m)
                    .map(move |c| {
                        if easts.contains(&(r, c)) {
                            '>'
                        } else if souths.contains(&(r, c)) {
                            'v'
                        } else {
                            '.'
                        }
                    })
                    .chain(iter::once('\n'))
            })
            .collect()
    }

    fn run(&self) -> u64 {
        let n = self.n;
        let m = self.m;

        let mut easts: HashSet<(usize, usize)> = self.easts.iter().copied().collect();
        let mut souths: HashSet<(usize, usize)> = self.souths.iter().copied().collect();

        let mut steps = 0;
        loop {
            steps += 1;

            let mut changed = false;

            let blocked = &easts | &souths;
            let new_easts = easts
                .iter()
                .map(|&(r, c)| {
                    if blocked.contains(&(r, inc(c, m))) {
                        (r, c)
                    } else {
                        changed = true;
                        (r, inc(c, m))
                    }
                })
                .collect();

            let blocked = &new_easts | &souths;
            let new_souths = souths
                .iter()
                .map(|&(r, c)| {
                    if blocked.contains(&(inc(r, n), c)) {
                        (r, c)
                    } else {
                        changed = true;
                        (inc(r, n), c)
                    }
                })
                .collect();

            if !changed {
                break;
            }

            easts = new_easts;
            souths = new_souths;
        }

        steps
    }
}

impl Day for Day25 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let grid: Grid = reader
            .lines()
            .map(|line_res| line_res.map(|line| line.bytes().collect()))
            .collect::<io::Result<_>>()?;

        let n = grid.len();
        let m = grid[0].len();

        assert!(grid.iter().all(|row| row.len() == m));

        let easts: Vec<(usize, usize)> = grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(c, cell)| match cell {
                        b'>' => Some((r, c)),
                        _ => None,
                    })
            })
            .collect();

        let souths: Vec<(usize, usize)> = grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(c, cell)| match cell {
                        b'v' => Some((r, c)),
                        _ => None,
                    })
            })
            .collect();

        Ok(Self {
            n,
            m,
            easts,
            souths,
        })
    }

    fn part1(&self) -> String {
        self.run().to_string()
    }

    fn part2(&self) -> String {
        "Remote start the sleigh!".to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day25.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day25::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "351");
    }
}
