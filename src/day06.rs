use crate::day::Day;
use std::error::Error;
use std::io::BufRead;

type LFState = [u64; 9];

pub struct Day06 {
    state: LFState,
}

fn solve(state: &LFState, days: u32) -> u64 {
    let mut cur = *state;
    for _ in 0..days {
        let zero = cur[0];
        for i in 0..8 {
            cur[i] = cur[i + 1];
        }
        cur[6] += zero;
        cur[8] = zero;
    }

    cur.into_iter().sum()
}

impl Day for Day06 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut nums = String::new();
        reader.read_line(&mut nums)?;

        let nums = nums
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>()?;

        let mut state: LFState = Default::default();
        for x in nums {
            state[x as usize] += 1;
        }

        Ok(Self { state })
    }

    fn part1(&self) -> String {
        solve(&self.state, 80).to_string()
    }

    fn part2(&self) -> String {
        solve(&self.state, 256).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day06.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day06::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "383160");
        assert_eq!(day.part2(), "1721148811504");
    }
}
