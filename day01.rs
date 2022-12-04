use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};

type Lines = io::Lines<io::BufReader<File>>;

const YEAR: &'static str = "2022";
const DAY: &'static str = "01";

fn read_lines(filename: &str) -> Result<Lines> {
    let file = File::open(filename).expect(&format!("The file [{}] unreachable!\nCause :", filename));
    Ok(BufReader::new(file).lines())
}

fn part1(lines: Lines) -> Result<u64> {
    let mut max: u64 = 0;
    let mut current_elf: u64 = 0;

    for (idx, line) in lines.enumerate() {
        let line = line.expect(format!("The line [{}] unreachable!", idx).as_str());
        let line = line.as_str();
        match line {
            "" => {
                if current_elf > max {
                    max = current_elf;
                }
                current_elf = 0;
            }
            num_str => {
                current_elf += num_str.parse::<u64>().expect(format!("Impossible to parse [{}]", num_str).as_str());
            }
        }
    }

    if current_elf > max {
        Ok(current_elf)
    } else {
        Ok(max)
    }
}

fn max_three(max: &[u64], value: u64, size: usize) -> Vec<u64> {
    let mut new_max = Vec::from(max);
    new_max.push(value);
    new_max.sort_by(|a, b| b.cmp(a));

    if new_max.len() > size {
        (&new_max[0..size]).to_vec()
    } else {
        new_max
    }
}

fn part2(lines: Lines) -> Result<u64> {
    let mut max: Vec<u64> = vec![];
    let mut current_elf: u64 = 0;

    for (idx, line) in lines.enumerate() {
        let line = line.expect(format!("The line [{}] unreachable!", idx).as_str());
        let line = line.as_str();
        match line {
            "" => {
                max = max_three(&max, current_elf, 3);
                current_elf = 0;
            }
            num_str => {
                current_elf += num_str.parse::<u64>().expect(format!("Impossible to parse [{}]", num_str).as_str());
            }
        }
    }

    max = max_three(&max, current_elf, 3);
    Ok(max.iter().sum())
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

