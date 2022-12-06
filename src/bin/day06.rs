use std::{collections::HashSet, io};

const YEAR: &'static str = "2022";
const DAY: &'static str = "06";

fn start_after_distinct(data: &str, size: usize) -> usize {
    let signals: Vec<char> = data.chars().collect();

    signals
        .windows(size)
        .enumerate()
        .find_map(|(idx, subroutine)| {
            if subroutine.into_iter().collect::<HashSet<_>>().len() == size {
                Some(idx + size)
            } else {
                None
            }
        })
        .unwrap()
}

fn part1(data: &str) -> usize {
    start_after_distinct(data, 4)
}

fn part2(data: &str) -> usize {
    start_after_distinct(data, 14)
}

fn main() -> io::Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day06.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day06.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
