use crate::day::Day;
use crate::util::ParseInputError;

use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;

// (number of turns, final scores)
fn play_fixed(start: &[u32; 2], sides: u32, max_score: u32) -> (u32, [u32; 2]) {
    fn advance(f: u32, s: u32) -> u32 {
        let ans = f + 1;

        if ans > s {
            ans - s
        } else {
            ans
        }
    }

    let mut turns = 0;
    let mut pos = start.clone();
    let mut score = [0, 0];

    let mut player = 0;
    let mut face = 1;
    while cmp::max(score[0], score[1]) < max_score {
        for _ in 0..3 {
            pos[player] += face;
            face = advance(face, sides);
            turns += 1;
        }

        pos[player] %= 10;
        score[player] += pos[player] + 1;

        player ^= 1;
    }

    (turns, score)
}

/// Returns the number of universes each player wins in
fn play_quantum(start: &[u32; 2], sides: u32, max_score: u32) -> [u64; 2] {
    let mut memo = HashMap::new();
    let mut opts = vec![0; (3 * sides + 1) as usize];
    for a in 1..=sides {
        for b in 1..=sides {
            for c in 1..=sides {
                opts[(a + b + c) as usize] += 1;
            }
        }
    }

    fn solve(
        memo: &mut HashMap<([u32; 2], [u32; 2]), [u64; 2]>,
        opts: &[u64],
        sides: u32,
        max_score: u32,
        pos: &[u32; 2],
        score: &[u32; 2],
    ) -> [u64; 2] {
        let state = (*pos, *score);
        if let Some(res) = memo.get(&state) {
            return *res;
        }

        let res = if score[0] >= max_score {
            [1, 0]
        } else if score[1] >= max_score {
            [0, 1]
        } else {
            let mut res = [0, 0];
            for (roll, &freq) in opts.iter().enumerate() {
                if freq == 0 {
                    continue;
                }

                let roll = roll as u32;
                let new_pos = (pos[0] + roll) % 10;
                let new_score = score[0] + new_pos + 1;

                let ways = solve(
                    memo,
                    opts,
                    sides,
                    max_score,
                    &[pos[1], new_pos],
                    &[score[1], new_score],
                );

                res[0] += freq * ways[1];
                res[1] += freq * ways[0];
            }

            res
        };

        memo.insert(state, res);
        res
    }

    solve(&mut memo, &opts, sides, max_score, start, &[0, 0])
}

pub struct Day21 {
    start: [u32; 2],
}

impl Day for Day21 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut start: [u32; 2] = Default::default();

        for idx in 0..2 {
            let mut line = String::new();
            reader.read_line(&mut line)?;

            let x: u32 = line
                .split_whitespace()
                .last()
                .ok_or(ParseInputError(line.clone()))?
                .parse()?;
            start[idx] = x - 1;
        }

        Ok(Self { start })
    }

    fn part1(&self) -> String {
        let (turns, score) = play_fixed(&self.start, 100, 1000);

        (turns * cmp::min(score[0], score[1])).to_string()
    }

    fn part2(&self) -> String {
        let universes = play_quantum(&self.start, 3, 21);

        cmp::max(universes[0], universes[1]).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day21.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day21::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "1002474");
        assert_eq!(day.part2(), "919758187195363");
    }
}
