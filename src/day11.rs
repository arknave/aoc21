use crate::day::Day;
use std::error::Error;
use std::io::{self, BufRead};

// TODO: write some grid module if this is useful elsewhere
type Grid = Vec<Vec<u8>>;

fn step(grid: &Grid) -> (Grid, u32) {
    let n = grid.len();
    let m = grid[0].len();
    assert!(grid.iter().all(|row| row.len() == m));

    let in_bounds = |r, c| 0 <= r && r < (n as i64) && 0 <= c && c < (m as i64);

    let mut res: Grid = grid
        .iter()
        .map(|row| row.iter().map(|x| x + 1).collect())
        .collect();
    let mut stk = vec![];
    let mut flips = vec![];
    for row_idx in 0..n {
        for col_idx in 0..n {
            if res[row_idx][col_idx] == 10 {
                stk.push((row_idx, col_idx));
            }
        }
    }

    while !stk.is_empty() {
        let (row_idx, col_idx) = stk.pop().unwrap();
        flips.push((row_idx, col_idx));
        for dr in -1i64..=1 {
            for dc in -1i64..=1 {
                let new_row = (row_idx as i64) + dr;
                let new_col = (col_idx as i64) + dc;
                if in_bounds(new_row, new_col) {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    res[new_row][new_col] += 1;
                    if res[new_row][new_col] == 10 {
                        stk.push((new_row, new_col));
                    }
                }
            }
        }
    }

    for &(row_idx, col_idx) in flips.iter() {
        res[row_idx][col_idx] = 0;
    }

    (res, flips.len() as u32)
}

pub struct Day11 {
    grid: Grid,
}

impl Day for Day11 {
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
        let (_, ans) = (0..100).fold((self.grid.clone(), 0), |(grid, acc), _| {
            let (nxt_grid, flashes) = step(&grid);
            (nxt_grid, acc + flashes)
        });

        ans.to_string()
    }

    fn part2(&self) -> String {
        let mut cur_step = 0;
        let mut grid = self.grid.clone();
        let mut last_flash = 0u32;

        let all_cells = (grid.len() * grid[0].len()).try_into().unwrap();

        while last_flash != all_cells {
            let (nxt_grid, flashes) = step(&grid);
            grid = nxt_grid.clone();
            last_flash = flashes;
            cur_step += 1;
        }

        cur_step.to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day11.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day11::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "1637");
        assert_eq!(day.part2(), "242");
    }
}
