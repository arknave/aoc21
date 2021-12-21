use crate::day::Day;
use std::error::Error;
use std::io::{self, BufRead};

fn part1(report: &[String]) -> i64 {
    let num_bits = report[0].len();
    assert!(report.iter().all(|x| x.len() == num_bits));

    let mut freq = vec![0; num_bits];
    for diag in report.iter() {
        for i in 0..num_bits {
            if diag.bytes().nth(i) == Some(b'1') {
                freq[i] += 1;
            }
        }
    }

    let mut ans = 0;
    for f in freq.iter() {
        ans <<= 1;
        if f + f >= report.len() {
            ans |= 1;
        }
    }

    ans
}

fn _part2(report: &[String], idx: usize, win: u8) -> i64 {
    if report.len() == 1 {
        return i64::from_str_radix(&report[0], 2).unwrap();
    }

    let num_bits = report[0].len();
    assert!(report.iter().all(|x| x.len() == num_bits));

    let column_sum: usize = report
        .iter()
        .map(|s| {
            if s.bytes().nth(idx) == Some(b'1') {
                1
            } else {
                0
            }
        })
        .sum();

    let goal = if column_sum + column_sum >= report.len() {
        win
    } else {
        win ^ 1
    };
    let sub_reports: Vec<String> = report
        .into_iter()
        .filter(|s| s.bytes().nth(idx) == Some(goal))
        .cloned()
        .collect();

    _part2(&sub_reports, idx + 1, win)
}

fn part2(report: &[String], win: u8) -> i64 {
    _part2(report, 0, win)
}

pub struct Day03 {
    report: Vec<String>,
}

impl Day for Day03 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let report: Vec<String> = reader.lines().collect::<io::Result<Vec<_>>>()?;

        Ok(Self { report: report })
    }

    fn part1(&self) -> String {
        let gamma = part1(&self.report);
        // hack: epsilon is the same as gamma with the bits flipped.
        let num_bits = self.report[0].len();
        let epsilon = ((1 << num_bits) - 1) ^ gamma;

        (gamma * epsilon).to_string()
    }

    fn part2(&self) -> String {
        let oxygen = part2(&self.report, b'1');
        let co2 = part2(&self.report, b'0');

        (oxygen * co2).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day03.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day03::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "738234");
        assert_eq!(day.part2(), "3969126");
    }
}
