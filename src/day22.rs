use crate::day::Day;
use crate::util::ParseInputError;

use std::error::Error;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandType {
    On,
    Off,
}

impl From<CommandType> for bool {
    fn from(command_type: CommandType) -> Self {
        match command_type {
            CommandType::On => true,
            CommandType::Off => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Command {
    command_type: CommandType,
    bounds: [(i64, i64); 3],
}

impl FromStr for Command {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let command_type = match parts[0] {
            "on" => Ok(CommandType::On),
            "off" => Ok(CommandType::Off),
            _ => Err(ParseInputError(s.to_string())),
        }?;

        let bounds: Vec<(i64, i64)> = parts[1]
            .split(',')
            .map(|block| {
                // strip out var name and equals sign
                let block = &block[2..];
                let bound: Result<Vec<_>, _> = block
                    .split("..")
                    .map(|bound| {
                        bound
                            .parse()
                            .map_err(|_| ParseInputError(block.to_string()))
                    })
                    .collect();

                // Change from inclusive to exclusive upper bound
                bound.map(|bound| (bound[0], bound[1] + 1))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            command_type: command_type,
            bounds: [bounds[0], bounds[1], bounds[2]],
        })
    }
}

fn solve(commands: &[Command]) -> u64 {
    fn parse_bounds(commands: &[Command], idx: usize) -> Vec<i64> {
        let mut bounds: Vec<_> = commands
            .iter()
            .flat_map(|command| {
                let bound = command.bounds[idx];
                [bound.0, bound.1].into_iter()
            })
            .collect();

        bounds.sort();
        bounds.dedup();

        bounds
    }

    let xs = parse_bounds(commands, 0);
    let ys = parse_bounds(commands, 1);
    let zs = parse_bounds(commands, 2);

    // TODO: Flatten?
    let x_segs = xs.len() - 1;
    let y_segs = ys.len() - 1;
    let z_segs = zs.len() - 1;

    let mut is_on: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; z_segs]; y_segs]; x_segs];

    let count_on = |on_data: &Vec<Vec<Vec<bool>>>| {
        let mut ans = 0;
        for x_idx in 0..x_segs {
            let x_range = xs[x_idx + 1] - xs[x_idx];

            for y_idx in 0..y_segs {
                let y_range = ys[y_idx + 1] - ys[y_idx];

                for z_idx in 0..z_segs {
                    let z_range = zs[z_idx + 1] - zs[z_idx];

                    if on_data[x_idx][y_idx][z_idx] {
                        ans += x_range * y_range * z_range;
                    }
                }
            }
        }

        ans as u64
    };

    for command in commands.iter() {
        let set_val = bool::from(command.command_type);

        // TODO: eugh
        let x_start = xs.partition_point(|x| x < &command.bounds[0].0);
        for x_idx in x_start..x_segs {
            if !(xs[x_idx + 1] <= command.bounds[0].1) {
                break;
            }

            let y_start = ys.partition_point(|y| y < &command.bounds[1].0);
            for y_idx in y_start..y_segs {
                if !(ys[y_idx + 1] <= command.bounds[1].1) {
                    break;
                }

                let z_start = zs.partition_point(|z| z < &command.bounds[2].0);
                for z_idx in z_start..z_segs {
                    if !(zs[z_idx + 1] <= command.bounds[2].1) {
                        break;
                    }

                    is_on[x_idx][y_idx][z_idx] = set_val;
                }
            }
        }
    }

    count_on(&is_on)
}

pub struct Day22 {
    commands: Vec<Command>,
}

impl Day for Day22 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let commands: Vec<Command> = reader
            .lines()
            .map(|line_res| line_res.map(|line| line.parse()))
            .collect::<io::Result<Result<Vec<Command>, _>>>()??;

        Ok(Self { commands: commands })
    }

    fn part1(&self) -> String {
        let small_commands: Vec<Command> = self
            .commands
            .iter()
            .cloned()
            .filter(|command| {
                command
                    .bounds
                    .iter()
                    .all(|bound| -50 <= bound.0 && bound.1 <= 51)
            })
            .collect();

        solve(&small_commands).to_string()
    }

    fn part2(&self) -> String {
        solve(&self.commands).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day22.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day22::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "556501");
        assert_eq!(day.part2(), "1217140271559773");
    }
}
