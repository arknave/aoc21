use crate::day::Day;
use crate::util::ParseInputError;

use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::io::{self, BufRead};

fn add_map<K>(hash_map: &mut HashMap<K, u64>, key: K, inc: u64)
where
    K: Eq + Hash,
{
    let new_val = hash_map.get(&key).unwrap_or(&0) + inc;
    hash_map.insert(key, new_val);
}

pub struct Day14 {
    template: Vec<u8>,
    rules: HashMap<(u8, u8), u8>,
}

impl Day14 {
    fn advance(&self, pairs: &HashMap<(u8, u8), u64>) -> HashMap<(u8, u8), u64> {
        let mut res = HashMap::new();
        for (&(a, b), &f) in pairs.iter() {
            match self.rules.get(&(a, b)) {
                Some(&c) => {
                    add_map(&mut res, (a, c), f);
                    add_map(&mut res, (c, b), f);
                }
                None => {
                    add_map(&mut res, (a, b), f);
                }
            }
        }

        res
    }

    fn solve(&self, days: u32) -> u64 {
        // Implementation hack: add a dummy value to the end of the string
        // That way we only need to check the frequency of (c, _) for all c
        assert!(!self.template.is_empty());

        let mut pair_freq = HashMap::new();
        for window in self.template.windows(2) {
            let pair = (window[0], window[1]);
            add_map(&mut pair_freq, pair, 1);
        }

        pair_freq.insert((*self.template.last().unwrap(), b'#'), 1);

        let pair_freq = (0..days).fold(pair_freq, |pairs, _| self.advance(&pairs));

        let mut single_freq = HashMap::new();
        for ((k, _), f) in pair_freq {
            add_map(&mut single_freq, k, f);
        }

        let (lo, hi) = single_freq.values().fold((u64::MAX, u64::MIN), |acc, &v| {
            (cmp::min(acc.0, v), cmp::max(acc.1, v))
        });

        hi - lo
    }
}

impl Day for Day14 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        fn parse_line(s: String) -> Result<((u8, u8), u8), ParseInputError> {
            let bytes = s.as_bytes();

            if bytes.len() == 7 {
                Ok(((bytes[0], bytes[1]), bytes[6]))
            } else {
                Err(ParseInputError(s))
            }
        }

        let mut template = String::new();
        reader.read_line(&mut template)?;
        let template = template.trim().bytes().collect();

        let rules: HashMap<(u8, u8), u8> = reader
            .lines()
            .skip(1) // Skip the empty line between template and rules
            .map(|line_res| line_res.map(parse_line))
            .collect::<io::Result<Result<_, _>>>()??;

        Ok(Self { template, rules })
    }

    fn part1(&self) -> String {
        self.solve(10).to_string()
    }

    fn part2(&self) -> String {
        self.solve(40).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day14.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day14::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "2712");
        assert_eq!(day.part2(), "8336623059567");
    }
}
