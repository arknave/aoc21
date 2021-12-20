use std::error::Error;
use std::io::BufReader;

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use crate::day::*;
use crate::day01::*;
use crate::day02::*;
use crate::day03::*;
use crate::day04::*;
use crate::day05::*;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut reader = BufReader::new(stdin);

    let day = Day01::new(&mut reader)?;
    let p1 = day.part1();
    let p2 = day.part2();

    println!("{} {}", p1, p2);

    Ok(())
}
