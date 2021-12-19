use crate::day::Day;
use std::io::{self, BufRead};

fn part1(report: &Vec<String>) -> i64 {
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
        ans *= 2;
        if f + f >= report.len() {
            ans += 1;
        }
    }

    ans
}

fn _part2(report: &Vec<String>, idx: usize, win: u8) -> i64 {
    if report.len() == 1 {
        return i64::from_str_radix(&report[0], 2).unwrap();
    }

    let num_bits = report[0].len();
    assert!(report.iter().all(|x| x.len() == num_bits));

    let f: usize = report
        .iter()
        .map(|s| {
            if s.bytes().nth(idx) == Some(b'1') {
                1
            } else {
                0
            }
        })
        .sum();
    let goal = if f + f >= report.len() { win } else { win ^ 1 };

    let sub_reports = &report
        .into_iter()
        .filter(|s| s.bytes().nth(idx) == Some(goal))
        .cloned()
        .collect();

    _part2(&sub_reports, idx + 1, win)
}

fn part2(report: &Vec<String>, win: u8) -> i64 {
    _part2(report, 0, win)
}

pub struct Day03 {
    report: Vec<String>,
}

impl Day for Day03 {
    fn new<R: BufRead>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            report: reader
                .lines()
                .map(|x| x.unwrap().parse().unwrap())
                .collect(),
        })
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
