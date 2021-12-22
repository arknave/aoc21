use crate::day::Day;
use crate::util::Point;

use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
}

fn do_fold<I>(points: I, fold_axis: Axis, fold_val: i64) -> HashSet<Point>
where
    I: Iterator<Item = Point>,
{
    points
        .map(|pt| match (fold_axis, pt.x > fold_val, pt.y > fold_val) {
            (Axis::X, true, _) => Point::new(fold_val - (pt.x - fold_val), pt.y),
            (Axis::Y, _, true) => Point::new(pt.x, fold_val - (pt.y - fold_val)),
            _ => pt,
        })
        .collect()
}

/// Given a set of points, display them to the terminal.
/// Uses `.` for blank space and `#` for filled space.
fn display_grid(points: &HashSet<Point>) -> String {
    let (x_range, y_range) = points.iter().fold(
        ((i64::MAX, i64::MIN), (i64::MAX, i64::MIN)),
        |((xlo, xhi), (ylo, yhi)), pt| {
            (
                (cmp::min(xlo, pt.x), cmp::max(xhi, pt.x)),
                (cmp::min(ylo, pt.y), cmp::max(yhi, pt.y)),
            )
        },
    );

    let mut res = String::new();
    for y in y_range.0..=y_range.1 {
        for x in x_range.0..=x_range.1 {
            res.push(if points.contains(&Point::new(x, y)) {
                '#'
            } else {
                '.'
            });
        }
        res.push('\n');
    }

    res
}

pub struct Day13 {
    points: HashSet<Point>,
    folds: Vec<(Axis, i64)>,
}

impl Day for Day13 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut line_iter = reader.lines();
        let points: HashSet<Point> = line_iter
            .by_ref()
            .take_while(|line_res| !line_res.as_ref().unwrap().is_empty())
            .map(|line_res| line_res.map(|line| line.parse()))
            .collect::<io::Result<Result<_, _>>>()??;

        let folds: Vec<(Axis, i64)> = line_iter
            .map(|line_res| {
                line_res.map(|line| {
                    let fold_data = line.split(' ').last().unwrap();
                    let fold_data: Vec<_> = fold_data.split('=').collect();

                    let axis = match fold_data[0] {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        _ => unreachable!(),
                    };

                    let fold_point = fold_data[1].parse();

                    fold_point.map(|pt| (axis, pt))
                })
            })
            .collect::<io::Result<Result<_, _>>>()??;

        Ok(Self {
            points: points,
            folds: folds,
        })
    }

    fn part1(&self) -> String {
        let (axis, fold_val) = self.folds[0];
        let points = do_fold(self.points.iter().cloned(), axis, fold_val);
        let ans = points.len();

        ans.to_string()
    }

    fn part2(&self) -> String {
        let ans: HashSet<Point> = self
            .folds
            .iter()
            .fold(
                self.points.clone().into_iter(),
                |pts_iter, &(axis, fold_val)| do_fold(pts_iter, axis, fold_val).into_iter(),
            )
            .collect();

        display_grid(&ans)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        const ANS: &'static str = concat!(
            ".##..###..#..#...##.####.###...##...##.\n",
            "#..#.#..#.#.#.....#.#....#..#.#..#.#..#\n",
            "#..#.###..##......#.###..###..#....#...\n",
            "####.#..#.#.#.....#.#....#..#.#.##.#...\n",
            "#..#.#..#.#.#..#..#.#....#..#.#..#.#..#\n",
            "#..#.###..#..#..##..#....###...###..##.\n",
        );

        let data = include_bytes!("../data_files/day13.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day13::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "745");
        assert_eq!(day.part2(), ANS);
    }
}
