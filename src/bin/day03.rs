use std::{collections::HashSet, io::Result};

const YEAR: &'static str = "2022";
const DAY: &'static str = "03";

fn priority(c: char) -> u64 {
    match c {
        min if min >= 'a' && min <= 'z' => 1_u64 + min as u64 - 'a' as u64,
        max if max >= 'A' && max <= 'Z' => 27_u64 + max as u64 - 'A' as u64,
        _ => unreachable!("Not alphabetic"),
    }
}

fn to_hashes(s: &str) -> HashSet<u64> {
    s.chars()
        .map(|c| priority(c))
        .fold(HashSet::<u64>::new(), |mut res, priority| {
            res.insert(priority);
            res
        })
}

fn part1(data: &str) -> u64 {
    data.lines()
        .map(|line| line.split_at((line.len() / 2).into()))
        .map(|(left, right)| (to_hashes(left), to_hashes(right)))
        .map(|(l, r)| {
            l.iter()
                .filter(|priority| r.contains(priority))
                .map(|&p| p)
                .sum::<u64>()
        })
        .sum::<u64>()
}

fn part2(data: &str) -> u64 {
    let lines: Vec<&str> = data.lines().filter(|&l| !l.eq("")).collect();

    lines.chunks(3)
        .map(|window| 
            window.iter()
                .map(|line| to_hashes(line))
                .collect::<Vec<HashSet<u64>>>()
        )
        .map(|mut hashes| (hashes.pop().unwrap(), hashes))
        .map(|(head, tail)| 
            head.into_iter()
                .filter(|priority| 
                    tail.iter()
                        .all(|priorities| priorities.contains(priority))
                )
                .sum::<u64>()
        )
        .sum()
}

fn main() -> Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day03.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day03.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
