use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn signum(x: i64) -> i64 {
    if x > 0 {
        1
    } else if x == 0 {
        0
    } else {
        -1
    }
}

#[derive(Debug, Clone)]
struct ParseInputError(String);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xy: Vec<&str> = s.split(',').collect();

        assert!(xy.len() == 2);
        let x = xy[0].parse().map_err(|_e| ParseInputError(s.to_string()))?;
        let y = xy[1].parse().map_err(|_e| ParseInputError(s.to_string()))?;

        Ok(Point { x: x, y: y })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Line {
    p0: Point,
    p1: Point,
}

impl FromStr for Line {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ps: Vec<Point> = s.split(" -> ").map(|p| p.parse().unwrap()).collect();

        assert!(ps.len() == 2);
        let p0 = ps[0];
        let p1 = ps[1];

        Ok(Line { p0: p0, p1: p1 })
    }
}

impl Line {
    fn get_points(&self) -> Vec<Point> {
        let dx = signum(self.p1.x - self.p0.x);
        let dy = signum(self.p1.y - self.p0.y);

        let mut res = vec![self.p0];
        let mut p = self.p0;
        while p != self.p1 {
            p.x += dx;
            p.y += dy;
            res.push(p);
        }

        res
    }
}

fn solve(lines: &[Line]) -> i64 {
    let mut freq = HashMap::new();
    for line in lines.iter() {
        for pt in line.get_points() {
            freq.insert(pt, freq.get(&pt).unwrap_or(&0) + 1);
        }
    }

    freq.values().filter(|&&v| v > 1).count() as i64
}

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let reader = BufReader::new(stdin);

    let lines: Vec<Line> = reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    let hv_lines: Vec<Line> = lines
        .iter()
        .filter(|l| l.p0.x == l.p1.x || l.p0.y == l.p1.y)
        .cloned()
        .collect();

    let part1 = solve(&hv_lines);
    let part2 = solve(&lines);

    println!("{} {}", part1, part2);

    Ok(())
}
