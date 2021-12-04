use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct ParseCommandError(String);

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

        match parts[0] {
            "forward" => Ok(Command::Forward(val)),
            "down" => Ok(Command::Down(val)),
            "up" => Ok(Command::Up(val)),
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
        .fold(Position::new(), |pos, cmd| pos.update(*cmd))
}

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let reader = BufReader::new(stdin);

    let commands: Vec<Command> = reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    let state = solve(&commands);
    println!("{} {}", state.part1(), state.part2());

    Ok(())
}
