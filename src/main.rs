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
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod util;

use crate::day::*;
use crate::day01::*;
use crate::day02::*;
use crate::day03::*;
use crate::day04::*;
use crate::day05::*;
use crate::day06::*;
use crate::day07::*;
use crate::day08::*;
use crate::day09::*;
use crate::day10::*;
use crate::day11::*;
use crate::day12::*;
use crate::day13::*;
use crate::day14::*;
use crate::day15::*;
use crate::day16::*;
use crate::day17::*;
use crate::day18::*;
use crate::day19::*;

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
        8 => run(Day08::new(reader)?),
        9 => run(Day09::new(reader)?),
        10 => run(Day10::new(reader)?),
        11 => run(Day11::new(reader)?),
        12 => run(Day12::new(reader)?),
        13 => run(Day13::new(reader)?),
        14 => run(Day14::new(reader)?),
        15 => run(Day15::new(reader)?),
        16 => run(Day16::new(reader)?),
        17 => run(Day17::new(reader)?),
        18 => run(Day18::new(reader)?),
        19 => run(Day19::new(reader)?),
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
