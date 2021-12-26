use crate::day::Day;
use crate::util::Point3D;

use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufRead};

/// Assume the points in src are centered at the origin and using a canonical rep
/// Find a center and orientation for the points in "other" that leads to at least 12 matches.
fn match_scanners(
    scanner: &HashSet<Point3D>,
    other: &[Point3D],
) -> Option<(Point3D, Vec<Point3D>)> {
    // Mapping from one point to another with a fixed permutation + axis negation + center is an invertible
    // linear transform. Therefore we can just compute all possible transforms and just return the
    // one that works.

    // Note that this implementation checks 48 orientations (3! axis orderings, and then 2^3 axis
    // flips), but it should only need to check 24 because of the right hand rule. We'll leave that
    // as a future optimization.

    // TODO: Don't hard code these
    const PERMS: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    const FLIPS: [[i64; 3]; 8] = [
        [1, 1, 1],
        [1, 1, -1],
        [1, -1, 1],
        [1, -1, -1],
        [-1, 1, 1],
        [-1, 1, -1],
        [-1, -1, 1],
        [-1, -1, -1],
    ];

    for perm in PERMS {
        for flip in FLIPS {
            let fixed: Vec<Point3D> = other.iter().map(|p| p.transform(perm, flip)).collect();
            for source in scanner.iter() {
                for dest in fixed.iter() {
                    let center = *source - *dest;
                    let shifted: Vec<Point3D> = fixed.iter().map(|p| *p + center).collect();
                    let found = shifted.iter().filter(|p| scanner.contains(p)).count();

                    if found >= 12 {
                        return Some((center, shifted));
                    }
                }
            }
        }
    }

    None
}

pub struct Day19 {
    centers: Vec<Point3D>,
    fixed_points: Vec<HashSet<Point3D>>,
}

impl Day19 {
    fn build(scanners: &Vec<Vec<Point3D>>) -> Self {
        let (centers, fixed_points) = Self::solve(scanners);
        Self {
            centers,
            fixed_points,
        }
    }

    fn solve(scanners: &Vec<Vec<Point3D>>) -> (Vec<Point3D>, Vec<HashSet<Point3D>>) {
        let n = scanners.len();
        let mut vis = vec![false; n];
        let mut centers = vec![Point3D::new(0, 0, 0); n];
        let mut fixed = vec![HashSet::new(); n];

        vis[0] = true;
        fixed[0] = scanners[0].iter().cloned().collect();

        let mut stk = vec![0];
        while !stk.is_empty() {
            let u = stk.pop().unwrap();

            // TODO: Figure out how to avoid this intermediate allocation
            let results: Vec<_> = (0..n)
                .filter(|&v| !vis[v])
                .filter_map(|v| match_scanners(&fixed[u], &scanners[v]).map(|res| (v, res)))
                .collect();

            for (v, (center, fixed_points)) in results {
                centers[v] = center;
                fixed[v] = fixed_points.into_iter().collect();
                vis[v] = true;
                stk.push(v);
            }
        }

        assert!(vis.iter().all(|&x| x));
        (centers, fixed)
    }
}

impl Day for Day19 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
        let scanners: Vec<Vec<Point3D>> = lines
            .split(|line| line.is_empty())
            .map(|scanner| {
                scanner[1..]
                    .into_iter()
                    .map(|pt: &String| pt.parse::<Point3D>())
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;

        Ok(Self::build(&scanners))
    }

    fn part1(&self) -> String {
        let all_points: HashSet<Point3D> = self.fixed_points.iter().flatten().cloned().collect();

        all_points.len().to_string()
    }

    fn part2(&self) -> String {
        let centers = &self.centers;

        let ans = centers
            .iter()
            .map(|&center0| {
                centers
                    .iter()
                    .map(|&center1| center0.manhattan_dist(center1))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();

        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    // TODO: This is too slow to run with other tests
    #[test]
    #[ignore]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day19.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day19::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "449");
        assert_eq!(day.part2(), "13128");
    }
}
