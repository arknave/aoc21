use crate::day::Day;
use std::error::Error;
use std::io::{self, BufRead};

struct UnionFind {
    // parent[x] is the index if >= 0 and negative size if < 0
    parent: Vec<i64>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![-1; n],
        }
    }

    pub fn find(&mut self, idx: usize) -> usize {
        if self.parent[idx] < 0 {
            idx
        } else {
            // Path compression
            let res = self.find(self.parent[idx] as usize);
            self.parent[idx] = res as i64;

            res
        }
    }

    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let ur = self.find(u);
        let vr = self.find(v);
        if ur != vr {
            self.parent[ur] += self.parent[vr];
            self.parent[vr] = ur as i64;
        }

        ur != vr
    }

    pub fn get_sizes(&self) -> Vec<i64> {
        self.parent
            .iter()
            .filter_map(|&val| if val < 0 { Some(-val) } else { None })
            .collect()
    }
}

// TODO: write some grid module if this is useful elsewhere
type Grid = Vec<Vec<u8>>;

fn low_points(grid: &Grid) -> Vec<u8> {
    let n = grid.len();
    let m = grid[0].len();
    assert!(grid.iter().all(|row| row.len() == m));

    let neighbors = |x, y| {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x + 1 < n {
            neighbors.push((x + 1, y));
        }
        if y + 1 < m {
            neighbors.push((x, y + 1));
        }

        neighbors
    };

    let mut res = vec![];
    for row_idx in 0..n {
        for col_idx in 0..m {
            let mut is_low = true;
            for (neighbor_row, neighbor_col) in neighbors(row_idx, col_idx) {
                is_low &= grid[row_idx][col_idx] < grid[neighbor_row][neighbor_col]
            }

            if is_low {
                res.push(grid[row_idx][col_idx]);
            }
        }
    }

    res
}

fn components(grid: &Grid) -> Vec<i64> {
    // TODO: This method returns each cell of size 9 as a size-1 CC, which may not be great...
    let n = grid.len();
    let m = grid[0].len();
    assert!(grid.iter().all(|row| row.len() == m));

    let get_idx = |row_idx, col_idx| m * row_idx + col_idx;

    let mut uf = UnionFind::new(n * m);

    for row_idx in 0..n {
        for col_idx in 0..m {
            if grid[row_idx][col_idx] == 9 {
                continue;
            }

            let cur_idx = get_idx(row_idx, col_idx);
            if row_idx + 1 < n && grid[row_idx + 1][col_idx] != 9 {
                let next_idx = get_idx(row_idx + 1, col_idx);
                uf.merge(cur_idx, next_idx);
            }
            if col_idx + 1 < m && grid[row_idx][col_idx + 1] != 9 {
                let next_idx = get_idx(row_idx, col_idx + 1);
                uf.merge(cur_idx, next_idx);
            }
        }
    }

    uf.get_sizes()
}

pub struct Day09 {
    grid: Grid,
}

impl Day for Day09 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let grid: Vec<Vec<u8>> = reader
            .lines()
            .map(|line_res| {
                line_res.map(|line| line.bytes().map(|c| c - b'0').collect::<Vec<u8>>())
            })
            .collect::<io::Result<Vec<Vec<u8>>>>()?;

        Ok(Self { grid: grid })
    }

    fn part1(&self) -> String {
        let low_pts = low_points(&self.grid);
        (low_pts.len() + low_pts.iter().map::<usize, _>(|&v| v.into()).sum::<usize>()).to_string()
    }

    fn part2(&self) -> String {
        let mut components = components(&self.grid);
        components.sort_unstable_by(|a, b| a.cmp(b).reverse());
        components.into_iter().take(3).product::<i64>().to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day09.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day09::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "500");
        assert_eq!(day.part2(), "970200");
    }
}
