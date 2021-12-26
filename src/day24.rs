use crate::day::Day;

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};
use std::iter;

/// This day is impossible to solve in a purely general way. Here I've extracted
/// out what I think is the common structure to all inputs. Good enough to get the stars!
pub struct Day24 {
    data: Vec<(i64, i64)>,
}

/// Run one layer of the algorithm
fn step(z: i64, d: i64, c1: i64, c2: i64) -> i64 {
    let mut res = z;

    let cond = (z % 26 + c1) != d;
    if c1 < 0 {
        res /= 26;
    }
    if cond {
        res = 26 * res + d + c2;
    }

    res
}

fn solve(data: &[(i64, i64)], is_maximum: bool) -> i64 {
    // TODO: prove this is sufficient, maybe raise to 6
    const CAP: i64 = 26_i64.pow(4);

    let digits: Vec<i64> = if is_maximum {
        (1..10).collect()
    } else {
        (1..10).rev().collect()
    };

    let zs = data.iter().fold(
        iter::once((0, 0)).collect(),
        |zs: HashMap<i64, i64>, &(c1, c2)| {
            digits
                .iter()
                .flat_map(|&d| {
                    zs.iter().filter_map(move |(&z, &v)| match z <= CAP {
                        true => Some((step(z, d, c1, c2), 10 * v + d)),
                        false => None,
                    })
                })
                .collect()
        },
    );

    *zs.get(&0).unwrap()
}

impl Day for Day24 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let commands: Vec<String> = reader.lines().collect::<io::Result<_>>()?;

        let data: Vec<(i64, i64)> = commands
            .chunks(18)
            .map(|chunk| {
                let c1 = chunk[5].split_whitespace().last().unwrap_or("");
                let c2 = chunk[15].split_whitespace().last().unwrap_or("");

                c1.parse().and_then(|c1| c2.parse().map(|c2| (c1, c2)))
            })
            .collect::<Result<_, _>>()?;

        dbg!(&data);
        Ok(Self { data: data })
    }

    fn part1(&self) -> String {
        solve(&self.data, true).to_string()
    }

    fn part2(&self) -> String {
        solve(&self.data, false).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day24.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day24::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "91897399498995");
        assert_eq!(day.part2(), "51121176121391");
    }
}
