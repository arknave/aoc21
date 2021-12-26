use crate::day::Day;
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct ParseCommandError(String);

impl fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for ParseCommandError {}

#[derive(Debug, Clone, Copy)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let val = parts[1]
            .parse()
            .map_err(|_| ParseCommandError(s.to_string()))?;

        match parts.get(0) {
            Some(&"forward") => Ok(Command::Forward(val)),
            Some(&"down") => Ok(Command::Down(val)),
            Some(&"up") => Ok(Command::Up(val)),
            _ => Err(ParseCommandError(s.to_string())),
        }
    }
}

struct Position {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Position {
    pub fn new() -> Self {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn part1(&self) -> i64 {
        self.horizontal * self.aim
    }

    pub fn part2(&self) -> i64 {
        self.horizontal * self.depth
    }

    fn update(&self, cmd: Command) -> Self {
        match cmd {
            Command::Forward(f) => Self {
                horizontal: self.horizontal + f,
                depth: self.depth + self.aim * f,
                aim: self.aim,
            },
            Command::Down(d) => Self {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + d,
            },
            Command::Up(u) => Self {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim - u,
            },
        }
    }
}

fn solve(cmds: &[Command]) -> Position {
    cmds.iter()
        .fold(Position::new(), |pos, &cmd| pos.update(cmd))
}

pub struct Day02 {
    end: Position,
}

impl Day for Day02 {
    fn new<R: BufRead>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let commands: Vec<Command> = reader
            .lines()
            .map(|line_res| line_res.map(|line| line.parse()))
            .collect::<io::Result<Result<Vec<Command>, _>>>()??;

        let end = solve(&commands);

        Ok(Self { end })
    }

    fn part1(&self) -> String {
        self.end.part1().to_string()
    }

    fn part2(&self) -> String {
        self.end.part2().to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data() {
        use crate::*;
        use std::io::BufReader;

        let data = include_bytes!("../data_files/day02.txt");
        let mut reader = BufReader::new(&data[..]);

        let day = Day02::new(&mut reader).unwrap();
        assert_eq!(day.part1(), "2187380");
        assert_eq!(day.part2(), "2086357770");
    }
}
