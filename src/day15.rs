use crate::day::Day;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::io::{self, BufRead};

// TODO: write some grid module if this is useful elsewhere
type Grid = Vec<Vec<u8>>;
type GridRef<'a> = &'a [Vec<u8>];

fn shortest_path(grid: GridRef) -> u64 {
    // TODO: rework this so you dont have to pass in the array every time
    #[inline]
    fn get_dist(dist: &[Vec<u64>], pos: &(usize, usize)) -> u64 {
        dist[pos.0][pos.1]
    }

    let n = grid.len();
    let m = grid[0].len();
    assert!(grid.iter().all(|row| row.len() == m));

    let neighbors = |(r, c)| {
        let mut neighbors = vec![];
        if r > 0 {
            neighbors.push((r - 1, c));
        }
        if c > 0 {
            neighbors.push((r, c - 1));
        }
        if r + 1 < n {
            neighbors.push((r + 1, c));
        }
        if c + 1 < m {
            neighbors.push((r, c + 1));
        }

        neighbors
    };

    // heap is a max-heap, so wrap everything in reverse to make it a min heap
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, (0, 0))));

    let mut dist = vec![vec![u64::MAX; m]; n];
    dist[0][0] = 0;

    while !heap.is_empty() {
        let (cur_dist, pos) = heap.pop().unwrap().0;
        if get_dist(&dist, &pos) != cur_dist {
            continue;
        }

        for neighbor in neighbors(pos) {
            let new_dist = cur_dist + u64::from(grid[neighbor.0][neighbor.1]);
            if new_dist < get_dist(&dist, &neighbor) {
                dist[neighbor.0][neighbor.1] = new_dist;
                heap.push(Reverse((new_dist, neighbor)));
            }
        }
    }

    get_dist(&dist, &(n - 1, m - 1))
}

fn expand_grid(grid: GridRef, copies: usize) -> Grid {
    let n = grid.len();
    let m = grid[0].len();

    (0..(copies * n))
        .map(|row| {
            let (row_add, row_idx) = (row / n, row % n);

            (0..(copies * m))
                .map(move |col| {
                    let (col_add, col_idx) = (col / m, col % m);

                    ((row_add + col_add) as u8 + grid[row_idx][col_idx] - 1) % 9 + 1
                })
                .collect()
        })
        .collect()
}

pub struct Day15 {
    grid: Grid,
}

impl Day for Day15 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let grid: Grid = reader
            .lines()
            .map(|line_res| {
                line_res.map(|line| line.bytes().map(|c| c - b'0').collect::<Vec<u8>>())
            })
            .collect::<io::Result<_>>()?;

        Ok(Self { grid })
    }

    fn part1(&self) -> String {
        shortest_path(&self.grid).to_string()
    }

    fn part2(&self) -> String {
        let big_grid = expand_grid(&self.grid, 5);
        shortest_path(&big_grid).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day15.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day15::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "435");
        assert_eq!(day.part2(), "2842");
    }
}
