use std::{io, num::ParseIntError, str::FromStr};

const YEAR: &'static str = "2022";
const DAY: &'static str = "04";

#[derive(Debug)]
struct Section {
    start: u64,
    end: u64,
}

impl FromStr for Section {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range: Vec<&str> = s.split("-").collect();
        if range.len() != 2 {
            unreachable!("Syntax ERROR!");
        } else {
            Ok(Self {
                start: range[0].parse::<u64>().unwrap(),
                end: range[1].parse::<u64>().unwrap(),
            })
        }
    }
}

#[derive(Debug)]
struct Pair {
    left: Section,
    right: Section,
}

impl Pair {
    fn is_full_overlaps(&self) -> bool {
        self.right.end <= self.left.end 
    }

    fn is_overlaps(&self) -> bool {
        self.right.start <= self.left.end
    }
}

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs: Vec<&str> = s.split(",").collect();

        if pairs.len() != 2 {
            unreachable!("Syntax ERROR!");
        } else {
            let mut pairs: Vec<Section> = pairs.iter()
                .map(|section| section.parse::<Section>().unwrap())
                .collect();

            pairs.sort_by(|a, b| 
                b.start.cmp(&a.start)
                    .then(a.end.cmp(&b.end))
            );
            
            Ok(Pair {
                left: pairs.pop().unwrap(),
                right: pairs.pop().unwrap(),
            })
        }

    }
}

fn part1(data: &str) -> usize {
    data.lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .filter(|pair| pair.is_full_overlaps())
        .count()
}

fn part2(data: &str) -> usize {
    data.lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .filter(|pair| pair.is_overlaps())
        .count()
}

fn main() -> io::Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day04.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day04.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
