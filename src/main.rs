use std::io::{BufRead, BufReader};

fn solve<T>(iter: &[T], offset: usize) -> u32
where
    T: std::cmp::PartialOrd,
{
    iter.windows(offset + 1)
        .map(|window| (window[0] < window[offset]) as u32)
        .sum()
}

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let reader = BufReader::new(stdin);

    let nums: Vec<i32> = reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    let ans1 = solve(&nums, 1);
    let ans2 = solve(&nums, 3);
    println!("{} {}", ans1, ans2);

    Ok(())
}
