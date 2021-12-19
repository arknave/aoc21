use crate::day::Day;
use std::collections::HashMap;
use std::io::{self, BufRead};

type Bingo = Vec<Vec<u8>>;

fn solve_row(lookup: &HashMap<u8, u8>, board: &Bingo) -> u8 {
    *board
        .iter()
        .map(|row| row.iter().map(|x| lookup.get(x).unwrap()).max().unwrap())
        .min()
        .unwrap()
}

fn transpose(board: &Bingo) -> Bingo {
    let n = board.len();
    (0..n)
        .map(|idx| board.iter().map(|board| board[idx]).collect())
        .collect()
}

fn get_time(lookup: &HashMap<u8, u8>, board: &Bingo) -> u8 {
    // do the rows
    let row_time = solve_row(lookup, board);
    let flip_board = transpose(board);
    let col_time = solve_row(lookup, &flip_board);

    std::cmp::min(row_time, col_time)
}

fn solve(
    nums: &[u8],
    boards: &Vec<Bingo>,
    cmp: fn((u8, Bingo), (u8, Bingo)) -> (u8, Bingo),
) -> u16 {
    let lookup: HashMap<u8, u8> = nums
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i as u8))
        .collect();

    let (time, best_board) = boards
        .iter()
        .map(|board| (get_time(&lookup, board), board.clone()))
        .reduce(cmp)
        .unwrap();

    let total: u16 = best_board
        .iter()
        .flatten()
        .filter(|cell| *lookup.get(cell).unwrap() > time)
        .map(|&cell| cell as u16)
        .sum();

    total * (nums[time as usize] as u16)
}

pub struct Day04 {
    nums: Vec<u8>,
    bingos: Vec<Bingo>,
}

impl Day for Day04 {
    fn new<R: BufRead>(reader: &mut R) -> io::Result<Self> {
        let mut nums = String::new();
        reader.read_line(&mut nums)?;

        let nums: Vec<u8> = nums.trim().split(',').map(|x| x.parse().unwrap()).collect();

        let boards: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

        let bingos: Vec<Bingo> = boards
            .chunks_exact(6)
            .map(|board| {
                board
                    .into_iter()
                    .skip(1)
                    .map(|row| {
                        row.trim()
                            .split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect()
                    })
                    .collect()
            })
            .collect();

        Ok(Self {
            nums: nums,
            bingos: bingos,
        })
    }

    fn part1(&self) -> String {
        solve(&self.nums, &self.bingos, std::cmp::min::<(u8, Bingo)>).to_string()
    }

    fn part2(&self) -> String {
        solve(&self.nums, &self.bingos, std::cmp::max::<(u8, Bingo)>).to_string()
    }
}