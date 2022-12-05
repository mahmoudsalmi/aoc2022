use std::io::Result;

const YEAR: &'static str = "2022";
const DAY: &'static str = "01";

fn to_elf_calories(data: &str) -> Vec<u64> {
    data.split("\n\n")
        .map(|elf_bloc| {
            elf_bloc.lines()
                .map(|line| line.parse::<u64>().expect("Parse Error!"))
                .sum()
        })
        .collect()
}

fn part1(data: &str) -> u64 {
    to_elf_calories(data).iter().map(|&x| x).max().unwrap()
}

fn part2(data: &str) -> u64 {
    let mut elf_calories: Vec<u64> = to_elf_calories(data);
    elf_calories.sort_by(|a, b| b.cmp(a));
    elf_calories.iter()
        .map(|&x| x)
        .take(3)
        .sum::<u64>()
}
fn main() -> Result<()> {
    println!();
    println!("---( AOC{} - Day {} )-----------------------[Rust]----", YEAR, DAY);

    let test_data = include_str!("day01.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");
    

    let input_data = include_str!("day01.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
