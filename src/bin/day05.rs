use std::{io, num::ParseIntError, str::FromStr};

const YEAR: &'static str = "2022";
const DAY: &'static str = "05";

struct Crates {
    size: usize,
    crates: Vec<Vec<char>>,
}

impl Crates {
    fn move_action(&mut self, from: usize, to: usize) {
        if to > self.size || from > self.size {
            unreachable!("OUT!");
        }

        let value: &char = &self.crates.get_mut(from - 1).unwrap().pop().unwrap();
        let _ = &self.crates.get_mut(to - 1).unwrap().push(*value);
    }

    fn apply_action(&mut self, action: Action) {
        for _ in 0..(action.size) {
            let _ = &self.move_action(action.from, action.to);
        }
    }

    fn apply_action_2(&mut self, action: Action) {
        let values: Vec<char> = {
            let from: &mut Vec<char> = self.crates.get_mut(action.from - 1).unwrap();
            let end: usize = from.len();
            let start: usize = end - action.size;
            from.drain(start..end).collect()
        };

        let to: &mut Vec<char> = self.crates.get_mut(action.to - 1).unwrap();
        values.iter().for_each(|c| to.push(*c))
    }

    fn top_crates(&self) -> String {
        self.crates
            .iter()
            .map(|stack| {
                if stack.len() > 0 {
                    Some(stack.last().unwrap())
                } else {
                    None
                }
            })
            .filter(|top| *top != None)
            .map(|c| c.unwrap())
            .collect()
    }
}

impl FromStr for Crates {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data_lines: Vec<&str> = s.lines().collect();

        if let Some((numbers, lines)) = data_lines.split_last() {
            let size: usize = numbers.trim().split(" ").last().unwrap().parse().unwrap();

            let mut crates: Vec<Vec<char>> = vec![vec![]; size];

            let mut lines: Vec<&str> = Vec::from(lines);
            lines.reverse();
            lines.iter().for_each(|line| {
                for i in 0..size {
                    let value: char = line.chars().nth(4 * i + 1).unwrap();
                    if value != ' ' {
                        crates.get_mut(i).unwrap().push(value);
                    }
                }
            });

            Ok(Crates { size, crates })
        } else {
            unreachable!("Invalid Crates!");
        }
    }
}

struct Action {
    size: usize,
    from: usize,
    to: usize,
}

impl FromStr for Action {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums: Vec<&str> = s
            .split(" ")
            .filter(|w| w.trim().chars().all(|c| c.is_numeric()))
            .collect();

        let to: usize = nums.pop().unwrap().parse().unwrap();
        let from: usize = nums.pop().unwrap().parse().unwrap();
        let size: usize = nums.pop().unwrap().parse().unwrap();

        Ok(Action { size, from, to })
    }
}

fn part1(data: &str) -> String {
    let data_split = data.split("\n\n").collect::<Vec<&str>>();

    let mut crates: Crates = data_split[0].parse().unwrap();

    data_split[1]
        .lines()
        .map(|line| line.parse::<Action>().unwrap())
        .for_each(|action| crates.apply_action(action));

    crates.top_crates()
}

fn part2(data: &str) -> String {
    let data_split = data.split("\n\n").collect::<Vec<&str>>();

    let mut crates: Crates = data_split[0].parse().unwrap();

    data_split[1]
        .lines()
        .map(|line| line.parse::<Action>().unwrap())
        .for_each(|action| crates.apply_action_2(action));

    crates.top_crates()
}

fn main() -> io::Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day05.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day05.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
