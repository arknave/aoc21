use std::env;
use std::error::Error;
use std::io::{self, BufRead, BufReader};

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use crate::day::*;
use crate::day01::*;
use crate::day02::*;
use crate::day03::*;
use crate::day04::*;
use crate::day05::*;
use crate::day06::*;
use crate::day07::*;

fn run<D: Day>(day: D) -> Result<(String, String), Box<dyn Error>> {
    Ok((day.part1(), day.part2()))
}

fn run_day<R: BufRead>(day: u8, reader: &mut R) -> Result<(String, String), Box<dyn Error>> {
    match day {
        1 => run(Day01::new(reader)?),
        2 => run(Day02::new(reader)?),
        3 => run(Day03::new(reader)?),
        4 => run(Day04::new(reader)?),
        5 => run(Day05::new(reader)?),
        6 => run(Day06::new(reader)?),
        7 => run(Day07::new(reader)?),
        _ => panic!("Unsupported day {}", day),
    }
}

fn start() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Invalid arguments. pass in a day");
        return Err(Box::new(io::Error::from(io::ErrorKind::InvalidInput)));
    }

    let day: u8 = args[0].parse()?;

    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut reader = BufReader::new(stdin);

    let (p1, p2) = run_day(day, &mut reader)?;
    println!("{} {}", p1, p2);

    Ok(())
}

fn main() {
    std::process::exit(match start() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            1
        }
    });
}
