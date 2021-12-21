use crate::day::Day;
use std::error::Error;
use std::io::BufRead;

pub struct Day07 {
    positions: Vec<i64>,
}

fn solve(positions: &[i64], dist: fn(i64) -> i64) -> u64 {
    if positions.is_empty() {
        return 0;
    }

    // TODO: Optimize to O(n) or O(n log n) if this ends up being slow
    let &lo = positions
        .iter()
        .min()
        .expect("Must have at least one position");
    let &hi = positions
        .iter()
        .max()
        .expect("Must have at least one position");
    (lo..=hi)
        .map(|center| {
            // TODO: Replace with abs_diff when not experimental
            positions
                .iter()
                .map(|x| dist(i64::abs(center - x)))
                .sum::<i64>() as u64
        })
        .min()
        .expect("Range cannot be empty")
}

impl Day for Day07 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut positions = String::new();
        reader.read_line(&mut positions)?;

        let positions = positions
            .trim()
            .split(',')
            .map(|x| x.parse())
            .collect::<Result<Vec<i64>, _>>()?;

        // TODO: sort positions here if that matters
        Ok(Self {
            positions: positions,
        })
    }

    fn part1(&self) -> String {
        solve(&self.positions, |x| x).to_string()
    }

    fn part2(&self) -> String {
        solve(&self.positions, |x| x * (x + 1) / 2).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day07.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day07::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "347011");
        assert_eq!(day.part2(), "98363777");
    }
}
