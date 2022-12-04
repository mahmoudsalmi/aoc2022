use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};

type Lines = io::Lines<io::BufReader<File>>;

const YEAR: &'static str = "2022";
const DAY: &'static str = "XX";

fn read_lines(filename: &str) -> Result<Lines> {
    let file = File::open(filename)
        .expect(format!("The file [{}] unreachable!\nCause :", filename).as_str());
    Ok(BufReader::new(file).lines())
}

fn part1(_lines: Lines) -> Result<i64> {
    Ok(-99999)
}

fn part2(_lines: Lines) -> Result<i64> {
    Ok(-99999)
}

fn main() -> Result<()> {
    let example_file = &format!("day{}.ex.in", DAY);
    let input_file = &format!("day{}.in", DAY);

    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );
    println!(
        "Example :: Part 1 ====>     {}",
        part1(read_lines(example_file)?)?
    );
    println!(
        "Example :: Part 2 ====>     {}",
        part2(read_lines(example_file)?)?
    );
    println!("--------------------------------------------------------");
    println!(
        "Input   :: Part 1 ====>     {}",
        part1(read_lines(input_file)?)?
    );
    println!(
        "Input   :: Part 2 ====>     {}",
        part2(read_lines(input_file)?)?
    );
    println!("--------------------------------------------------------");
    println!();

    Ok(())
}
